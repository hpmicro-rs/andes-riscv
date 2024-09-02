#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Instruction local memory base address."]
#[inline(always)]
pub const fn milmb() -> crate::common::Reg<regs::Milmb, CSR_MILMB, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MILMB;
impl crate::common::SealedCSR for CSR_MILMB {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7c0, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7c0, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MILMB {}
#[doc = "Data local memory base address."]
#[inline(always)]
pub const fn mdlmb() -> crate::common::Reg<regs::Mdlmb, CSR_MDLMB, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MDLMB;
impl crate::common::SealedCSR for CSR_MDLMB {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7c1, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7c1, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MDLMB {}
#[doc = "ECC code."]
#[inline(always)]
pub const fn mecc_code() -> crate::common::Reg<regs::MeccCode, CSR_MECC_CODE, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MECC_CODE;
impl crate::common::SealedCSR for CSR_MECC_CODE {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7c2, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7c2, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MECC_CODE {}
#[doc = "NMI-handler base address."]
#[inline(always)]
pub const fn mnvec() -> crate::common::Reg<u32, CSR_MNVEC, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MNVEC;
impl crate::common::SealedCSR for CSR_MNVEC {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7c3, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7c3, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MNVEC {}
#[doc = "Additional machine mode status"]
#[inline(always)]
pub const fn mxstatus() -> crate::common::Reg<regs::Mxstatus, CSR_MXSTATUS, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MXSTATUS;
impl crate::common::SealedCSR for CSR_MXSTATUS {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7c4, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7c4, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MXSTATUS {}
#[doc = "Performance throttling control"]
#[inline(always)]
pub const fn mpft_ctl() -> crate::common::Reg<regs::MpftCtl, CSR_MPFT_CTL, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MPFT_CTL;
impl crate::common::SealedCSR for CSR_MPFT_CTL {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7c5, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7c5, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MPFT_CTL {}
#[doc = "Hardware stack protection control"]
#[inline(always)]
pub const fn mhsp_ctl() -> crate::common::Reg<regs::MhspCtl, CSR_MHSP_CTL, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MHSP_CTL;
impl crate::common::SealedCSR for CSR_MHSP_CTL {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7c6, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7c6, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MHSP_CTL {}
#[doc = "SP bound register"]
#[inline(always)]
pub const fn msp_bound() -> crate::common::Reg<u32, CSR_MSP_BOUND, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MSP_BOUND;
impl crate::common::SealedCSR for CSR_MSP_BOUND {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7c7, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7c7, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MSP_BOUND {}
#[doc = "SP base register"]
#[inline(always)]
pub const fn msp_base() -> crate::common::Reg<u32, CSR_MSP_BASE, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MSP_BASE;
impl crate::common::SealedCSR for CSR_MSP_BASE {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7c8, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7c8, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MSP_BASE {}
#[doc = "Detailed exception cause"]
#[inline(always)]
pub const fn mdcause() -> crate::common::Reg<regs::Mdcause, CSR_MDCAUSE, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MDCAUSE;
impl crate::common::SealedCSR for CSR_MDCAUSE {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7c9, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7c9, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MDCAUSE {}
#[doc = "Cache control"]
#[inline(always)]
pub const fn mcache_ctl() -> crate::common::Reg<regs::McacheCtl, CSR_MCACHE_CTL, crate::common::RW>
{
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MCACHE_CTL;
impl crate::common::SealedCSR for CSR_MCACHE_CTL {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7ca, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7ca, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MCACHE_CTL {}
#[doc = "CCTL begin address"]
#[inline(always)]
pub const fn mcctlbeginaddr() -> crate::common::Reg<u32, CSR_MCCTLBEGINADDR, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MCCTLBEGINADDR;
impl crate::common::SealedCSR for CSR_MCCTLBEGINADDR {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7cb, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7cb, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MCCTLBEGINADDR {}
#[doc = "CCTL command"]
#[inline(always)]
pub const fn mcctlcommand() -> crate::common::Reg<u32, CSR_MCCTLCOMMAND, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MCCTLCOMMAND;
impl crate::common::SealedCSR for CSR_MCCTLCOMMAND {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7cc, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7cc, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MCCTLCOMMAND {}
#[doc = "CCTL data"]
#[inline(always)]
pub const fn mcctldata() -> crate::common::Reg<u32, CSR_MCCTLDATA, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MCCTLDATA;
impl crate::common::SealedCSR for CSR_MCCTLDATA {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7cd, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7cd, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MCCTLDATA {}
#[doc = "Counter write enable"]
#[inline(always)]
pub const fn mcounterwen(
) -> crate::common::Reg<regs::McounterCommon, CSR_MCOUNTERWEN, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MCOUNTERWEN;
impl crate::common::SealedCSR for CSR_MCOUNTERWEN {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7ce, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7ce, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MCOUNTERWEN {}
#[doc = "Counter overflow interrupt enable"]
#[inline(always)]
pub const fn mcounterinten(
) -> crate::common::Reg<regs::McounterCommon, CSR_MCOUNTERINTEN, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MCOUNTERINTEN;
impl crate::common::SealedCSR for CSR_MCOUNTERINTEN {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7cf, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7cf, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MCOUNTERINTEN {}
#[doc = "Miscellaneous control"]
#[inline(always)]
pub const fn mmisc_ctl() -> crate::common::Reg<regs::MmiscCtl, CSR_MMISC_CTL, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MMISC_CTL;
impl crate::common::SealedCSR for CSR_MMISC_CTL {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7d0, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7d0, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MMISC_CTL {}
#[doc = "Counter not counting in M-mode"]
#[inline(always)]
pub const fn mcountermask_m(
) -> crate::common::Reg<regs::McounterCommon, CSR_MCOUNTERMASK_M, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MCOUNTERMASK_M;
impl crate::common::SealedCSR for CSR_MCOUNTERMASK_M {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7d1, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7d1, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MCOUNTERMASK_M {}
#[doc = "Counter not counting in S-mode"]
#[inline(always)]
pub const fn mcountermask_s(
) -> crate::common::Reg<regs::McounterCommon, CSR_MCOUNTERMASK_S, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MCOUNTERMASK_S;
impl crate::common::SealedCSR for CSR_MCOUNTERMASK_S {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7d2, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7d2, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MCOUNTERMASK_S {}
#[doc = "Counter not counting in U-mode"]
#[inline(always)]
pub const fn mcountermask_u(
) -> crate::common::Reg<regs::McounterCommon, CSR_MCOUNTERMASK_U, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MCOUNTERMASK_U;
impl crate::common::SealedCSR for CSR_MCOUNTERMASK_U {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7d3, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7d3, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MCOUNTERMASK_U {}
#[doc = "Counter overflow status"]
#[inline(always)]
pub const fn mcounterovf(
) -> crate::common::Reg<regs::McounterCommon, CSR_MCOUNTEROVF, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MCOUNTEROVF;
impl crate::common::SealedCSR for CSR_MCOUNTEROVF {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7d4, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7d4, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MCOUNTEROVF {}
#[doc = "Supervisor local interrupt delegation"]
#[inline(always)]
pub const fn mslideleg() -> crate::common::Reg<regs::Mslideleg, CSR_MSLIDELEG, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MSLIDELEG;
impl crate::common::SealedCSR for CSR_MSLIDELEG {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7d5, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7d5, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MSLIDELEG {}
#[doc = "Status save register (level 1 & level 2)"]
#[inline(always)]
pub const fn msavestatus(
) -> crate::common::Reg<regs::Msavestatus, CSR_MSAVESTATUS, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MSAVESTATUS;
impl crate::common::SealedCSR for CSR_MSAVESTATUS {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7d6, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7d6, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MSAVESTATUS {}
#[doc = "EPC save register (level 1)"]
#[inline(always)]
pub const fn msaveepc1() -> crate::common::Reg<u32, CSR_MSAVEEPC1, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MSAVEEPC1;
impl crate::common::SealedCSR for CSR_MSAVEEPC1 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7d7, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7d7, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MSAVEEPC1 {}
#[doc = "Exception cause save register (level 1)"]
#[inline(always)]
pub const fn msavecause1() -> crate::common::Reg<u32, CSR_MSAVECAUSE1, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MSAVECAUSE1;
impl crate::common::SealedCSR for CSR_MSAVECAUSE1 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7d8, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7d8, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MSAVECAUSE1 {}
#[doc = "EPC save register (level 2)"]
#[inline(always)]
pub const fn msaveepc2() -> crate::common::Reg<u32, CSR_MSAVEEPC2, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MSAVEEPC2;
impl crate::common::SealedCSR for CSR_MSAVEEPC2 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7d9, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7d9, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MSAVEEPC2 {}
#[doc = "Exception cause save register (level 2)"]
#[inline(always)]
pub const fn msavecause2() -> crate::common::Reg<u32, CSR_MSAVECAUSE2, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MSAVECAUSE2;
impl crate::common::SealedCSR for CSR_MSAVECAUSE2 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7da, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7da, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MSAVECAUSE2 {}
#[doc = "Detailed exception cause save (level 1)"]
#[inline(always)]
pub const fn msavedcause1() -> crate::common::Reg<u32, CSR_MSAVEDCAUSE1, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MSAVEDCAUSE1;
impl crate::common::SealedCSR for CSR_MSAVEDCAUSE1 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7db, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7db, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MSAVEDCAUSE1 {}
#[doc = "Detailed exception cause save (level 2)"]
#[inline(always)]
pub const fn msavedcause2() -> crate::common::Reg<u32, CSR_MSAVEDCAUSE2, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MSAVEDCAUSE2;
impl crate::common::SealedCSR for CSR_MSAVEDCAUSE2 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7dc, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7dc, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MSAVEDCAUSE2 {}
#[doc = "Clock control"]
#[inline(always)]
pub const fn mclk_ctl() -> crate::common::Reg<u32, CSR_MCLK_CTL, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MCLK_CTL;
impl crate::common::SealedCSR for CSR_MCLK_CTL {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7df, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7df, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MCLK_CTL {}
#[doc = "Enable exception to enter Halt Mode."]
#[inline(always)]
pub const fn dexc2dbg() -> crate::common::Reg<regs::Dexc2dbg, CSR_DEXC2DBG, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_DEXC2DBG;
impl crate::common::SealedCSR for CSR_DEXC2DBG {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7e0, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7e0, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_DEXC2DBG {}
#[doc = "Detailed exception type information when an exception enters Halt Mode."]
#[inline(always)]
pub const fn ddcause() -> crate::common::Reg<regs::Ddcause, CSR_DDCAUSE, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_DDCAUSE;
impl crate::common::SealedCSR for CSR_DDCAUSE {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7e1, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7e1, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_DDCAUSE {}
#[doc = "Store mxstatus to stack"]
#[inline(always)]
pub const fn pushmxstatus() -> crate::common::Reg<u32, CSR_PUSHMXSTATUS, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PUSHMXSTATUS;
impl crate::common::SealedCSR for CSR_PUSHMXSTATUS {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7eb, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7eb, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PUSHMXSTATUS {}
#[doc = "Interrupt common entry point"]
#[inline(always)]
pub const fn mirq_entry() -> crate::common::Reg<regs::MirqEntry, CSR_MIRQ_ENTRY, crate::common::RW>
{
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MIRQ_ENTRY;
impl crate::common::SealedCSR for CSR_MIRQ_ENTRY {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7ec, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7ec, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MIRQ_ENTRY {}
#[doc = "Select interrupt and call ISR"]
#[inline(always)]
pub const fn mintsel_jal() -> crate::common::Reg<u32, CSR_MINTSEL_JAL, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MINTSEL_JAL;
impl crate::common::SealedCSR for CSR_MINTSEL_JAL {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7ed, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7ed, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MINTSEL_JAL {}
#[doc = "Store mcause to stack"]
#[inline(always)]
pub const fn pushmcause() -> crate::common::Reg<u32, CSR_PUSHMCAUSE, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PUSHMCAUSE;
impl crate::common::SealedCSR for CSR_PUSHMCAUSE {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7ee, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7ee, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PUSHMCAUSE {}
#[doc = "Store mepc to stack"]
#[inline(always)]
pub const fn pushmepc() -> crate::common::Reg<u32, CSR_PUSHMEPC, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PUSHMEPC;
impl crate::common::SealedCSR for CSR_PUSHMEPC {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7ef, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7ef, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PUSHMEPC {}
#[doc = "Private peripheral interface base address"]
#[inline(always)]
pub const fn mppib() -> crate::common::Reg<regs::Mppib, CSR_MPPIB, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MPPIB;
impl crate::common::SealedCSR for CSR_MPPIB {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7f0, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7f0, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MPPIB {}
#[doc = "Fast IO interface base address"]
#[inline(always)]
pub const fn mfiob() -> crate::common::Reg<regs::Mfiob, CSR_MFIOB, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MFIOB;
impl crate::common::SealedCSR for CSR_MFIOB {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x7f1, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x7f1, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MFIOB {}
#[doc = "Instruction table base address for CoDense extension."]
#[inline(always)]
pub const fn uitb() -> crate::common::Reg<regs::Uitb, CSR_UITB, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_UITB;
impl crate::common::SealedCSR for CSR_UITB {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x800, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x800, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_UITB {}
#[doc = "Contains overflow flag for DSP extension."]
#[inline(always)]
pub const fn ucode() -> crate::common::Reg<regs::Ucode, CSR_UCODE, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_UCODE;
impl crate::common::SealedCSR for CSR_UCODE {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x801, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x801, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_UCODE {}
#[doc = "User detailed trap cause"]
#[inline(always)]
pub const fn udcause() -> crate::common::Reg<u32, CSR_UDCAUSE, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_UDCAUSE;
impl crate::common::SealedCSR for CSR_UDCAUSE {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x809, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x809, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_UDCAUSE {}
#[doc = "CCTL begin address"]
#[inline(always)]
pub const fn ucctlbeginaddr() -> crate::common::Reg<u32, CSR_UCCTLBEGINADDR, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_UCCTLBEGINADDR;
impl crate::common::SealedCSR for CSR_UCCTLBEGINADDR {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x80b, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x80b, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_UCCTLBEGINADDR {}
#[doc = "CCTL command"]
#[inline(always)]
pub const fn ucctlcommand() -> crate::common::Reg<u32, CSR_UCCTLCOMMAND, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_UCCTLCOMMAND;
impl crate::common::SealedCSR for CSR_UCCTLCOMMAND {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x80c, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x80c, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_UCCTLCOMMAND {}
#[doc = "Wait for event control"]
#[inline(always)]
pub const fn wfe() -> crate::common::Reg<regs::Wfe, CSR_WFE, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_WFE;
impl crate::common::SealedCSR for CSR_WFE {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x810, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x810, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_WFE {}
#[doc = "Sleep value"]
#[inline(always)]
pub const fn sleepvalue() -> crate::common::Reg<regs::Sleepvalue, CSR_SLEEPVALUE, crate::common::RW>
{
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_SLEEPVALUE;
impl crate::common::SealedCSR for CSR_SLEEPVALUE {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x811, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x811, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_SLEEPVALUE {}
#[doc = "Transmit event"]
#[inline(always)]
pub const fn txevt() -> crate::common::Reg<regs::Txevt, CSR_TXEVT, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_TXEVT;
impl crate::common::SealedCSR for CSR_TXEVT {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x812, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x812, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_TXEVT {}
#[doc = "Supervisor local interrupt enable"]
#[inline(always)]
pub const fn slie() -> crate::common::Reg<regs::Slie, CSR_SLIE, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_SLIE;
impl crate::common::SealedCSR for CSR_SLIE {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x9c4, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x9c4, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_SLIE {}
#[doc = "Supervisor local interrupt pending"]
#[inline(always)]
pub const fn slip() -> crate::common::Reg<regs::Slip, CSR_SLIP, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_SLIP;
impl crate::common::SealedCSR for CSR_SLIP {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x9c5, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x9c5, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_SLIP {}
#[doc = "Detailed exception cause"]
#[inline(always)]
pub const fn sdcause() -> crate::common::Reg<regs::Sdcause, CSR_SDCAUSE, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_SDCAUSE;
impl crate::common::SealedCSR for CSR_SDCAUSE {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x9c9, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x9c9, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_SDCAUSE {}
#[doc = "CCTL data"]
#[inline(always)]
pub const fn scctldata() -> crate::common::Reg<u32, CSR_SCCTLDATA, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_SCCTLDATA;
impl crate::common::SealedCSR for CSR_SCCTLDATA {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x9cd, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x9cd, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_SCCTLDATA {}
#[doc = "Counter overflow interrupt enable"]
#[inline(always)]
pub const fn scounterinten(
) -> crate::common::Reg<regs::ScounterCommon, CSR_SCOUNTERINTEN, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_SCOUNTERINTEN;
impl crate::common::SealedCSR for CSR_SCOUNTERINTEN {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x9cf, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x9cf, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_SCOUNTERINTEN {}
#[doc = "Miscellaneous control"]
#[inline(always)]
pub const fn smisc_ctl() -> crate::common::Reg<regs::SmiscCtl, CSR_SMISC_CTL, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_SMISC_CTL;
impl crate::common::SealedCSR for CSR_SMISC_CTL {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x9d0, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x9d0, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_SMISC_CTL {}
#[doc = "Counter mask for M-mode"]
#[inline(always)]
pub const fn scountermask_m(
) -> crate::common::Reg<regs::ScounterCommon, CSR_SCOUNTERMASK_M, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_SCOUNTERMASK_M;
impl crate::common::SealedCSR for CSR_SCOUNTERMASK_M {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x9d1, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x9d1, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_SCOUNTERMASK_M {}
#[doc = "Counter mask for S-mode"]
#[inline(always)]
pub const fn scountermask_s(
) -> crate::common::Reg<regs::ScounterCommon, CSR_SCOUNTERMASK_S, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_SCOUNTERMASK_S;
impl crate::common::SealedCSR for CSR_SCOUNTERMASK_S {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x9d2, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x9d2, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_SCOUNTERMASK_S {}
#[doc = "Counter mask for U-mode"]
#[inline(always)]
pub const fn scountermask_u(
) -> crate::common::Reg<regs::ScounterCommon, CSR_SCOUNTERMASK_U, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_SCOUNTERMASK_U;
impl crate::common::SealedCSR for CSR_SCOUNTERMASK_U {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x9d3, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x9d3, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_SCOUNTERMASK_U {}
#[doc = "Counter overflow status"]
#[inline(always)]
pub const fn scounterovf(
) -> crate::common::Reg<regs::ScounterCommon, CSR_SCOUNTEROVF, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_SCOUNTEROVF;
impl crate::common::SealedCSR for CSR_SCOUNTEROVF {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x9d4, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x9d4, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_SCOUNTEROVF {}
#[doc = "Supervisor counter inhibit"]
#[inline(always)]
pub const fn scountinhibit(
) -> crate::common::Reg<regs::ScounterCommon, CSR_SCOUNTINHIBIT, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_SCOUNTINHIBIT;
impl crate::common::SealedCSR for CSR_SCOUNTINHIBIT {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x9e0, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x9e0, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_SCOUNTINHIBIT {}
#[doc = "Performance monitoring event selection"]
#[inline(always)]
pub const fn shpmevent3() -> crate::common::Reg<u32, CSR_SHPMEVENT3, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_SHPMEVENT3;
impl crate::common::SealedCSR for CSR_SHPMEVENT3 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x9e3, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x9e3, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_SHPMEVENT3 {}
#[doc = "Performance monitoring event selection"]
#[inline(always)]
pub const fn shpmevent4() -> crate::common::Reg<u32, CSR_SHPMEVENT4, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_SHPMEVENT4;
impl crate::common::SealedCSR for CSR_SHPMEVENT4 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x9e4, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x9e4, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_SHPMEVENT4 {}
#[doc = "Performance monitoring event selection"]
#[inline(always)]
pub const fn shpmevent5() -> crate::common::Reg<u32, CSR_SHPMEVENT5, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_SHPMEVENT5;
impl crate::common::SealedCSR for CSR_SHPMEVENT5 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x9e5, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x9e5, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_SHPMEVENT5 {}
#[doc = "Performance monitoring event selection"]
#[inline(always)]
pub const fn shpmevent6() -> crate::common::Reg<u32, CSR_SHPMEVENT6, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_SHPMEVENT6;
impl crate::common::SealedCSR for CSR_SHPMEVENT6 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0x9e6, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0x9e6, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_SHPMEVENT6 {}
#[doc = "PMA configuration"]
#[inline(always)]
pub const fn pmacfg0() -> crate::common::Reg<regs::Pmacfg, CSR_PMACFG0, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMACFG0;
impl crate::common::SealedCSR for CSR_PMACFG0 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbc0, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbc0, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMACFG0 {}
#[doc = "PMA configuration"]
#[inline(always)]
pub const fn pmacfg1() -> crate::common::Reg<regs::Pmacfg, CSR_PMACFG1, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMACFG1;
impl crate::common::SealedCSR for CSR_PMACFG1 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbc0, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbc0, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMACFG1 {}
#[doc = "PMA configuration"]
#[inline(always)]
pub const fn pmacfg2() -> crate::common::Reg<regs::Pmacfg, CSR_PMACFG2, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMACFG2;
impl crate::common::SealedCSR for CSR_PMACFG2 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbc0, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbc0, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMACFG2 {}
#[doc = "PMA configuration"]
#[inline(always)]
pub const fn pmacfg3() -> crate::common::Reg<regs::Pmacfg, CSR_PMACFG3, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMACFG3;
impl crate::common::SealedCSR for CSR_PMACFG3 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbc0, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbc0, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMACFG3 {}
#[doc = "PMA address"]
#[inline(always)]
pub const fn pmaaddr0() -> crate::common::Reg<u32, CSR_PMAADDR0, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMAADDR0;
impl crate::common::SealedCSR for CSR_PMAADDR0 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbd0, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbd0, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMAADDR0 {}
#[doc = "PMA address"]
#[inline(always)]
pub const fn pmaaddr1() -> crate::common::Reg<u32, CSR_PMAADDR1, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMAADDR1;
impl crate::common::SealedCSR for CSR_PMAADDR1 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbd1, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbd1, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMAADDR1 {}
#[doc = "PMA address"]
#[inline(always)]
pub const fn pmaaddr2() -> crate::common::Reg<u32, CSR_PMAADDR2, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMAADDR2;
impl crate::common::SealedCSR for CSR_PMAADDR2 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbd2, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbd2, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMAADDR2 {}
#[doc = "PMA address"]
#[inline(always)]
pub const fn pmaaddr3() -> crate::common::Reg<u32, CSR_PMAADDR3, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMAADDR3;
impl crate::common::SealedCSR for CSR_PMAADDR3 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbd3, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbd3, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMAADDR3 {}
#[doc = "PMA address"]
#[inline(always)]
pub const fn pmaaddr4() -> crate::common::Reg<u32, CSR_PMAADDR4, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMAADDR4;
impl crate::common::SealedCSR for CSR_PMAADDR4 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbd4, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbd4, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMAADDR4 {}
#[doc = "PMA address"]
#[inline(always)]
pub const fn pmaaddr5() -> crate::common::Reg<u32, CSR_PMAADDR5, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMAADDR5;
impl crate::common::SealedCSR for CSR_PMAADDR5 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbd5, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbd5, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMAADDR5 {}
#[doc = "PMA address"]
#[inline(always)]
pub const fn pmaaddr6() -> crate::common::Reg<u32, CSR_PMAADDR6, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMAADDR6;
impl crate::common::SealedCSR for CSR_PMAADDR6 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbd6, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbd6, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMAADDR6 {}
#[doc = "PMA address"]
#[inline(always)]
pub const fn pmaaddr7() -> crate::common::Reg<u32, CSR_PMAADDR7, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMAADDR7;
impl crate::common::SealedCSR for CSR_PMAADDR7 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbd7, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbd7, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMAADDR7 {}
#[doc = "PMA address"]
#[inline(always)]
pub const fn pmaaddr8() -> crate::common::Reg<u32, CSR_PMAADDR8, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMAADDR8;
impl crate::common::SealedCSR for CSR_PMAADDR8 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbd8, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbd8, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMAADDR8 {}
#[doc = "PMA address"]
#[inline(always)]
pub const fn pmaaddr9() -> crate::common::Reg<u32, CSR_PMAADDR9, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMAADDR9;
impl crate::common::SealedCSR for CSR_PMAADDR9 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbd9, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbd9, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMAADDR9 {}
#[doc = "PMA address"]
#[inline(always)]
pub const fn pmaaddr10() -> crate::common::Reg<u32, CSR_PMAADDR10, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMAADDR10;
impl crate::common::SealedCSR for CSR_PMAADDR10 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbda, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbda, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMAADDR10 {}
#[doc = "PMA address"]
#[inline(always)]
pub const fn pmaaddr11() -> crate::common::Reg<u32, CSR_PMAADDR11, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMAADDR11;
impl crate::common::SealedCSR for CSR_PMAADDR11 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbdb, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbdb, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMAADDR11 {}
#[doc = "PMA address"]
#[inline(always)]
pub const fn pmaaddr12() -> crate::common::Reg<u32, CSR_PMAADDR12, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMAADDR12;
impl crate::common::SealedCSR for CSR_PMAADDR12 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbdc, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbdc, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMAADDR12 {}
#[doc = "PMA address"]
#[inline(always)]
pub const fn pmaaddr13() -> crate::common::Reg<u32, CSR_PMAADDR13, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMAADDR13;
impl crate::common::SealedCSR for CSR_PMAADDR13 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbdd, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbdd, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMAADDR13 {}
#[doc = "PMA address"]
#[inline(always)]
pub const fn pmaaddr14() -> crate::common::Reg<u32, CSR_PMAADDR14, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMAADDR14;
impl crate::common::SealedCSR for CSR_PMAADDR14 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbde, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbde, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMAADDR14 {}
#[doc = "PMA address"]
#[inline(always)]
pub const fn pmaaddr15() -> crate::common::Reg<u32, CSR_PMAADDR15, crate::common::RW> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_PMAADDR15;
impl crate::common::SealedCSR for CSR_PMAADDR15 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xbdf, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xbdf, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_PMAADDR15 {}
#[doc = "Instruction cache/memory configuration"]
#[inline(always)]
pub const fn micm_cfg() -> crate::common::Reg<regs::MicmCfg, CSR_MICM_CFG, crate::common::R> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MICM_CFG;
impl crate::common::SealedCSR for CSR_MICM_CFG {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xfc0, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xfc0, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MICM_CFG {}
#[doc = "Data cache/memory configuration"]
#[inline(always)]
pub const fn mdcm_cfg() -> crate::common::Reg<regs::MdcmCfg, CSR_MDCM_CFG, crate::common::R> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MDCM_CFG;
impl crate::common::SealedCSR for CSR_MDCM_CFG {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xfc1, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xfc1, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MDCM_CFG {}
#[doc = "Miscellaneous configuration"]
#[inline(always)]
pub const fn mmsc_cfg() -> crate::common::Reg<regs::MmscCfg, CSR_MMSC_CFG, crate::common::R> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MMSC_CFG;
impl crate::common::SealedCSR for CSR_MMSC_CFG {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xfc2, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xfc2, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MMSC_CFG {}
#[doc = "Miscellaneous configuration (RV32)"]
#[inline(always)]
pub const fn mmsc_cfg2() -> crate::common::Reg<regs::MmscCfg2, CSR_MMSC_CFG2, crate::common::R> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MMSC_CFG2;
impl crate::common::SealedCSR for CSR_MMSC_CFG2 {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xfc3, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xfc3, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MMSC_CFG2 {}
#[doc = "Vector processor configuration"]
#[inline(always)]
pub const fn mvec_cfg() -> crate::common::Reg<regs::MvecCfg, CSR_MVEC_CFG, crate::common::R> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MVEC_CFG;
impl crate::common::SealedCSR for CSR_MVEC_CFG {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xfc7, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xfc7, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MVEC_CFG {}
#[doc = "Current state save for crash debugging"]
#[inline(always)]
pub const fn mcrash_statesave(
) -> crate::common::Reg<regs::McrashStatesave, CSR_MCRASH_STATESAVE, crate::common::R> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MCRASH_STATESAVE;
impl crate::common::SealedCSR for CSR_MCRASH_STATESAVE {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xfc8, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xfc8, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MCRASH_STATESAVE {}
#[doc = "mstatus state save for crash debugging"]
#[inline(always)]
pub const fn mstatus_crashsave() -> crate::common::Reg<u32, CSR_MSTATUS_CRASHSAVE, crate::common::R>
{
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MSTATUS_CRASHSAVE;
impl crate::common::SealedCSR for CSR_MSTATUS_CRASHSAVE {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xfc9, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xfc9, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MSTATUS_CRASHSAVE {}
#[doc = "RISC-V Architecture"]
#[inline(always)]
pub const fn mrvarch_cfg() -> crate::common::Reg<regs::MrvarchCfg, CSR_MRVARCH_CFG, crate::common::R>
{
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MRVARCH_CFG;
impl crate::common::SealedCSR for CSR_MRVARCH_CFG {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xfca, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xfca, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MRVARCH_CFG {}
#[doc = "Cluster cache control base address"]
#[inline(always)]
pub const fn mccache_ctl_base() -> crate::common::Reg<u32, CSR_MCCACHE_CTL_BASE, crate::common::R> {
    unsafe { crate::common::Reg::new() }
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct CSR_MCCACHE_CTL_BASE;
impl crate::common::SealedCSR for CSR_MCCACHE_CTL_BASE {
    #[inline]
    unsafe fn read_csr() -> usize {
        let r: usize;
        core :: arch :: asm ! ("csrrs {0}, 0xfcf, x0" , out (reg) r);
        r
    }
    #[inline]
    unsafe fn write_csr(value: usize) {
        core :: arch :: asm ! ("cswrs 0xfcf, {0}, x0" , in (reg) value);
    }
}
impl crate::common::CSR for CSR_MCCACHE_CTL_BASE {}
pub mod regs {
    #[doc = "Debug Detailed Cause Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ddcause(pub u32);
    impl Ddcause {
        #[doc = "Indicates the main types of a Debug Mode entrance. Its definition is listed below."]
        #[inline(always)]
        pub const fn maintype(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Indicates the main types of a Debug Mode entrance. Its definition is listed below."]
        #[inline(always)]
        pub fn set_maintype(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Indicates the subtypes of a main type. Its definition is listed below. A main type may not have a subtype definition."]
        #[inline(always)]
        pub const fn subtype(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Indicates the subtypes of a main type. Its definition is listed below. A main type may not have a subtype definition."]
        #[inline(always)]
        pub fn set_subtype(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Ddcause {
        #[inline(always)]
        fn default() -> Ddcause {
            Ddcause(0)
        }
    }
    #[doc = "Exception Redirection Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dexc2dbg(pub u32);
    impl Dexc2dbg {
        #[inline(always)]
        pub const fn iam(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_iam(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn iaf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_iaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ii(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ii(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn nmi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_nmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn lam(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_lam(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn laf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_laf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn sam(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_sam(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn saf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_saf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn uec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_uec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn sec(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn hec(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_hec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn mec(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_mec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn hsp(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_hsp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn ace(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ace(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn slpecc(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_slpecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn bwe(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_bwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn ipf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ipf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn lpf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_lpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn spf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_spf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn pmov(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_pmov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Dexc2dbg {
        #[inline(always)]
        fn default() -> Dexc2dbg {
            Dexc2dbg(0)
        }
    }
    #[doc = "Cache Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McacheCtl(pub u32);
    impl McacheCtl {
        #[doc = "Instruction Cache Enable"]
        #[inline(always)]
        pub const fn ic_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction Cache Enable"]
        #[inline(always)]
        pub fn set_ic_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data Cache Enable"]
        #[inline(always)]
        pub const fn dc_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data Cache Enable"]
        #[inline(always)]
        pub fn set_dc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Instruction Cache ECC Enable"]
        #[inline(always)]
        pub const fn ic_eccen(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Instruction Cache ECC Enable"]
        #[inline(always)]
        pub fn set_ic_eccen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Data Cache ECC Enable"]
        #[inline(always)]
        pub const fn dc_eccen(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Data Cache ECC Enable"]
        #[inline(always)]
        pub fn set_dc_eccen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Instruction Cache Read/Write ECC"]
        #[inline(always)]
        pub const fn ic_rwecc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction Cache Read/Write ECC"]
        #[inline(always)]
        pub fn set_ic_rwecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Data Cache Read/Write ECC"]
        #[inline(always)]
        pub const fn dc_rwecc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Data Cache Read/Write ECC"]
        #[inline(always)]
        pub fn set_dc_rwecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Cache Control SU Enable"]
        #[inline(always)]
        pub const fn cctl_suen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Cache Control SU Enable"]
        #[inline(always)]
        pub fn set_cctl_suen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Instruction Prefetch Enable"]
        #[inline(always)]
        pub const fn ipref_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction Prefetch Enable"]
        #[inline(always)]
        pub fn set_ipref_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Data Prefetch Enable"]
        #[inline(always)]
        pub const fn dpref_en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Data Prefetch Enable"]
        #[inline(always)]
        pub fn set_dpref_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Instruction Cache 1st Way Disable"]
        #[inline(always)]
        pub const fn ic_1st_wd(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction Cache 1st Way Disable"]
        #[inline(always)]
        pub fn set_ic_1st_wd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Data Cache 1st Way Disable"]
        #[inline(always)]
        pub const fn dc_1st_wd(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Data Cache 1st Way Disable"]
        #[inline(always)]
        pub fn set_dc_1st_wd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Data Cache Write-Around"]
        #[inline(always)]
        pub const fn dc_warnd(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "Data Cache Write-Around"]
        #[inline(always)]
        pub fn set_dc_warnd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "L2 Cache Write-Around"]
        #[inline(always)]
        pub const fn l2c_warnd(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x03;
            val as u8
        }
        #[doc = "L2 Cache Write-Around"]
        #[inline(always)]
        pub fn set_l2c_warnd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 15usize)) | (((val as u32) & 0x03) << 15usize);
        }
        #[doc = "TLB ECC Enable"]
        #[inline(always)]
        pub const fn tlb_eccen(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "TLB ECC Enable"]
        #[inline(always)]
        pub fn set_tlb_eccen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "Data Cache Coherence Enable"]
        #[inline(always)]
        pub const fn dc_cohen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Data Cache Coherence Enable"]
        #[inline(always)]
        pub fn set_dc_cohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Data Cache Coherence State"]
        #[inline(always)]
        pub const fn dc_cohsta(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Data Cache Coherence State"]
        #[inline(always)]
        pub fn set_dc_cohsta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Data Prefetch Mode"]
        #[inline(always)]
        pub const fn dpref_mode(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[doc = "Data Prefetch Mode"]
        #[inline(always)]
        pub fn set_dpref_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
    }
    impl Default for McacheCtl {
        #[inline(always)]
        fn default() -> McacheCtl {
            McacheCtl(0)
        }
    }
    #[doc = "Machine Counter Fields"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McounterCommon(pub u32);
    impl McounterCommon {
        #[doc = "Cycle counter"]
        #[inline(always)]
        pub const fn cy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cycle counter"]
        #[inline(always)]
        pub fn set_cy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Instruction retired counter"]
        #[inline(always)]
        pub const fn ir(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction retired counter"]
        #[inline(always)]
        pub fn set_ir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[inline(always)]
        pub const fn hpm(&self, n: usize) -> bool {
            assert!(n < 29usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[inline(always)]
        pub fn set_hpm(&mut self, n: usize, val: bool) {
            assert!(n < 29usize);
            let offs = 3usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for McounterCommon {
        #[inline(always)]
        fn default() -> McounterCommon {
            McounterCommon(0)
        }
    }
    #[doc = "Machine Crash State Save"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McrashStatesave(pub u32);
    impl McrashStatesave {
        #[inline(always)]
        pub const fn mie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_mie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn cp(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_cp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[inline(always)]
        pub const fn ppft_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ppft_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn pime(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_pime(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn pdme(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_pdme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn ptyp(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for McrashStatesave {
        #[inline(always)]
        fn default() -> McrashStatesave {
            McrashStatesave(0)
        }
    }
    #[doc = "Machine Detailed Trap Cause Register (for imprecise exception/interrupt)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdcause(pub u32);
    impl Mdcause {
        #[inline(always)]
        pub const fn mdcause(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_mdcause(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn pm(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_pm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
    }
    impl Default for Mdcause {
        #[inline(always)]
        fn default() -> Mdcause {
            Mdcause(0)
        }
    }
    #[doc = "Data Cache/Memory Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdcmCfg(pub u32);
    impl MdcmCfg {
        #[inline(always)]
        pub const fn dset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_dset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn dway(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_dway(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[inline(always)]
        pub const fn dsz(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_dsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[inline(always)]
        pub const fn dlck(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_dlck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn dc_ecc(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_dc_ecc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn dlmb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_dlmb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[inline(always)]
        pub const fn dlmsz(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_dlmsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
        #[inline(always)]
        pub const fn ulm_2bank(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ulm_2bank(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn dlm_ecc(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_dlm_ecc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[inline(always)]
        pub const fn seth(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_seth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn dc_repl(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_dc_repl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
    }
    impl Default for MdcmCfg {
        #[inline(always)]
        fn default() -> MdcmCfg {
            MdcmCfg(0)
        }
    }
    #[doc = "DLM (Data Local Memory) Base Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdlmb(pub u32);
    impl Mdlmb {
        #[doc = "DLM Enable"]
        #[inline(always)]
        pub const fn den(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DLM Enable"]
        #[inline(always)]
        pub fn set_den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ECC Enable"]
        #[inline(always)]
        pub const fn eccen(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "ECC Enable"]
        #[inline(always)]
        pub fn set_eccen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Read/Write ECC"]
        #[inline(always)]
        pub const fn rwecc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Read/Write ECC"]
        #[inline(always)]
        pub fn set_rwecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Reserved"]
        #[inline(always)]
        pub const fn reserved(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x3f;
            val as u8
        }
        #[doc = "Reserved"]
        #[inline(always)]
        pub fn set_reserved(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
        }
        #[doc = "Base Physical Address (DBPA)"]
        #[inline(always)]
        pub const fn dbpa(&self) -> u32 {
            let val = (self.0 >> 10usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "Base Physical Address (DBPA)"]
        #[inline(always)]
        pub fn set_dbpa(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
        }
    }
    impl Default for Mdlmb {
        #[inline(always)]
        fn default() -> Mdlmb {
            Mdlmb(0)
        }
    }
    #[doc = "ECC Code Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MeccCode(pub u32);
    impl MeccCode {
        #[doc = "ECC Code"]
        #[inline(always)]
        pub const fn code(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "ECC Code"]
        #[inline(always)]
        pub fn set_code(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Correctable Error Flag"]
        #[inline(always)]
        pub const fn c(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Correctable Error Flag"]
        #[inline(always)]
        pub fn set_c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Parity Error Flag"]
        #[inline(always)]
        pub const fn p(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Parity Error Flag"]
        #[inline(always)]
        pub fn set_p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "RAM Identifier"]
        #[inline(always)]
        pub const fn ramid(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[doc = "RAM Identifier"]
        #[inline(always)]
        pub fn set_ramid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
        #[doc = "Instruction Error Flag"]
        #[inline(always)]
        pub const fn insn(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction Error Flag"]
        #[inline(always)]
        pub fn set_insn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Syndrome"]
        #[inline(always)]
        pub const fn syndr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Syndrome"]
        #[inline(always)]
        pub fn set_syndr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for MeccCode {
        #[inline(always)]
        fn default() -> MeccCode {
            MeccCode(0)
        }
    }
    #[doc = "FIO (Fast IO Interface) Base Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mfiob(pub u32);
    impl Mfiob {
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn size(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
        }
        #[doc = "Base Physical Address"]
        #[inline(always)]
        pub const fn bpa(&self) -> u32 {
            let val = (self.0 >> 10usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "Base Physical Address"]
        #[inline(always)]
        pub fn set_bpa(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
        }
    }
    impl Default for Mfiob {
        #[inline(always)]
        fn default() -> Mfiob {
            Mfiob(0)
        }
    }
    #[doc = "Machine Hardware Stack Protection Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MhspCtl(pub u32);
    impl MhspCtl {
        #[inline(always)]
        pub const fn ovf_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ovf_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn udf_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_udf_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn schm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_schm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn u(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_u(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn s(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn m(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_m(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for MhspCtl {
        #[inline(always)]
        fn default() -> MhspCtl {
            MhspCtl(0)
        }
    }
    #[doc = "Instruction Cache/Memory Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MicmCfg(pub u32);
    impl MicmCfg {
        #[inline(always)]
        pub const fn iset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_iset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn iway(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_iway(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[inline(always)]
        pub const fn isz(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_isz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[inline(always)]
        pub const fn ilck(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ilck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn ic_ecc(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ic_ecc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn ilmb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_ilmb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[inline(always)]
        pub const fn ilmsz(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ilmsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
        #[inline(always)]
        pub const fn ulm_2bank(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ulm_2bank(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn ilm_ecc(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ilm_ecc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[inline(always)]
        pub const fn ilm_xonly(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ilm_xonly(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn seth(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_seth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn ic_repl(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ic_repl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
    }
    impl Default for MicmCfg {
        #[inline(always)]
        fn default() -> MicmCfg {
            MicmCfg(0)
        }
    }
    #[doc = "ILM (Instruction Local Memory) Base Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Milmb(pub u32);
    impl Milmb {
        #[doc = "ILM Enable"]
        #[inline(always)]
        pub const fn ien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ILM Enable"]
        #[inline(always)]
        pub fn set_ien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ECC Enable"]
        #[inline(always)]
        pub const fn eccen(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "ECC Enable"]
        #[inline(always)]
        pub fn set_eccen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Read/Write ECC"]
        #[inline(always)]
        pub const fn rwecc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Read/Write ECC"]
        #[inline(always)]
        pub fn set_rwecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Base Physical Address (IBPA)"]
        #[inline(always)]
        pub const fn ibpa(&self) -> u32 {
            let val = (self.0 >> 10usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "Base Physical Address (IBPA)"]
        #[inline(always)]
        pub fn set_ibpa(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
        }
    }
    impl Default for Milmb {
        #[inline(always)]
        fn default() -> Milmb {
            Milmb(0)
        }
    }
    #[doc = "Machine Interrupt Common Entry Address Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MirqEntry(pub u32);
    impl MirqEntry {
        #[doc = "Enable"]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable"]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interrupt Common Entry Address"]
        #[inline(always)]
        pub const fn icea(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Interrupt Common Entry Address"]
        #[inline(always)]
        pub fn set_icea(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for MirqEntry {
        #[inline(always)]
        fn default() -> MirqEntry {
            MirqEntry(0)
        }
    }
    #[doc = "Machine Miscellaneous Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmiscCtl(pub u32);
    impl MmiscCtl {
        #[doc = "Andes Custom Extension (ACE) enable"]
        #[inline(always)]
        pub const fn ace(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Andes Custom Extension (ACE) enable"]
        #[inline(always)]
        pub fn set_ace(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Vectored external PLIC interrupt enable"]
        #[inline(always)]
        pub const fn vec_plic(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Vectored external PLIC interrupt enable"]
        #[inline(always)]
        pub fn set_vec_plic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RV compatibility mode enable bit"]
        #[inline(always)]
        pub const fn rvcompm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RV compatibility mode enable bit"]
        #[inline(always)]
        pub fn set_rvcompm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn brpe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_brpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn aces(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_aces(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn msa_una(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_msa_una(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn nbld_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_nbld_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn newnmi(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_newnmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn vcgl1_en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_vcgl1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn vcgl2_en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_vcgl2_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn vcgl3_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_vcgl3_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "“Load to x0” exception generation control bit"]
        #[inline(always)]
        pub const fn ldx0nxp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "“Load to x0” exception generation control bit"]
        #[inline(always)]
        pub fn set_ldx0nxp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for MmiscCtl {
        #[inline(always)]
        fn default() -> MmiscCtl {
            MmiscCtl(0)
        }
    }
    #[doc = "Misc. Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmscCfg(pub u32);
    impl MmscCfg {
        #[inline(always)]
        pub const fn ecc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn tlb_ecc(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_tlb_ecc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[inline(always)]
        pub const fn ecd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ecd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn pft(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_pft(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn hsp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_hsp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn ace(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ace(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn addpmc(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_addpmc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 7usize)) | (((val as u32) & 0x1f) << 7usize);
        }
        #[inline(always)]
        pub const fn vplic(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_vplic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn ev5pe(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ev5pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn lmslvp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_lmslvp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn pmnds(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_pmnds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn cctlcsr(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_cctlcsr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn efhw(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_efhw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn vcctl(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_vcctl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[inline(always)]
        pub const fn excslvl(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_excslvl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[inline(always)]
        pub const fn nopmc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_nopmc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn spe_aft(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_spe_aft(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn esleep(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_esleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn ppi(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ppi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn fio(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_fio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn clic(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_clic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn eclic(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_eclic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn edsp(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_edsp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn ppma(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ppma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn msc_ext(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_msc_ext(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MmscCfg {
        #[inline(always)]
        fn default() -> MmscCfg {
            MmscCfg(0)
        }
    }
    #[doc = "Misc. Configuration 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmscCfg2(pub u32);
    impl MmscCfg2 {
        #[inline(always)]
        pub const fn bf16cvt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_bf16cvt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn zfh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_zfh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn vl4(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_vl4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn crashsave(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_crashsave(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn veccfg(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_veccfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn finv(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_finv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn pp16(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_pp16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn vsih(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_vsih(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn ecdv(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ecdv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[inline(always)]
        pub const fn vdot(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_vdot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn vpfh(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_vpfh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn ccachemp_cfg(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ccachemp_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn ccache(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ccache(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn io_cohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_io_cohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn core_pclus(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_core_pclus(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn rvarch(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_rvarch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for MmscCfg2 {
        #[inline(always)]
        fn default() -> MmscCfg2 {
            MmscCfg2(0)
        }
    }
    #[doc = "Performance Throttling Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MpftCtl(pub u32);
    impl MpftCtl {
        #[doc = "Throttling Level"]
        #[inline(always)]
        pub const fn t_level(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Throttling Level"]
        #[inline(always)]
        pub fn set_t_level(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Fast Interrupt"]
        #[inline(always)]
        pub const fn fast_int(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Fast Interrupt"]
        #[inline(always)]
        pub fn set_fast_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for MpftCtl {
        #[inline(always)]
        fn default() -> MpftCtl {
            MpftCtl(0)
        }
    }
    #[doc = "PPI (Private Peripheral Interface) Base Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mppib(pub u32);
    impl Mppib {
        #[doc = "Private Peripheral Interface enable bit"]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Private Peripheral Interface enable bit"]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Indicates the power-of-2 size of the PPI region"]
        #[inline(always)]
        pub const fn size(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x1f;
            val as u8
        }
        #[doc = "Indicates the power-of-2 size of the PPI region"]
        #[inline(always)]
        pub fn set_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
        }
        #[doc = "Base Physical Address"]
        #[inline(always)]
        pub const fn bpa(&self) -> u32 {
            let val = (self.0 >> 10usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "Base Physical Address"]
        #[inline(always)]
        pub fn set_bpa(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
        }
    }
    impl Default for Mppib {
        #[inline(always)]
        fn default() -> Mppib {
            Mppib(0)
        }
    }
    #[doc = "RISC-V Architecture Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MrvarchCfg(pub u32);
    impl MrvarchCfg {
        #[inline(always)]
        pub const fn zba(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_zba(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn zbb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_zbb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn zbc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_zbc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn zbs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_zbs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn smepmp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_smepmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn svinval(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_svinval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn smstateen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_smstateen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn sscofmpf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_sscofmpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn sstc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_sstc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn zicbom(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_zicbom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn zicbop(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_zicbop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn zicboz(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_zicboz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn zbk(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_zbk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn zkn(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_zkn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn zks(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_zks(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn zkt(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_zkt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn zkr(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_zkr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn sm_verion(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_sm_verion(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[inline(always)]
        pub const fn ss_version(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_ss_version(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[inline(always)]
        pub const fn svpbmt(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_svpbmt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn svnapot(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_svnapot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn zihintpause(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_zihintpause(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for MrvarchCfg {
        #[inline(always)]
        fn default() -> MrvarchCfg {
            MrvarchCfg(0)
        }
    }
    #[doc = "Machine Status Save Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Msavestatus(pub u32);
    impl Msavestatus {
        #[inline(always)]
        pub const fn mpie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_mpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn mpp(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_mpp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[inline(always)]
        pub const fn ppft_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ppft_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn pime(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_pime(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn pdme(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_pdme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn ptyp(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for Msavestatus {
        #[inline(always)]
        fn default() -> Msavestatus {
            Msavestatus(0)
        }
    }
    #[doc = "Machine Supervisor Local Interrupt Delegation Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mslideleg(pub u32);
    impl Mslideleg {
        #[inline(always)]
        pub const fn imecci(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_imecci(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn bwei(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_bwei(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn pmovi(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_pmovi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn imeccdmr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_imeccdmr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn aceerr(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_aceerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Mslideleg {
        #[inline(always)]
        fn default() -> Mslideleg {
            Mslideleg(0)
        }
    }
    #[doc = "Vector Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MvecCfg(pub u32);
    impl MvecCfg {
        #[inline(always)]
        pub const fn minor(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_minor(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn major(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_major(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn dw(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_dw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[inline(always)]
        pub const fn mw(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_mw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
        }
        #[inline(always)]
        pub const fn misew(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_misew(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[inline(always)]
        pub const fn mfsew(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_mfsew(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for MvecCfg {
        #[inline(always)]
        fn default() -> MvecCfg {
            MvecCfg(0)
        }
    }
    #[doc = "Machine Extended Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mxstatus(pub u32);
    impl Mxstatus {
        #[inline(always)]
        pub const fn pft_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_pft_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ppft_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ppft_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ime(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ime(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn pime(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_pime(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn dme(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_dme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn pdme(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_pdme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn typ(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_typ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn ptyp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Mxstatus {
        #[inline(always)]
        fn default() -> Mxstatus {
            Mxstatus(0)
        }
    }
    #[doc = "PMA Configuration Register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmacfg(pub u32);
    impl Pmacfg {
        #[doc = "Event Type"]
        #[inline(always)]
        pub const fn etyp(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "Event Type"]
        #[inline(always)]
        pub fn set_etyp(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "Memory Type"]
        #[inline(always)]
        pub const fn mtyp(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "Memory Type"]
        #[inline(always)]
        pub fn set_mtyp(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
        #[doc = "Naturally Aligned Memory Operation"]
        #[inline(always)]
        pub const fn namo(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 6usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Naturally Aligned Memory Operation"]
        #[inline(always)]
        pub fn set_namo(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 6usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Pmacfg {
        #[inline(always)]
        fn default() -> Pmacfg {
            Pmacfg(0)
        }
    }
    #[doc = "Machine Counter Fields"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ScounterCommon(pub u32);
    impl ScounterCommon {
        #[doc = "Cycle counter"]
        #[inline(always)]
        pub const fn cy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cycle counter"]
        #[inline(always)]
        pub fn set_cy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Instruction retired counter"]
        #[inline(always)]
        pub const fn ir(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction retired counter"]
        #[inline(always)]
        pub fn set_ir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[inline(always)]
        pub const fn hpm(&self, n: usize) -> bool {
            assert!(n < 29usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[inline(always)]
        pub fn set_hpm(&mut self, n: usize, val: bool) {
            assert!(n < 29usize);
            let offs = 3usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for ScounterCommon {
        #[inline(always)]
        fn default() -> ScounterCommon {
            ScounterCommon(0)
        }
    }
    #[doc = "Supervisor Detailed Cause (for imprecise exception/interrupt)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdcause(pub u32);
    impl Sdcause {
        #[doc = "Detailed cause for imprecise exception"]
        #[inline(always)]
        pub const fn sdcause(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Detailed cause for imprecise exception"]
        #[inline(always)]
        pub fn set_sdcause(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Privilege Mode"]
        #[inline(always)]
        pub const fn pm(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Privilege Mode"]
        #[inline(always)]
        pub fn set_pm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
    }
    impl Default for Sdcause {
        #[inline(always)]
        fn default() -> Sdcause {
            Sdcause(0)
        }
    }
    #[doc = "Sleep Value Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sleepvalue(pub u32);
    impl Sleepvalue {
        #[doc = "Sleep value"]
        #[inline(always)]
        pub const fn sv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sleep value"]
        #[inline(always)]
        pub fn set_sv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Sleepvalue {
        #[inline(always)]
        fn default() -> Sleepvalue {
            Sleepvalue(0)
        }
    }
    #[doc = "Supervisor Local Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Slie(pub u32);
    impl Slie {
        #[doc = "Enable S-mode slave-port ECC error local interrupt"]
        #[inline(always)]
        pub const fn imecci(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Enable S-mode slave-port ECC error local interrupt"]
        #[inline(always)]
        pub fn set_imecci(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Enable S-mode bus read/write transaction error local interrupt"]
        #[inline(always)]
        pub const fn bwei(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Enable S-mode bus read/write transaction error local interrupt"]
        #[inline(always)]
        pub fn set_bwei(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Enable S-mode performance monitor overflow local interrupt"]
        #[inline(always)]
        pub const fn pmovi(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Enable S-mode performance monitor overflow local interrupt"]
        #[inline(always)]
        pub fn set_pmovi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Enable S-mode ECC DMR error local interrupt"]
        #[inline(always)]
        pub const fn imeccdmr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Enable S-mode ECC DMR error local interrupt"]
        #[inline(always)]
        pub fn set_imeccdmr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Enable S-mode ACE error local interrupt"]
        #[inline(always)]
        pub const fn aceerr(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Enable S-mode ACE error local interrupt"]
        #[inline(always)]
        pub fn set_aceerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Slie {
        #[inline(always)]
        fn default() -> Slie {
            Slie(0)
        }
    }
    #[doc = "Supervisor Local Interrupt Pending"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Slip(pub u32);
    impl Slip {
        #[inline(always)]
        pub const fn imecci(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_imecci(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn bwei(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_bwei(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn pmovi(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_pmovi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn imeccdmr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_imeccdmr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn aceerr(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_aceerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Slip {
        #[inline(always)]
        fn default() -> Slip {
            Slip(0)
        }
    }
    #[doc = "Supervisor Miscellaneous Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SmiscCtl(pub u32);
    impl SmiscCtl {
        #[doc = "ACE (Access Control Extension) Status"]
        #[inline(always)]
        pub const fn aces(&self) -> super::vals::Aces {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Aces::from_bits(val as u8)
        }
        #[doc = "ACE (Access Control Extension) Status"]
        #[inline(always)]
        pub fn set_aces(&mut self, val: super::vals::Aces) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
    }
    impl Default for SmiscCtl {
        #[inline(always)]
        fn default() -> SmiscCtl {
            SmiscCtl(0)
        }
    }
    #[doc = "Transmit Event Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txevt(pub u32);
    impl Txevt {
        #[doc = "Event output"]
        #[inline(always)]
        pub const fn evto(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Event output"]
        #[inline(always)]
        pub fn set_evto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Txevt {
        #[inline(always)]
        fn default() -> Txevt {
            Txevt(0)
        }
    }
    #[doc = "Code Register - Stores the overflow flag of the DSP extension"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ucode(pub u32);
    impl Ucode {
        #[doc = "Overflow flag"]
        #[inline(always)]
        pub const fn ov(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow flag"]
        #[inline(always)]
        pub fn set_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ucode {
        #[inline(always)]
        fn default() -> Ucode {
            Ucode(0)
        }
    }
    #[doc = "Instruction Table Base Address Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uitb(pub u32);
    impl Uitb {
        #[doc = "if the Xcodense instruction table is hardwired"]
        #[inline(always)]
        pub const fn hw(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "if the Xcodense instruction table is hardwired"]
        #[inline(always)]
        pub fn set_hw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Uitb {
        #[inline(always)]
        fn default() -> Uitb {
            Uitb(0)
        }
    }
    #[doc = "Wait for Event Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wfe(pub u32);
    impl Wfe {
        #[doc = "Wait for event"]
        #[inline(always)]
        pub const fn wfe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Wait for event"]
        #[inline(always)]
        pub fn set_wfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Wfe {
        #[inline(always)]
        fn default() -> Wfe {
            Wfe(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Aces {
        OFF = 0x0,
        INITIAL = 0x01,
        CLEAN = 0x02,
        DIRTY = 0x03,
    }
    impl Aces {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Aces {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Aces {
        #[inline(always)]
        fn from(val: u8) -> Aces {
            Aces::from_bits(val)
        }
    }
    impl From<Aces> for u8 {
        #[inline(always)]
        fn from(val: Aces) -> u8 {
            Aces::to_bits(val)
        }
    }
}
