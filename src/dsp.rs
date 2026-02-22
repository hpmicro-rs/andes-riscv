//! Andes V5 DSP (P-extension) intrinsics — complete set.
//!
//! All ~190 RV32 DSP instructions in one module, works on **stable Rust** (uses `core::arch::asm!`).
//!
//! ## Usage
//!
//! ```no_run
//! use andes_riscv::dsp;
//!
//! let r = dsp::add16(0x0003_0002, 0x0004_0005); // packed 16-bit add
//! let m = dsp::smmul(a, b);                      // MSW 32×32 multiply
//! let (lo, hi) = dsp::smar64((0, 0), a, b);      // 64-bit MAC
//! ```
//!
//! ## vs `core::arch::riscv32`
//!
//! | | `core::arch::riscv32` | `andes_riscv::dsp` |
//! |---|---|---|
//! | 指令数 | 105 (基础 SIMD) | ~190 (完整 P-ext) |
//! | Rust 版本 | nightly only | **stable** |
//! | Feature gate | `riscv_ext_intrinsics` | 无 |
//!
//! ## Encodings
//!
//! All instructions use opcode `0x7F` (custom-3), differentiated by funct7 + funct3.
//!
//! **Important**: Andes V5 DSP uses opcode `0x7F` (custom-3), NOT `0x77` (OP-V/OP-P)
//! which is used by the RISC-V P-extension draft.
//! Encodings verified against **Andes V5 DSP ISA Extension Specification (UM199 V1.0)**.
//!
//! ## Runtime Detection
//!
//! ```no_run
//! if andes_riscv::register::mmsc_cfg::read().edsp() {
//!     // DSP instructions available
//! }
//! ```
//!
//! ## Performance Notes (`.insn` limitations)
//!
//! This module uses `.insn` (inline assembly) to emit DSP instructions because LLVM/Rust
//! do not natively support Andes V5 DSP. This works correctly but has performance
//! implications compared to native compiler intrinsics:
//!
//! ### 1. Fixed register constraints (64-bit pair instructions)
//!
//! Instructions that produce 64-bit results (e.g. `smar64`, `smalda`, `mulsr64`, `smul16`)
//! require a **register pair (a4/a5)**. Since `.insn` cannot express "any even register pair",
//! the implementation hard-codes a4/a5. This means:
//!
//! - The compiler must **spill/reload** a4/a5 if they are in use for other purposes
//! - In tight loops, the compiler **cannot freely allocate** the accumulator to an optimal
//!   register pair — it must always route through a4/a5
//! - Function calls within a loop body will clobber a4/a5 (they are caller-saved),
//!   forcing additional save/restore
//!
//! ### 2. Opaque semantics to the compiler
//!
//! The compiler treats `.insn` as a black box. It cannot:
//!
//! - **Constant-fold**: `kadd16(0x0001_0002, 0x0003_0004)` won't be optimized to a constant
//! - **Instruction-schedule**: reorder DSP instructions relative to other operations
//! - **Common subexpression elimination**: duplicate DSP calls with same inputs won't be merged
//! - **Auto-vectorize**: the compiler cannot replace scalar loops with DSP SIMD equivalents
//!
//! ### 3. Benchmark results (HPM6750, N=1000, opt-level="z")
//!
//! | Instruction | DSP cycles/iter | Scalar cycles/iter | Speedup | Notes |
//! |-------------|----------------:|-------------------:|--------:|-------|
//! | `kadd16`    |               7 |                 19 |   2.7x  | 2×16-bit saturating add |
//! | `smmul`     |               7 |                  7 |   1.0x  | Scalar `mulh` already equivalent |
//! | `kmda`      |               8 |                 15 |   1.8x  | Dual 16×16 multiply-add |
//! | `smaqa`     |               10 |                 46 |   4.5x  | 4×8-bit multiply-accumulate |
//! | `smar64`    |              17 |                 19 |   1.1x  | Register pair overhead dominates |
//!
//! ### Recommendations
//!
//! - **Single-register DSP** (`kadd16`, `kmda`, `smaqa`, etc.): use freely, the `.insn`
//!   overhead is negligible after inlining
//! - **`smmul`**: no benefit over scalar `mulh` — skip unless you need rounding variant (`smmul_u`)
//! - **64-bit pair instructions** (`smar64`, `smalda`, etc.): beneficial only in tight
//!   inner loops (e.g. FIR filters) where the accumulator stays in a4/a5 across iterations.
//!   Avoid function calls or complex control flow within the loop body
//! - If LLVM gains native Andes P-extension support in the future, these limitations
//!   would be eliminated

use core::arch::asm;

// ---------------------------------------------------------------------------
// Macros
// ---------------------------------------------------------------------------

/// R-type: rd = f(rs1, rs2)
macro_rules! dsp_rr {
    ($(#[$meta:meta])* $name:ident, $f3:literal, $f7:literal) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: usize, b: usize) -> usize {
            let value: usize;
            unsafe {
                asm!(
                    concat!(".insn r 0x7F,", $f3, ", ", $f7, ", {}, {}, {}"),
                    lateout(reg) value, in(reg) a, in(reg) b,
                    options(pure, nomem, nostack),
                );
            }
            value
        }
    };
}

/// R-type accumulate: rd = f(rd, rs1, rs2)
macro_rules! dsp_rrr {
    ($(#[$meta:meta])* $name:ident, $f3:literal, $f7:literal) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(t: usize, a: usize, b: usize) -> usize {
            let mut value: usize;
            unsafe {
                asm!(
                    concat!(".insn r 0x7F,", $f3, ", ", $f7, ", {}, {}, {}"),
                    inlateout(reg) t => value, in(reg) a, in(reg) b,
                    options(pure, nomem, nostack),
                );
            }
            value
        }
    };
}

/// I-type ONEOP: rd = f(rs1) — single operand, sub-function in imm12
macro_rules! dsp_oneop {
    ($(#[$meta:meta])* $name:ident, $imm:literal) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: usize) -> usize {
            let value: usize;
            unsafe {
                asm!(
                    concat!(".insn i 0x7F, 0x0,{}, {}, %lo(", $imm, ")"),
                    lateout(reg) value, in(reg) a,
                    options(pure, nomem, nostack),
                );
            }
            value
        }
    };
}

// ###########################################################################
// Part 1: stdarch-compatible instructions (105)
//
// These mirror `core::arch::riscv32::*` but work on stable Rust.
// ###########################################################################

// ===========================================================================
// 16-bit Parallel Addition & Subtraction
// ===========================================================================

dsp_rr!(/// Packed 16-bit addition, discarding overflow
    add16, "0x0", "0x20");
dsp_rr!(/// Packed 16-bit signed halving addition
    radd16, "0x0", "0x00");
dsp_rr!(/// Packed 16-bit unsigned halving addition
    uradd16, "0x0", "0x10");
dsp_rr!(/// Packed 16-bit signed saturating addition
    kadd16, "0x0", "0x08");
dsp_rr!(/// Packed 16-bit unsigned saturating addition
    ukadd16, "0x0", "0x18");
dsp_rr!(/// Packed 16-bit subtraction, discarding overflow
    sub16, "0x0", "0x21");
dsp_rr!(/// Packed 16-bit signed halving subtraction
    rsub16, "0x0", "0x01");
dsp_rr!(/// Packed 16-bit unsigned halving subtraction
    ursub16, "0x0", "0x11");
dsp_rr!(/// Packed 16-bit signed saturating subtraction
    ksub16, "0x0", "0x09");
dsp_rr!(/// Packed 16-bit unsigned saturating subtraction
    uksub16, "0x0", "0x19");

// ===========================================================================
// 16-bit Cross Addition & Subtraction
// ===========================================================================

dsp_rr!(/// Cross add & subtract: hi=a.hi+b.lo, lo=a.lo-b.hi
    cras16, "0x0", "0x22");
dsp_rr!(/// Halving cross add & subtract (signed)
    rcras16, "0x0", "0x02");
dsp_rr!(/// Halving cross add & subtract (unsigned)
    urcras16, "0x0", "0x12");
dsp_rr!(/// Saturating cross add & subtract (signed)
    kcras16, "0x0", "0x0A");
dsp_rr!(/// Saturating cross add & subtract (unsigned)
    ukcras16, "0x0", "0x1A");
dsp_rr!(/// Cross subtract & add: hi=a.hi-b.lo, lo=a.lo+b.hi
    crsa16, "0x0", "0x23");
dsp_rr!(/// Halving cross subtract & add (signed)
    rcrsa16, "0x0", "0x03");
dsp_rr!(/// Halving cross subtract & add (unsigned)
    urcrsa16, "0x0", "0x13");
dsp_rr!(/// Saturating cross subtract & add (signed)
    kcrsa16, "0x0", "0x0B");
dsp_rr!(/// Saturating cross subtract & add (unsigned)
    ukcrsa16, "0x0", "0x1B");

// ===========================================================================
// 16-bit Straight Addition & Subtraction
// ===========================================================================

dsp_rr!(/// Straight add & subtract: hi=a.hi+b.hi, lo=a.lo-b.lo
    stas16, "0x3", "0x22");
dsp_rr!(/// Halving straight add & subtract (signed)
    rstas16, "0x3", "0x02");
dsp_rr!(/// Halving straight add & subtract (unsigned)
    urstas16, "0x3", "0x12");
dsp_rr!(/// Saturating straight add & subtract (signed)
    kstas16, "0x3", "0x0A");
dsp_rr!(/// Saturating straight add & subtract (unsigned)
    ukstas16, "0x3", "0x1A");
dsp_rr!(/// Straight subtract & add: hi=a.hi-b.hi, lo=a.lo+b.lo
    stsa16, "0x3", "0x23");
dsp_rr!(/// Halving straight subtract & add (signed)
    rstsa16, "0x3", "0x03");
dsp_rr!(/// Halving straight subtract & add (unsigned)
    urstsa16, "0x3", "0x13");
dsp_rr!(/// Saturating straight subtract & add (signed)
    kstsa16, "0x3", "0x0B");
dsp_rr!(/// Saturating straight subtract & add (unsigned)
    ukstsa16, "0x3", "0x1B");

// ===========================================================================
// 8-bit Parallel Addition & Subtraction
// ===========================================================================

dsp_rr!(/// Packed 8-bit addition, discarding overflow
    add8, "0x0", "0x24");
dsp_rr!(/// Packed 8-bit signed halving addition
    radd8, "0x0", "0x04");
dsp_rr!(/// Packed 8-bit unsigned halving addition
    uradd8, "0x0", "0x14");
dsp_rr!(/// Packed 8-bit signed saturating addition
    kadd8, "0x0", "0x0C");
dsp_rr!(/// Packed 8-bit unsigned saturating addition
    ukadd8, "0x0", "0x1C");
dsp_rr!(/// Packed 8-bit subtraction, discarding overflow
    sub8, "0x0", "0x25");
dsp_rr!(/// Packed 8-bit signed halving subtraction
    rsub8, "0x0", "0x05");
dsp_rr!(/// Packed 8-bit unsigned halving subtraction
    ursub8, "0x0", "0x15");
dsp_rr!(/// Packed 8-bit signed saturating subtraction
    ksub8, "0x0", "0x0D");
dsp_rr!(/// Packed 8-bit unsigned saturating subtraction
    uksub8, "0x0", "0x1D");

// ===========================================================================
// 16-bit Shift
// ===========================================================================

dsp_rr!(/// Packed 16-bit arithmetic right shift
    sra16, "0x0", "0x28");
dsp_rr!(/// Packed 16-bit rounding arithmetic right shift
    sra16u, "0x0", "0x30");
dsp_rr!(/// Packed 16-bit logical right shift
    srl16, "0x0", "0x29");
dsp_rr!(/// Packed 16-bit rounding logical right shift
    srl16u, "0x0", "0x31");
dsp_rr!(/// Packed 16-bit logical left shift
    sll16, "0x0", "0x2A");
dsp_rr!(/// Packed 16-bit saturating logical left shift
    ksll16, "0x0", "0x32");
dsp_rr!(/// Packed 16-bit saturating left / arithmetic right shift
    kslra16, "0x0", "0x2B");
dsp_rr!(/// Packed 16-bit saturating left / rounding arithmetic right shift
    kslra16u, "0x0", "0x33");

// ===========================================================================
// 8-bit Shift
// ===========================================================================

dsp_rr!(/// Packed 8-bit arithmetic right shift
    sra8, "0x0", "0x2C");
dsp_rr!(/// Packed 8-bit rounding arithmetic right shift
    sra8u, "0x0", "0x34");
dsp_rr!(/// Packed 8-bit logical right shift
    srl8, "0x0", "0x2D");
dsp_rr!(/// Packed 8-bit rounding logical right shift
    srl8u, "0x0", "0x35");
dsp_rr!(/// Packed 8-bit logical left shift
    sll8, "0x0", "0x2E");
dsp_rr!(/// Packed 8-bit saturating logical left shift
    ksll8, "0x0", "0x36");
dsp_rr!(/// Packed 8-bit saturating left / arithmetic right shift
    kslra8, "0x0", "0x2F");
dsp_rr!(/// Packed 8-bit saturating left / rounding arithmetic right shift
    kslra8u, "0x0", "0x37");

// ===========================================================================
// 16-bit Compare
// ===========================================================================

dsp_rr!(/// Packed 16-bit equal compare (result: 0xFFFF or 0x0000)
    cmpeq16, "0x0", "0x26");
dsp_rr!(/// Packed 16-bit signed less-than compare
    scmplt16, "0x0", "0x06");
dsp_rr!(/// Packed 16-bit signed less-equal compare
    scmple16, "0x0", "0x0E");
dsp_rr!(/// Packed 16-bit unsigned less-than compare
    ucmplt16, "0x0", "0x16");
dsp_rr!(/// Packed 16-bit unsigned less-equal compare
    ucmple16, "0x0", "0x1E");

// ===========================================================================
// 8-bit Compare
// ===========================================================================

dsp_rr!(/// Packed 8-bit equal compare (result: 0xFF or 0x00)
    cmpeq8, "0x0", "0x27");
dsp_rr!(/// Packed 8-bit signed less-than compare
    scmplt8, "0x0", "0x07");
dsp_rr!(/// Packed 8-bit signed less-equal compare
    scmple8, "0x0", "0x0F");
dsp_rr!(/// Packed 8-bit unsigned less-than compare
    ucmplt8, "0x0", "0x17");
dsp_rr!(/// Packed 8-bit unsigned less-equal compare
    ucmple8, "0x0", "0x1F");

// ===========================================================================
// Min / Max
// ===========================================================================

dsp_rr!(/// Packed 16-bit signed minimum
    smin16, "0x0", "0x40");
dsp_rr!(/// Packed 16-bit unsigned minimum
    umin16, "0x0", "0x48");
dsp_rr!(/// Packed 16-bit signed maximum
    smax16, "0x0", "0x41");
dsp_rr!(/// Packed 16-bit unsigned maximum
    umax16, "0x0", "0x49");
dsp_rr!(/// Packed 8-bit signed minimum
    smin8, "0x0", "0x44");
dsp_rr!(/// Packed 8-bit unsigned minimum
    umin8, "0x0", "0x4C");
dsp_rr!(/// Packed 8-bit signed maximum
    smax8, "0x0", "0x45");
dsp_rr!(/// Packed 8-bit unsigned maximum
    umax8, "0x0", "0x4D");

// ===========================================================================
// Absolute Value / Leading Bit Count (ONEOP — single operand)
// ===========================================================================

dsp_oneop!(/// Packed 16-bit saturating absolute value
    kabs16, "0xAD1");
dsp_oneop!(/// Packed 8-bit saturating absolute value
    kabs8, "0xAD0");
dsp_oneop!(/// Packed 16-bit count leading redundant sign bits
    clrs16, "0xAE8");
dsp_oneop!(/// Packed 8-bit count leading redundant sign bits
    clrs8, "0xAE0");
dsp_oneop!(/// 32-bit count leading redundant sign bits
    clrs32, "0xAF8");
dsp_oneop!(/// Packed 16-bit count leading zeros
    clz16, "0xAE9");
dsp_oneop!(/// Packed 8-bit count leading zeros
    clz8, "0xAE1");
dsp_oneop!(/// 32-bit count leading zeros
    clz32, "0xAF9");

// ===========================================================================
// Swap / Pack
// ===========================================================================

/// Swap 16-bit halfwords: `rd = {a[15:0], a[31:16]}`
///
/// Alias for `pkbt rd, rs1, rs1`.
#[inline(always)]
pub fn swap16(a: usize) -> usize {
    let value: usize;
    unsafe {
        asm!(".insn r 0x7F,0x0, 0x0F, {}, {}, {}",
            lateout(reg) value, in(reg) a, in(reg) a,
            options(pure, nomem, nostack));
    }
    value
}

dsp_oneop!(/// Swap bytes within each 16-bit halfword
    swap8, "0xAD8");

dsp_rr!(/// Pack bottom of a + top of b into 16-bit halves
    pkbt16, "0x1", "0x0F");
dsp_rr!(/// Pack top of a + bottom of b into 16-bit halves
    pktb16, "0x1", "0x1F");

// ===========================================================================
// Unpack (signed / unsigned byte → halfword)
// ===========================================================================

dsp_oneop!(/// Signed unpack byte[1] & byte[0] → i16×2
    sunpkd810, "0xAC8");
dsp_oneop!(/// Signed unpack byte[2] & byte[0] → i16×2
    sunpkd820, "0xAC9");
dsp_oneop!(/// Signed unpack byte[3] & byte[0] → i16×2
    sunpkd830, "0xACA");
dsp_oneop!(/// Signed unpack byte[3] & byte[1] → i16×2
    sunpkd831, "0xACB");
dsp_oneop!(/// Signed unpack byte[3] & byte[2] → i16×2
    sunpkd832, "0xAD3");
dsp_oneop!(/// Unsigned unpack byte[1] & byte[0] → u16×2
    zunpkd810, "0xACC");
dsp_oneop!(/// Unsigned unpack byte[2] & byte[0] → u16×2
    zunpkd820, "0xACD");
dsp_oneop!(/// Unsigned unpack byte[3] & byte[0] → u16×2
    zunpkd830, "0xACE");
dsp_oneop!(/// Unsigned unpack byte[3] & byte[1] → u16×2
    zunpkd831, "0xACF");
dsp_oneop!(/// Unsigned unpack byte[3] & byte[2] → u16×2
    zunpkd832, "0xAD7");

// ===========================================================================
// 8-bit Multiply-Accumulate (SMAQA)
// ===========================================================================

dsp_rrr!(/// Signed 8-bit multiply, 16-bit accumulate: `rd += Σ(rs1.b[i] × rs2.b[i])`
    smaqa, "0x0", "0x64");
dsp_rrr!(/// Unsigned 8-bit multiply, 16-bit accumulate
    umaqa, "0x0", "0x66");
dsp_rrr!(/// Signed × unsigned 8-bit multiply-accumulate
    smaqasu, "0x0", "0x65");

// ===========================================================================
// Half-word Arithmetic (Q15 / U16 saturation)
// ===========================================================================

dsp_rr!(/// Signed lower 16-bit addition with Q15 saturation
    kaddh, "0x1", "0x02");
dsp_rr!(/// Signed lower 16-bit subtraction with Q15 saturation
    ksubh, "0x1", "0x03");
dsp_rr!(/// Unsigned lower 16-bit addition with U16 saturation
    ukaddh, "0x1", "0x0A");
dsp_rr!(/// Unsigned lower 16-bit subtraction with U16 saturation
    uksubh, "0x1", "0x0B");

// ===========================================================================
// Byte Absolute Difference
// ===========================================================================

dsp_rr!(/// Sum of absolute byte differences: `Σ|a.byte[i] - b.byte[i]|`
    pbsad, "0x0", "0x7E");
dsp_rrr!(/// Accumulating sum of absolute byte differences: `rd += Σ|a.byte[i] - b.byte[i]|`
    pbsada, "0x0", "0x7F");

// ###########################################################################
// Part 2: Extended instructions (multiply, word arithmetic, 64-bit, etc.)
//
// Not in stdarch. Only available through this module.
// ###########################################################################

// ===========================================================================
// 1. MSW 32×32 Multiply (funct3=0x1)
//    rd = (rs1 × rs2) >> 32
// ===========================================================================

dsp_rr!(
    /// Signed MSW 32×32 multiply: `rd = (rs1 × rs2) >> 32`
    smmul, "0x1", "0x20"
);

dsp_rr!(
    /// Signed MSW 32×32 multiply with rounding: `rd = (rs1 × rs2 + 0x80000000) >> 32`
    smmul_u, "0x1", "0x28"
);

dsp_rr!(
    /// Saturating signed MSW multiply & double: `rd = SAT.Q31((rs1 × rs2) >> 31)`
    kwmmul, "0x1", "0x31"
);

dsp_rr!(
    /// Saturating signed MSW multiply & double with rounding
    kwmmul_u, "0x1", "0x39"
);

dsp_rrr!(
    /// Saturating MSW multiply-accumulate: `rd = SAT.Q31(rd + (rs1 × rs2) >> 32)`
    kmmac, "0x1", "0x30"
);

dsp_rrr!(
    /// Saturating MSW multiply-accumulate with rounding
    kmmac_u, "0x1", "0x38"
);

dsp_rrr!(
    /// Saturating MSW multiply-subtract: `rd = SAT.Q31(rd - (rs1 × rs2) >> 32)`
    kmmsb, "0x1", "0x21"
);

dsp_rrr!(
    /// Saturating MSW multiply-subtract with rounding
    kmmsb_u, "0x1", "0x29"
);

// ===========================================================================
// 2. MSW 32×16 Multiply (funct3=0x1)
//    rd = (rs1 × rs2.half) >> 16
// ===========================================================================

dsp_rr!(
    /// Signed MSW multiply 32×16(bottom): `rd = (rs1 × rs2.B16) >> 16`
    smmwb, "0x1", "0x22"
);

dsp_rr!(
    /// Signed MSW multiply 32×16(bottom) with rounding
    smmwb_u, "0x1", "0x2A"
);

dsp_rr!(
    /// Signed MSW multiply 32×16(top): `rd = (rs1 × rs2.T16) >> 16`
    smmwt, "0x1", "0x32"
);

dsp_rr!(
    /// Signed MSW multiply 32×16(top) with rounding
    smmwt_u, "0x1", "0x3A"
);

dsp_rr!(
    /// Saturating signed MSW multiply 32×16(bottom)×2
    kmmwb2, "0x1", "0x47"
);

dsp_rr!(
    /// Saturating signed MSW multiply 32×16(bottom)×2 with rounding
    kmmwb2_u, "0x1", "0x4F"
);

dsp_rr!(
    /// Saturating signed MSW multiply 32×16(top)×2
    kmmwt2, "0x1", "0x57"
);

dsp_rr!(
    /// Saturating signed MSW multiply 32×16(top)×2 with rounding
    kmmwt2_u, "0x1", "0x5F"
);

dsp_rrr!(
    /// Saturating signed MSW 32×16(bottom) multiply-accumulate
    kmmawb, "0x1", "0x23"
);

dsp_rrr!(
    /// Saturating signed MSW 32×16(bottom) multiply-accumulate with rounding
    kmmawb_u, "0x1", "0x2B"
);

dsp_rrr!(
    /// Saturating signed MSW 32×16(top) multiply-accumulate
    kmmawt, "0x1", "0x33"
);

dsp_rrr!(
    /// Saturating signed MSW 32×16(top) multiply-accumulate with rounding
    kmmawt_u, "0x1", "0x3B"
);

dsp_rrr!(
    /// Saturating signed MSW 32×16(bottom)×2 multiply-accumulate
    kmmawb2, "0x1", "0x67"
);

dsp_rrr!(
    /// Saturating signed MSW 32×16(bottom)×2 multiply-accumulate with rounding
    kmmawb2_u, "0x1", "0x6F"
);

dsp_rrr!(
    /// Saturating signed MSW 32×16(top)×2 multiply-accumulate
    kmmawt2, "0x1", "0x77"
);

dsp_rrr!(
    /// Saturating signed MSW 32×16(top)×2 multiply-accumulate with rounding
    kmmawt2_u, "0x1", "0x7F"
);

// ===========================================================================
// 3. Signed 16×16 Multiply with 32-bit Result (funct3=0x1)
//    bb = bottom×bottom, bt = bottom×top, tt = top×top
// ===========================================================================

dsp_rr!(
    /// Signed 16×16 multiply (bottom×bottom): `rd = rs1.B16 × rs2.B16`
    smbb16, "0x1", "0x04"
);

dsp_rr!(
    /// Signed 16×16 multiply (bottom×top): `rd = rs1.B16 × rs2.T16`
    smbt16, "0x1", "0x0C"
);

dsp_rr!(
    /// Signed 16×16 multiply (top×top): `rd = rs1.T16 × rs2.T16`
    smtt16, "0x1", "0x14"
);

dsp_rr!(
    /// Signed dual 16-bit multiply and add: `rd = rs1.B16×rs2.B16 + rs1.T16×rs2.T16`
    kmda, "0x1", "0x1C"
);

dsp_rr!(
    /// Signed crossed dual 16-bit multiply and add: `rd = rs1.B16×rs2.T16 + rs1.T16×rs2.B16`
    kmxda, "0x1", "0x1D"
);

dsp_rr!(
    /// Signed dual 16-bit multiply and subtract: `rd = rs1.T16×rs2.T16 - rs1.B16×rs2.B16`
    smds, "0x1", "0x2C"
);

dsp_rr!(
    /// Signed dual 16-bit multiply and reverse subtract: `rd = rs1.B16×rs2.B16 - rs1.T16×rs2.T16`
    smdrs, "0x1", "0x34"
);

dsp_rr!(
    /// Signed crossed dual 16-bit multiply and subtract: `rd = rs1.T16×rs2.B16 - rs1.B16×rs2.T16`
    smxds, "0x1", "0x3C"
);

// --- 16×16 Multiply-Accumulate ---

dsp_rrr!(
    /// Saturating signed 16×16(bb) multiply-accumulate: `rd = SAT(rd + rs1.B16×rs2.B16)`
    kmabb, "0x1", "0x2D"
);

dsp_rrr!(
    /// Saturating signed 16×16(bt) multiply-accumulate: `rd = SAT(rd + rs1.B16×rs2.T16)`
    kmabt, "0x1", "0x35"
);

dsp_rrr!(
    /// Saturating signed 16×16(tt) multiply-accumulate: `rd = SAT(rd + rs1.T16×rs2.T16)`
    kmatt, "0x1", "0x3D"
);

dsp_rrr!(
    /// Saturating signed dual multiply-accumulate: `rd = SAT(rd + rs1.B16×rs2.B16 + rs1.T16×rs2.T16)`
    kmada, "0x1", "0x24"
);

dsp_rrr!(
    /// Saturating signed crossed dual multiply-accumulate
    kmaxda, "0x1", "0x25"
);

dsp_rrr!(
    /// Saturating signed dual multiply-subtract accumulate: `rd = SAT(rd + rs1.T16×rs2.T16 - rs1.B16×rs2.B16)`
    kmads, "0x1", "0x2E"
);

dsp_rrr!(
    /// Saturating signed dual multiply reverse-subtract accumulate
    kmadrs, "0x1", "0x36"
);

dsp_rrr!(
    /// Saturating signed crossed dual multiply-subtract accumulate
    kmaxds, "0x1", "0x3E"
);

dsp_rrr!(
    /// Saturating signed dual multiply subtract-accumulate: `rd = SAT(rd - rs1.B16×rs2.B16 - rs1.T16×rs2.T16)`
    kmsda, "0x1", "0x26"
);

dsp_rrr!(
    /// Saturating signed crossed dual multiply subtract-accumulate
    kmsxda, "0x1", "0x27"
);

// ===========================================================================
// 4. KDM/KHM — Double/Half Saturating Multiply (funct3=0x1)
// ===========================================================================

dsp_rr!(
    /// Signed saturating double multiply (bb): `rd = SAT.Q31(2 × rs1.B16 × rs2.B16)`
    kdmbb, "0x1", "0x05"
);

dsp_rr!(
    /// Signed saturating double multiply (bt): `rd = SAT.Q31(2 × rs1.B16 × rs2.T16)`
    kdmbt, "0x1", "0x0D"
);

dsp_rr!(
    /// Signed saturating double multiply (tt): `rd = SAT.Q31(2 × rs1.T16 × rs2.T16)`
    kdmtt, "0x1", "0x15"
);

dsp_rrr!(
    /// Saturating double multiply-accumulate (bb): `rd = SAT.Q31(rd + 2 × rs1.B16 × rs2.B16)`
    kdmabb, "0x1", "0x69"
);

dsp_rrr!(
    /// Saturating double multiply-accumulate (bt): `rd = SAT.Q31(rd + 2 × rs1.B16 × rs2.T16)`
    kdmabt, "0x1", "0x71"
);

dsp_rrr!(
    /// Saturating double multiply-accumulate (tt): `rd = SAT.Q31(rd + 2 × rs1.T16 × rs2.T16)`
    kdmatt, "0x1", "0x79"
);

dsp_rr!(
    /// Signed saturating Q15 half multiply (bb): `rd = SAT.Q15((rs1.B16 × rs2.B16) >> 15)`
    khmbb, "0x1", "0x06"
);

dsp_rr!(
    /// Signed saturating Q15 half multiply (bt): `rd = SAT.Q15((rs1.B16 × rs2.T16) >> 15)`
    khmbt, "0x1", "0x0E"
);

dsp_rr!(
    /// Signed saturating Q15 half multiply (tt): `rd = SAT.Q15((rs1.T16 × rs2.T16) >> 15)`
    khmtt, "0x1", "0x16"
);

// --- Packed KHM (SIMD saturating Q15/Q7 multiply) ---

dsp_rr!(
    /// SIMD packed Q15 saturating multiply: each 16-bit `rd[i] = SAT.Q15((rs1[i] × rs2[i]) >> 15)`
    khm16, "0x0", "0x43"
);

dsp_rr!(
    /// SIMD crossed packed Q15 saturating multiply
    khmx16, "0x0", "0x4B"
);

dsp_rr!(
    /// SIMD packed Q7 saturating multiply: each 8-bit `rd[i] = SAT.Q7((rs1[i] × rs2[i]) >> 7)`
    khm8, "0x0", "0x47"
);

dsp_rr!(
    /// SIMD crossed packed Q7 saturating multiply
    khmx8, "0x0", "0x4F"
);

// ===========================================================================
// 5. 32-bit Word Arithmetic (funct3=0x1)
// ===========================================================================

dsp_rr!(
    /// Signed saturating 32-bit addition (Q31): `rd = SAT.Q31(rs1 + rs2)`
    kaddw, "0x1", "0x00"
);

dsp_rr!(
    /// Signed saturating 32-bit subtraction (Q31): `rd = SAT.Q31(rs1 - rs2)`
    ksubw, "0x1", "0x01"
);

dsp_rr!(
    /// Unsigned saturating 32-bit addition: `rd = SAT.U32(rs1 + rs2)`
    ukaddw, "0x1", "0x08"
);

dsp_rr!(
    /// Unsigned saturating 32-bit subtraction: `rd = SAT.U32(rs1 - rs2)`
    uksubw, "0x1", "0x09"
);

dsp_rr!(
    /// Signed halving 32-bit addition: `rd = (rs1 + rs2) >> 1`
    raddw, "0x1", "0x10"
);

dsp_rr!(
    /// Signed halving 32-bit subtraction: `rd = (rs1 - rs2) >> 1`
    rsubw, "0x1", "0x11"
);

dsp_rr!(
    /// Unsigned halving 32-bit addition: `rd = (rs1 + rs2) >> 1` (unsigned)
    uraddw, "0x1", "0x18"
);

dsp_rr!(
    /// Unsigned halving 32-bit subtraction: `rd = (rs1 - rs2) >> 1` (unsigned)
    ursubw, "0x1", "0x19"
);

// ===========================================================================
// 6. Non-SIMD Misc (funct3=0x0 or 0x1)
// ===========================================================================

dsp_rr!(
    /// Signed 32-bit maximum: `rd = max(rs1, rs2)`
    maxw, "0x0", "0x79"
);

dsp_rr!(
    /// Signed 32-bit minimum: `rd = min(rs1, rs2)`
    minw, "0x0", "0x78"
);

dsp_rr!(
    /// Average with rounding: `rd = (rs1 + rs2 + 1) >> 1`
    ave, "0x0", "0x70"
);

dsp_rr!(
    /// Rounding shift right arithmetic: `rd = (rs1 >> rs2[4:0]) + round_bit`
    sra_u, "0x1", "0x12"
);

dsp_rr!(
    /// Saturating shift left logical for word: `rd = SAT.Q31(rs1 << rs2[4:0])`
    ksllw, "0x1", "0x13"
);

dsp_rr!(
    /// Saturating shift left or arithmetic right for word
    kslraw, "0x1", "0x37"
);

dsp_rr!(
    /// Saturating shift left or rounding arithmetic right for word
    kslraw_u, "0x1", "0x3F"
);

dsp_rr!(
    /// Bit reverse: reverse bits of rs1 in range [0, rs2[4:0]]
    bitrev, "0x0", "0x73"
);

// --- Non-SIMD Multiply-Accumulate (32-bit result) ---

dsp_rrr!(
    /// Multiply and add to 32-bit: `rd = rd + rs1 × rs2` (low 32 bits)
    maddr32, "0x1", "0x62"
);

dsp_rrr!(
    /// Multiply and subtract from 32-bit: `rd = rd - rs1 × rs2` (low 32 bits)
    msubr32, "0x1", "0x63"
);

// ===========================================================================
// 7. Additional Packing (not in stdarch)
// ===========================================================================

dsp_rr!(
    /// Pack bottom-bottom 16-bit: `rd = {rs1.B16, rs2.B16}`
    pkbb16, "0x1", "0x07"
);

dsp_rr!(
    /// Pack top-top 16-bit: `rd = {rs1.T16, rs2.T16}`
    pktt16, "0x1", "0x17"
);

// ===========================================================================
// 8. Saturating Absolute Value (ONEOP format)
// ===========================================================================

/// Saturating 32-bit absolute value: `rd = (rs1 < 0) ? SAT(-rs1) : rs1`
///
/// Uses ONEOP encoding: funct7=0x56, rs2=0x14 (fixed).
#[inline(always)]
pub fn kabsw(a: usize) -> usize {
    let value: usize;
    unsafe {
        // ONEOP: .insn r 0x7F,0x0, 0x56, rd, rs1, x20
        // x20 (s4) is just encoding, not actually read by hardware
        asm!(
            ".insn r 0x7F,0x0, 0x56, {rd}, {rs1}, x20",
            rd = lateout(reg) value,
            rs1 = in(reg) a,
            out("x20") _,
            options(pure, nomem, nostack),
        );
    }
    value
}

/// Count leading ones, packed 8-bit
///
/// Uses ONEOP encoding: funct7=0x57, rs2=0x03.
#[inline(always)]
pub fn clo8(a: usize) -> usize {
    let value: usize;
    unsafe {
        asm!(
            ".insn r 0x7F,0x0, 0x57, {rd}, {rs1}, x3",
            rd = lateout(reg) value,
            rs1 = in(reg) a,
            options(pure, nomem, nostack),
        );
    }
    value
}

/// Count leading ones, packed 16-bit
#[inline(always)]
pub fn clo16(a: usize) -> usize {
    let value: usize;
    unsafe {
        asm!(
            ".insn r 0x7F,0x0, 0x57, {rd}, {rs1}, x11",
            rd = lateout(reg) value,
            rs1 = in(reg) a,
            options(pure, nomem, nostack),
        );
    }
    value
}

/// Count leading ones, 32-bit
#[inline(always)]
pub fn clo32(a: usize) -> usize {
    let value: usize;
    unsafe {
        asm!(
            ".insn r 0x7F,0x0, 0x57, {rd}, {rs1}, x27",
            rd = lateout(reg) value,
            rs1 = in(reg) a,
            options(pure, nomem, nostack),
        );
    }
    value
}

// ===========================================================================
// 9. Packed 16×16 / 8×8 Multiply (64-bit result via register pair)
//
// On RV32 these write to (Rd, Rd+1) register pair. Since Rust inline asm
// cannot constrain even-register allocation, we provide these with explicit
// register usage. The caller should use `black_box` or similar to prevent
// the compiler from reordering.
// ===========================================================================

/// Signed packed 16×16 multiply: two 16-bit multiplies → 64-bit result.
///
/// Returns `(lo, hi)` where `lo = rs1.B16 × rs2.B16` and `hi = rs1.T16 × rs2.T16`.
///
/// **Note**: Uses fixed registers (a4/a5) for the register pair output.
#[inline(always)]
pub fn smul16(a: usize, b: usize) -> (usize, usize) {
    let lo: usize;
    let hi: usize;
    unsafe {
        asm!(
            ".insn r 0x7F,0x0, 0x50, a4, {rs1}, {rs2}",
            rs1 = in(reg) a,
            rs2 = in(reg) b,
            out("a4") lo,
            out("a5") hi,
            options(pure, nomem, nostack),
        );
    }
    (lo, hi)
}

/// Signed crossed packed 16×16 multiply: `lo = rs1.B16 × rs2.T16`, `hi = rs1.T16 × rs2.B16`
#[inline(always)]
pub fn smulx16(a: usize, b: usize) -> (usize, usize) {
    let lo: usize;
    let hi: usize;
    unsafe {
        asm!(
            ".insn r 0x7F,0x0, 0x51, a4, {rs1}, {rs2}",
            rs1 = in(reg) a,
            rs2 = in(reg) b,
            out("a4") lo,
            out("a5") hi,
            options(pure, nomem, nostack),
        );
    }
    (lo, hi)
}

/// Unsigned packed 16×16 multiply
#[inline(always)]
pub fn umul16(a: usize, b: usize) -> (usize, usize) {
    let lo: usize;
    let hi: usize;
    unsafe {
        asm!(
            ".insn r 0x7F,0x0, 0x58, a4, {rs1}, {rs2}",
            rs1 = in(reg) a,
            rs2 = in(reg) b,
            out("a4") lo,
            out("a5") hi,
            options(pure, nomem, nostack),
        );
    }
    (lo, hi)
}

/// Unsigned crossed packed 16×16 multiply
#[inline(always)]
pub fn umulx16(a: usize, b: usize) -> (usize, usize) {
    let lo: usize;
    let hi: usize;
    unsafe {
        asm!(
            ".insn r 0x7F,0x0, 0x59, a4, {rs1}, {rs2}",
            rs1 = in(reg) a,
            rs2 = in(reg) b,
            out("a4") lo,
            out("a5") hi,
            options(pure, nomem, nostack),
        );
    }
    (lo, hi)
}

/// Signed packed 8×8 multiply: four 8-bit multiplies → 64-bit result (4 × i16).
#[inline(always)]
pub fn smul8(a: usize, b: usize) -> (usize, usize) {
    let lo: usize;
    let hi: usize;
    unsafe {
        asm!(
            ".insn r 0x7F,0x0, 0x54, a4, {rs1}, {rs2}",
            rs1 = in(reg) a,
            rs2 = in(reg) b,
            out("a4") lo,
            out("a5") hi,
            options(pure, nomem, nostack),
        );
    }
    (lo, hi)
}

/// Signed crossed packed 8×8 multiply
#[inline(always)]
pub fn smulx8(a: usize, b: usize) -> (usize, usize) {
    let lo: usize;
    let hi: usize;
    unsafe {
        asm!(
            ".insn r 0x7F,0x0, 0x55, a4, {rs1}, {rs2}",
            rs1 = in(reg) a,
            rs2 = in(reg) b,
            out("a4") lo,
            out("a5") hi,
            options(pure, nomem, nostack),
        );
    }
    (lo, hi)
}

/// Unsigned packed 8×8 multiply
#[inline(always)]
pub fn umul8(a: usize, b: usize) -> (usize, usize) {
    let lo: usize;
    let hi: usize;
    unsafe {
        asm!(
            ".insn r 0x7F,0x0, 0x5C, a4, {rs1}, {rs2}",
            rs1 = in(reg) a,
            rs2 = in(reg) b,
            out("a4") lo,
            out("a5") hi,
            options(pure, nomem, nostack),
        );
    }
    (lo, hi)
}

/// Unsigned crossed packed 8×8 multiply
#[inline(always)]
pub fn umulx8(a: usize, b: usize) -> (usize, usize) {
    let lo: usize;
    let hi: usize;
    unsafe {
        asm!(
            ".insn r 0x7F,0x0, 0x5D, a4, {rs1}, {rs2}",
            rs1 = in(reg) a,
            rs2 = in(reg) b,
            out("a4") lo,
            out("a5") hi,
            options(pure, nomem, nostack),
        );
    }
    (lo, hi)
}

// ===========================================================================
// 10. 64-bit Arithmetic (register pair on RV32)
//
// These use (Rd, Rd+1) and (Rs1, Rs1+1) register pairs for 64-bit values.
// We use fixed registers: a0/a1 for input A, a2/a3 for input B, a4/a5 for output.
// ===========================================================================

/// Macro for 64-bit pair operations: rd_pair = f(rs1_pair, rs2_pair)
macro_rules! dsp_pair {
    ($(#[$meta:meta])* $name:ident, $f3:literal, $f7:literal) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
            let lo: usize;
            let hi: usize;
            unsafe {
                asm!(
                    concat!(".insn r 0x7F,", $f3, ", ", $f7, ", a4, a0, a2"),
                    in("a0") a.0, in("a1") a.1,
                    in("a2") b.0, in("a3") b.1,
                    lateout("a4") lo, lateout("a5") hi,
                    options(pure, nomem, nostack),
                );
            }
            (lo, hi)
        }
    };
}

/// Macro for 64-bit pair accumulate: rd_pair = f(rd_pair, rs1, rs2)
macro_rules! dsp_pair_acc {
    ($(#[$meta:meta])* $name:ident, $f3:literal, $f7:literal) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(t: (usize, usize), a: usize, b: usize) -> (usize, usize) {
            let lo: usize;
            let hi: usize;
            unsafe {
                asm!(
                    concat!(".insn r 0x7F,", $f3, ", ", $f7, ", a4, {rs1}, {rs2}"),
                    rs1 = in(reg) a,
                    rs2 = in(reg) b,
                    inlateout("a4") t.0 => lo,
                    inlateout("a5") t.1 => hi,
                    options(pure, nomem, nostack),
                );
            }
            (lo, hi)
        }
    };
}

// --- 64-bit Addition & Subtraction ---

dsp_pair!(
    /// 64-bit addition
    add64, "0x1", "0x60"
);

dsp_pair!(
    /// 64-bit subtraction
    sub64, "0x1", "0x61"
);

dsp_pair!(
    /// 64-bit signed halving addition
    radd64, "0x1", "0x40"
);

dsp_pair!(
    /// 64-bit unsigned halving addition
    uradd64, "0x1", "0x50"
);

dsp_pair!(
    /// 64-bit signed saturating addition
    kadd64, "0x1", "0x48"
);

dsp_pair!(
    /// 64-bit unsigned saturating addition
    ukadd64, "0x1", "0x58"
);

dsp_pair!(
    /// 64-bit signed halving subtraction
    rsub64, "0x1", "0x41"
);

dsp_pair!(
    /// 64-bit unsigned halving subtraction
    ursub64, "0x1", "0x51"
);

dsp_pair!(
    /// 64-bit signed saturating subtraction
    ksub64, "0x1", "0x49"
);

dsp_pair!(
    /// 64-bit unsigned saturating subtraction
    uksub64, "0x1", "0x59"
);

// --- 32×32 Multiply to 64-bit ---

dsp_pair_acc!(
    /// Signed 32×32 multiply-accumulate to 64-bit: `rd_pair += rs1 × rs2`
    smar64, "0x1", "0x42"
);

dsp_pair_acc!(
    /// Signed 32×32 multiply-subtract from 64-bit: `rd_pair -= rs1 × rs2`
    smsr64, "0x1", "0x43"
);

dsp_pair_acc!(
    /// Unsigned 32×32 multiply-accumulate to 64-bit
    umar64, "0x1", "0x52"
);

dsp_pair_acc!(
    /// Unsigned 32×32 multiply-subtract from 64-bit
    umsr64, "0x1", "0x53"
);

dsp_pair_acc!(
    /// Signed saturating 32×32 multiply-accumulate to 64-bit
    kmar64, "0x1", "0x4A"
);

dsp_pair_acc!(
    /// Signed saturating 32×32 multiply-subtract from 64-bit
    kmsr64, "0x1", "0x4B"
);

dsp_pair_acc!(
    /// Unsigned saturating 32×32 multiply-accumulate to 64-bit
    ukmar64, "0x1", "0x5A"
);

dsp_pair_acc!(
    /// Unsigned saturating 32×32 multiply-subtract from 64-bit
    ukmsr64, "0x1", "0x5B"
);

// --- 16-bit Multiply to 64-bit ---

/// Macro for SMAL-family: rd_pair = f(rd_pair, rs1_pair_or_reg)
/// SMAL uses rd pair as accumulator and rs1 as packed 16-bit source
macro_rules! dsp_pair_acc_16 {
    ($(#[$meta:meta])* $name:ident, $f3:literal, $f7:literal) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(t: (usize, usize), a: usize, b: usize) -> (usize, usize) {
            let lo: usize;
            let hi: usize;
            unsafe {
                asm!(
                    concat!(".insn r 0x7F,", $f3, ", ", $f7, ", a4, {rs1}, {rs2}"),
                    rs1 = in(reg) a,
                    rs2 = in(reg) b,
                    inlateout("a4") t.0 => lo,
                    inlateout("a5") t.1 => hi,
                    options(pure, nomem, nostack),
                );
            }
            (lo, hi)
        }
    };
}

dsp_pair_acc_16!(
    /// Signed 16-bit multiply and add to 64-bit
    smal, "0x1", "0x2F"
);

dsp_pair_acc_16!(
    /// Signed 16×16(bb) multiply-accumulate to 64-bit
    smalbb, "0x1", "0x44"
);

dsp_pair_acc_16!(
    /// Signed 16×16(bt) multiply-accumulate to 64-bit
    smalbt, "0x1", "0x4C"
);

dsp_pair_acc_16!(
    /// Signed 16×16(tt) multiply-accumulate to 64-bit
    smaltt, "0x1", "0x54"
);

dsp_pair_acc_16!(
    /// Signed dual 16-bit multiply and add to 64-bit
    smalda, "0x1", "0x46"
);

dsp_pair_acc_16!(
    /// Signed crossed dual 16-bit multiply and add to 64-bit
    smalxda, "0x1", "0x4E"
);

dsp_pair_acc_16!(
    /// Signed dual 16-bit multiply-subtract and add to 64-bit
    smalds, "0x1", "0x45"
);

dsp_pair_acc_16!(
    /// Signed dual 16-bit multiply reverse-subtract and add to 64-bit
    smaldrs, "0x1", "0x4D"
);

dsp_pair_acc_16!(
    /// Signed crossed dual 16-bit multiply-subtract and add to 64-bit
    smalxds, "0x1", "0x55"
);

dsp_pair_acc_16!(
    /// Signed dual 16-bit multiply and subtract from 64-bit
    smslda, "0x1", "0x56"
);

dsp_pair_acc_16!(
    /// Signed crossed dual 16-bit multiply and subtract from 64-bit
    smslxda, "0x1", "0x5E"
);

// ===========================================================================
// 11. Multiply to 64-bit (non-accumulate)
// ===========================================================================

/// Unsigned 32×32 multiply to 64-bit: `rd_pair = rs1 × rs2` (unsigned)
#[inline(always)]
pub fn mulr64(a: usize, b: usize) -> (usize, usize) {
    let lo: usize;
    let hi: usize;
    unsafe {
        asm!(
            ".insn r 0x7F,0x1, 0x78, a4, {rs1}, {rs2}",
            rs1 = in(reg) a,
            rs2 = in(reg) b,
            lateout("a4") lo,
            lateout("a5") hi,
            options(pure, nomem, nostack),
        );
    }
    (lo, hi)
}

/// Signed 32×32 multiply to 64-bit: `rd_pair = rs1 × rs2` (signed)
#[inline(always)]
pub fn mulsr64(a: usize, b: usize) -> (usize, usize) {
    let lo: usize;
    let hi: usize;
    unsafe {
        asm!(
            ".insn r 0x7F,0x1, 0x70, a4, {rs1}, {rs2}",
            rs1 = in(reg) a,
            rs2 = in(reg) b,
            lateout("a4") lo,
            lateout("a5") hi,
            options(pure, nomem, nostack),
        );
    }
    (lo, hi)
}

// ===========================================================================
// 12. I-type Instructions (immediate operand)
// ===========================================================================

/// Signed 32-bit clip to range [-2^imm, 2^imm - 1].
///
/// `IMM` must be in 0..=31.
///
/// Encoding: funct7=0x72, funct3=0x0, imm5u at bits[24:20].
#[inline(always)]
pub fn sclip32<const IMM: u8>(a: usize) -> usize {
    let value: usize;
    unsafe {
        asm!(
            ".insn i 0x7F, 0x0,{rd}, {rs1}, {imm}",
            rd = lateout(reg) value,
            rs1 = in(reg) a,
            imm = const (0x72_i32 * 32 + (IMM as i32 & 0x1F)),
            options(pure, nomem, nostack),
        );
    }
    value
}

/// Unsigned 32-bit clip to range [0, 2^imm - 1].
///
/// `IMM` must be in 0..=31.
#[inline(always)]
pub fn uclip32<const IMM: u8>(a: usize) -> usize {
    let value: usize;
    unsafe {
        asm!(
            ".insn i 0x7F, 0x0,{rd}, {rs1}, {imm}",
            rd = lateout(reg) value,
            rs1 = in(reg) a,
            imm = const (0x7A_i32 * 32 + (IMM as i32 & 0x1F)),
            options(pure, nomem, nostack),
        );
    }
    value
}

/// Saturating shift left logical word (immediate).
///
/// `rd = SAT.Q31(rs1 << IMM)`. `IMM` must be in 0..=31.
#[inline(always)]
pub fn kslliw<const IMM: u8>(a: usize) -> usize {
    let value: usize;
    unsafe {
        asm!(
            ".insn i 0x7F, 0x1, {rd}, {rs1}, {imm}",
            rd = lateout(reg) value,
            rs1 = in(reg) a,
            imm = const (0x1B_i32 * 32 + (IMM as i32 & 0x1F)),
            options(pure, nomem, nostack),
        );
    }
    value
}

// ===========================================================================
// 13. WEXT — Extract 32-bit from 64-bit pair
// ===========================================================================

/// Extract 32-bit word from 64-bit register pair at bit position `lsb`.
///
/// `rd = (rs1_pair >> lsb)[31:0]` where rs2[4:0] = lsb position.
#[inline(always)]
pub fn wext(pair: (usize, usize), lsb: usize) -> usize {
    let value: usize;
    unsafe {
        asm!(
            ".insn r 0x7F,0x0, 0x67, {rd}, a4, {rs2}",
            rd = lateout(reg) value,
            in("a4") pair.0,
            in("a5") pair.1,
            rs2 = in(reg) lsb,
            options(pure, nomem, nostack),
        );
    }
    value
}

// ===========================================================================
// 14. BPICK — Bit-wise pick (R4-type, 3 source operands)
// ===========================================================================

/// Bit-wise pick: `rd = (rs1 & rc) | (rs2 & ~rc)`
///
/// Selects bits from rs1 where rc=1, from rs2 where rc=0.
///
/// **Note**: Uses R4-type encoding (funct2=0x3, funct3=0x2).
/// Fixed register (s0) for the rc operand.
#[inline(always)]
pub fn bpick(a: usize, b: usize, rc: usize) -> usize {
    let value: usize;
    unsafe {
        asm!(
            // R4-type: [31:30]=funct2(11), [29:25]=rc, [24:20]=rs2, [19:15]=rs1,
            //          [14:12]=funct3(010), [11:7]=rd, [6:0]=0x7F
            // Use .insn r4 format
            ".insn r4 0x7F, 0x2, 0x3, {rd}, {rs1}, {rs2}, {rc}",
            rd = lateout(reg) value,
            rs1 = in(reg) a,
            rs2 = in(reg) b,
            rc = in(reg) rc,
            options(pure, nomem, nostack),
        );
    }
    value
}

// ===========================================================================
// 15. INSB — Insert byte
// ===========================================================================

/// Insert the lowest byte of rs1 into byte position `POS` of rd.
///
/// `POS` must be in 0..=3.
#[inline(always)]
pub fn insb<const POS: u8>(t: usize, a: usize) -> usize {
    let mut value: usize;
    unsafe {
        asm!(
            ".insn i 0x7F, 0x0,{rd}, {rs1}, {imm}",
            rd = inlateout(reg) t => value,
            rs1 = in(reg) a,
            imm = const (0x56_i32 * 32 + (POS as i32 & 0x3)),
            options(pure, nomem, nostack),
        );
    }
    value
}
