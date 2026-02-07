#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]
#[doc = "Instruction local memory base address."]
pub mod milmb {
    #[doc = "ILM (Instruction Local Memory) Base Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Milmb(pub u32);
    impl Milmb {
        #[doc = "ILM Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ILM Enable"]
        #[inline(always)]
        pub const fn set_ien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ECC Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn eccen(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "ECC Enable"]
        #[inline(always)]
        pub const fn set_eccen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Read/Write ECC"]
        #[must_use]
        #[inline(always)]
        pub const fn rwecc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Read/Write ECC"]
        #[inline(always)]
        pub const fn set_rwecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Base Physical Address (IBPA)"]
        #[must_use]
        #[inline(always)]
        pub const fn ibpa(&self) -> u32 {
            let val = (self.0 >> 10usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "Base Physical Address (IBPA)"]
        #[inline(always)]
        pub const fn set_ibpa(&mut self, val: u32) {
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
    impl core::fmt::Debug for Milmb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Milmb")
                .field("ien", &self.ien())
                .field("eccen", &self.eccen())
                .field("rwecc", &self.rwecc())
                .field("ibpa", &self.ibpa())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Milmb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Milmb {{ ien: {=bool:?}, eccen: {=u8:?}, rwecc: {=bool:?}, ibpa: {=u32:?} }}",
                self.ien(),
                self.eccen(),
                self.rwecc(),
                self.ibpa()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7c0, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7c0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7c0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7c0, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Milmb {
        unsafe { Milmb(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Milmb) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Milmb) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "ILM Enable"]
    #[inline]
    pub unsafe fn set_ien() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "ILM Enable"]
    #[inline]
    pub unsafe fn clear_ien() {
        unsafe {
            _clear(1usize);
        }
    }
    #[doc = "ECC Enable"]
    #[inline]
    pub unsafe fn set_eccen(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(3usize << 1usize);
        bits |= (val as usize & 3usize) << 1usize;
        unsafe {
            _write(bits);
        }
    }
    #[doc = "Read/Write ECC"]
    #[inline]
    pub unsafe fn set_rwecc() {
        unsafe {
            _set(8usize);
        }
    }
    #[doc = "Read/Write ECC"]
    #[inline]
    pub unsafe fn clear_rwecc() {
        unsafe {
            _clear(8usize);
        }
    }
    #[doc = "Base Physical Address (IBPA)"]
    #[inline]
    pub unsafe fn set_ibpa(val: u32) {
        let mut bits = unsafe { _read() };
        bits &= !(4194303usize << 10usize);
        bits |= (val as usize & 4194303usize) << 10usize;
        unsafe {
            _write(bits);
        }
    }
}
#[doc = "Data local memory base address."]
pub mod mdlmb {
    #[doc = "DLM (Data Local Memory) Base Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdlmb(pub u32);
    impl Mdlmb {
        #[doc = "DLM Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn den(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DLM Enable"]
        #[inline(always)]
        pub const fn set_den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ECC Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn eccen(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "ECC Enable"]
        #[inline(always)]
        pub const fn set_eccen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Read/Write ECC"]
        #[must_use]
        #[inline(always)]
        pub const fn rwecc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Read/Write ECC"]
        #[inline(always)]
        pub const fn set_rwecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Reserved"]
        #[must_use]
        #[inline(always)]
        pub const fn reserved(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x3f;
            val as u8
        }
        #[doc = "Reserved"]
        #[inline(always)]
        pub const fn set_reserved(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
        }
        #[doc = "Base Physical Address (DBPA)"]
        #[must_use]
        #[inline(always)]
        pub const fn dbpa(&self) -> u32 {
            let val = (self.0 >> 10usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "Base Physical Address (DBPA)"]
        #[inline(always)]
        pub const fn set_dbpa(&mut self, val: u32) {
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
    impl core::fmt::Debug for Mdlmb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mdlmb")
                .field("den", &self.den())
                .field("eccen", &self.eccen())
                .field("rwecc", &self.rwecc())
                .field("reserved", &self.reserved())
                .field("dbpa", &self.dbpa())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mdlmb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mdlmb {{ den: {=bool:?}, eccen: {=u8:?}, rwecc: {=bool:?}, reserved: {=u8:?}, dbpa: {=u32:?} }}",
                self.den(),
                self.eccen(),
                self.rwecc(),
                self.reserved(),
                self.dbpa()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7c1, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7c1, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7c1, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7c1, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Mdlmb {
        unsafe { Mdlmb(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Mdlmb) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Mdlmb) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "DLM Enable"]
    #[inline]
    pub unsafe fn set_den() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "DLM Enable"]
    #[inline]
    pub unsafe fn clear_den() {
        unsafe {
            _clear(1usize);
        }
    }
    #[doc = "ECC Enable"]
    #[inline]
    pub unsafe fn set_eccen(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(3usize << 1usize);
        bits |= (val as usize & 3usize) << 1usize;
        unsafe {
            _write(bits);
        }
    }
    #[doc = "Read/Write ECC"]
    #[inline]
    pub unsafe fn set_rwecc() {
        unsafe {
            _set(8usize);
        }
    }
    #[doc = "Read/Write ECC"]
    #[inline]
    pub unsafe fn clear_rwecc() {
        unsafe {
            _clear(8usize);
        }
    }
    #[doc = "Reserved"]
    #[inline]
    pub unsafe fn set_reserved(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(63usize << 4usize);
        bits |= (val as usize & 63usize) << 4usize;
        unsafe {
            _write(bits);
        }
    }
    #[doc = "Base Physical Address (DBPA)"]
    #[inline]
    pub unsafe fn set_dbpa(val: u32) {
        let mut bits = unsafe { _read() };
        bits &= !(4194303usize << 10usize);
        bits |= (val as usize & 4194303usize) << 10usize;
        unsafe {
            _write(bits);
        }
    }
}
#[doc = "ECC code."]
pub mod mecc_code {
    #[doc = "ECC Code Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MeccCode(pub u32);
    impl MeccCode {
        #[doc = "ECC Code"]
        #[must_use]
        #[inline(always)]
        pub const fn code(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "ECC Code"]
        #[inline(always)]
        pub const fn set_code(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Correctable Error Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn c(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Correctable Error Flag"]
        #[inline(always)]
        pub const fn set_c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Parity Error Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn p(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Parity Error Flag"]
        #[inline(always)]
        pub const fn set_p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "RAM Identifier"]
        #[must_use]
        #[inline(always)]
        pub const fn ramid(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[doc = "RAM Identifier"]
        #[inline(always)]
        pub const fn set_ramid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
        #[doc = "Instruction Error Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn insn(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction Error Flag"]
        #[inline(always)]
        pub const fn set_insn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Syndrome"]
        #[must_use]
        #[inline(always)]
        pub const fn syndr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Syndrome"]
        #[inline(always)]
        pub const fn set_syndr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for MeccCode {
        #[inline(always)]
        fn default() -> MeccCode {
            MeccCode(0)
        }
    }
    impl core::fmt::Debug for MeccCode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MeccCode")
                .field("code", &self.code())
                .field("c", &self.c())
                .field("p", &self.p())
                .field("ramid", &self.ramid())
                .field("insn", &self.insn())
                .field("syndr", &self.syndr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MeccCode {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MeccCode {{ code: {=u8:?}, c: {=bool:?}, p: {=bool:?}, ramid: {=u8:?}, insn: {=bool:?}, syndr: {=bool:?} }}",
                self.code(),
                self.c(),
                self.p(),
                self.ramid(),
                self.insn(),
                self.syndr()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7c2, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7c2, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7c2, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7c2, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> MeccCode {
        unsafe { MeccCode(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: MeccCode) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut MeccCode) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "ECC Code"]
    #[inline]
    pub unsafe fn set_code(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(127usize << 0usize);
        bits |= (val as usize & 127usize) << 0usize;
        unsafe {
            _write(bits);
        }
    }
    #[doc = "Correctable Error Flag"]
    #[inline]
    pub unsafe fn set_c() {
        unsafe {
            _set(65536usize);
        }
    }
    #[doc = "Correctable Error Flag"]
    #[inline]
    pub unsafe fn clear_c() {
        unsafe {
            _clear(65536usize);
        }
    }
    #[doc = "Parity Error Flag"]
    #[inline]
    pub unsafe fn set_p() {
        unsafe {
            _set(131072usize);
        }
    }
    #[doc = "Parity Error Flag"]
    #[inline]
    pub unsafe fn clear_p() {
        unsafe {
            _clear(131072usize);
        }
    }
    #[doc = "RAM Identifier"]
    #[inline]
    pub unsafe fn set_ramid(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(15usize << 18usize);
        bits |= (val as usize & 15usize) << 18usize;
        unsafe {
            _write(bits);
        }
    }
    #[doc = "Instruction Error Flag"]
    #[inline]
    pub unsafe fn set_insn() {
        unsafe {
            _set(4194304usize);
        }
    }
    #[doc = "Instruction Error Flag"]
    #[inline]
    pub unsafe fn clear_insn() {
        unsafe {
            _clear(4194304usize);
        }
    }
    #[doc = "Syndrome"]
    #[inline]
    pub unsafe fn set_syndr() {
        unsafe {
            _set(8388608usize);
        }
    }
    #[doc = "Syndrome"]
    #[inline]
    pub unsafe fn clear_syndr() {
        unsafe {
            _clear(8388608usize);
        }
    }
}
#[doc = "NMI-handler base address."]
pub mod mnvec {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7c3, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7c3, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7c3, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7c3, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "Additional machine mode status"]
pub mod mxstatus {
    #[doc = "Machine Extended Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mxstatus(pub u32);
    impl Mxstatus {
        #[must_use]
        #[inline(always)]
        pub const fn pft_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pft_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ppft_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ppft_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ime(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ime(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pime(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pime(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dme(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pdme(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pdme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn typ(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_typ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ptyp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Mxstatus {
        #[inline(always)]
        fn default() -> Mxstatus {
            Mxstatus(0)
        }
    }
    impl core::fmt::Debug for Mxstatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mxstatus")
                .field("pft_en", &self.pft_en())
                .field("ppft_en", &self.ppft_en())
                .field("ime", &self.ime())
                .field("pime", &self.pime())
                .field("dme", &self.dme())
                .field("pdme", &self.pdme())
                .field("typ", &self.typ())
                .field("ptyp", &self.ptyp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mxstatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mxstatus {{ pft_en: {=bool:?}, ppft_en: {=bool:?}, ime: {=bool:?}, pime: {=bool:?}, dme: {=bool:?}, pdme: {=bool:?}, typ: {=u8:?}, ptyp: {=u8:?} }}",
                self.pft_en(),
                self.ppft_en(),
                self.ime(),
                self.pime(),
                self.dme(),
                self.pdme(),
                self.typ(),
                self.ptyp()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7c4, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7c4, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7c4, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7c4, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Mxstatus {
        unsafe { Mxstatus(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Mxstatus) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Mxstatus) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[inline]
    pub unsafe fn set_pft_en() {
        unsafe {
            _set(1usize);
        }
    }
    #[inline]
    pub unsafe fn clear_pft_en() {
        unsafe {
            _clear(1usize);
        }
    }
    #[inline]
    pub unsafe fn set_ppft_en() {
        unsafe {
            _set(2usize);
        }
    }
    #[inline]
    pub unsafe fn clear_ppft_en() {
        unsafe {
            _clear(2usize);
        }
    }
    #[inline]
    pub unsafe fn set_ime() {
        unsafe {
            _set(4usize);
        }
    }
    #[inline]
    pub unsafe fn clear_ime() {
        unsafe {
            _clear(4usize);
        }
    }
    #[inline]
    pub unsafe fn set_pime() {
        unsafe {
            _set(8usize);
        }
    }
    #[inline]
    pub unsafe fn clear_pime() {
        unsafe {
            _clear(8usize);
        }
    }
    #[inline]
    pub unsafe fn set_dme() {
        unsafe {
            _set(16usize);
        }
    }
    #[inline]
    pub unsafe fn clear_dme() {
        unsafe {
            _clear(16usize);
        }
    }
    #[inline]
    pub unsafe fn set_pdme() {
        unsafe {
            _set(32usize);
        }
    }
    #[inline]
    pub unsafe fn clear_pdme() {
        unsafe {
            _clear(32usize);
        }
    }
    #[inline]
    pub unsafe fn set_typ(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(3usize << 6usize);
        bits |= (val as usize & 3usize) << 6usize;
        unsafe {
            _write(bits);
        }
    }
    #[inline]
    pub unsafe fn set_ptyp(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(3usize << 8usize);
        bits |= (val as usize & 3usize) << 8usize;
        unsafe {
            _write(bits);
        }
    }
}
#[doc = "Performance throttling control"]
pub mod mpft_ctl {
    #[doc = "Performance Throttling Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MpftCtl(pub u32);
    impl MpftCtl {
        #[doc = "Throttling Level"]
        #[must_use]
        #[inline(always)]
        pub const fn t_level(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Throttling Level"]
        #[inline(always)]
        pub const fn set_t_level(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Fast Interrupt"]
        #[must_use]
        #[inline(always)]
        pub const fn fast_int(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Fast Interrupt"]
        #[inline(always)]
        pub const fn set_fast_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for MpftCtl {
        #[inline(always)]
        fn default() -> MpftCtl {
            MpftCtl(0)
        }
    }
    impl core::fmt::Debug for MpftCtl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MpftCtl")
                .field("t_level", &self.t_level())
                .field("fast_int", &self.fast_int())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MpftCtl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MpftCtl {{ t_level: {=u8:?}, fast_int: {=bool:?} }}",
                self.t_level(),
                self.fast_int()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7c5, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7c5, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7c5, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7c5, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> MpftCtl {
        unsafe { MpftCtl(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: MpftCtl) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut MpftCtl) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Throttling Level"]
    #[inline]
    pub unsafe fn set_t_level(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(15usize << 4usize);
        bits |= (val as usize & 15usize) << 4usize;
        unsafe {
            _write(bits);
        }
    }
    #[doc = "Fast Interrupt"]
    #[inline]
    pub unsafe fn set_fast_int() {
        unsafe {
            _set(256usize);
        }
    }
    #[doc = "Fast Interrupt"]
    #[inline]
    pub unsafe fn clear_fast_int() {
        unsafe {
            _clear(256usize);
        }
    }
}
#[doc = "Hardware stack protection control"]
pub mod mhsp_ctl {
    #[doc = "Machine Hardware Stack Protection Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MhspCtl(pub u32);
    impl MhspCtl {
        #[must_use]
        #[inline(always)]
        pub const fn ovf_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ovf_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn udf_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_udf_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn schm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_schm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn u(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_u(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn s(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn m(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_m(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for MhspCtl {
        #[inline(always)]
        fn default() -> MhspCtl {
            MhspCtl(0)
        }
    }
    impl core::fmt::Debug for MhspCtl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MhspCtl")
                .field("ovf_en", &self.ovf_en())
                .field("udf_en", &self.udf_en())
                .field("schm", &self.schm())
                .field("u", &self.u())
                .field("s", &self.s())
                .field("m", &self.m())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MhspCtl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MhspCtl {{ ovf_en: {=bool:?}, udf_en: {=bool:?}, schm: {=bool:?}, u: {=bool:?}, s: {=bool:?}, m: {=bool:?} }}",
                self.ovf_en(),
                self.udf_en(),
                self.schm(),
                self.u(),
                self.s(),
                self.m()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7c6, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7c6, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7c6, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7c6, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> MhspCtl {
        unsafe { MhspCtl(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: MhspCtl) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut MhspCtl) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[inline]
    pub unsafe fn set_ovf_en() {
        unsafe {
            _set(1usize);
        }
    }
    #[inline]
    pub unsafe fn clear_ovf_en() {
        unsafe {
            _clear(1usize);
        }
    }
    #[inline]
    pub unsafe fn set_udf_en() {
        unsafe {
            _set(2usize);
        }
    }
    #[inline]
    pub unsafe fn clear_udf_en() {
        unsafe {
            _clear(2usize);
        }
    }
    #[inline]
    pub unsafe fn set_schm() {
        unsafe {
            _set(4usize);
        }
    }
    #[inline]
    pub unsafe fn clear_schm() {
        unsafe {
            _clear(4usize);
        }
    }
    #[inline]
    pub unsafe fn set_u() {
        unsafe {
            _set(8usize);
        }
    }
    #[inline]
    pub unsafe fn clear_u() {
        unsafe {
            _clear(8usize);
        }
    }
    #[inline]
    pub unsafe fn set_s() {
        unsafe {
            _set(16usize);
        }
    }
    #[inline]
    pub unsafe fn clear_s() {
        unsafe {
            _clear(16usize);
        }
    }
    #[inline]
    pub unsafe fn set_m() {
        unsafe {
            _set(32usize);
        }
    }
    #[inline]
    pub unsafe fn clear_m() {
        unsafe {
            _clear(32usize);
        }
    }
}
#[doc = "SP bound register"]
pub mod msp_bound {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7c7, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7c7, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7c7, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7c7, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "SP base register"]
pub mod msp_base {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7c8, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7c8, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7c8, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7c8, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "Detailed exception cause"]
pub mod mdcause {
    #[doc = "Machine Detailed Trap Cause Register (for imprecise exception/interrupt)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdcause(pub u32);
    impl Mdcause {
        #[must_use]
        #[inline(always)]
        pub const fn mdcause(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mdcause(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pm(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_pm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
    }
    impl Default for Mdcause {
        #[inline(always)]
        fn default() -> Mdcause {
            Mdcause(0)
        }
    }
    impl core::fmt::Debug for Mdcause {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mdcause")
                .field("mdcause", &self.mdcause())
                .field("pm", &self.pm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mdcause {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mdcause {{ mdcause: {=u8:?}, pm: {=u8:?} }}",
                self.mdcause(),
                self.pm()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7c9, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7c9, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7c9, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7c9, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Mdcause {
        unsafe { Mdcause(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Mdcause) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Mdcause) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[inline]
    pub unsafe fn set_mdcause(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(31usize << 0usize);
        bits |= (val as usize & 31usize) << 0usize;
        unsafe {
            _write(bits);
        }
    }
    #[inline]
    pub unsafe fn set_pm(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(3usize << 5usize);
        bits |= (val as usize & 3usize) << 5usize;
        unsafe {
            _write(bits);
        }
    }
}
#[doc = "Cache control"]
pub mod mcache_ctl {
    #[doc = "Cache Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McacheCtl(pub u32);
    impl McacheCtl {
        #[doc = "Instruction Cache Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ic_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction Cache Enable"]
        #[inline(always)]
        pub const fn set_ic_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data Cache Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dc_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data Cache Enable"]
        #[inline(always)]
        pub const fn set_dc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Instruction Cache ECC Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ic_eccen(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Instruction Cache ECC Enable"]
        #[inline(always)]
        pub const fn set_ic_eccen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Data Cache ECC Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dc_eccen(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Data Cache ECC Enable"]
        #[inline(always)]
        pub const fn set_dc_eccen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Instruction Cache Read/Write ECC"]
        #[must_use]
        #[inline(always)]
        pub const fn ic_rwecc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction Cache Read/Write ECC"]
        #[inline(always)]
        pub const fn set_ic_rwecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Data Cache Read/Write ECC"]
        #[must_use]
        #[inline(always)]
        pub const fn dc_rwecc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Data Cache Read/Write ECC"]
        #[inline(always)]
        pub const fn set_dc_rwecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Cache Control SU Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn cctl_suen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Cache Control SU Enable"]
        #[inline(always)]
        pub const fn set_cctl_suen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Instruction Prefetch Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ipref_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction Prefetch Enable"]
        #[inline(always)]
        pub const fn set_ipref_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Data Prefetch Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dpref_en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Data Prefetch Enable"]
        #[inline(always)]
        pub const fn set_dpref_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Instruction Cache 1st Way Disable"]
        #[must_use]
        #[inline(always)]
        pub const fn ic_1st_wd(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction Cache 1st Way Disable"]
        #[inline(always)]
        pub const fn set_ic_1st_wd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Data Cache 1st Way Disable"]
        #[must_use]
        #[inline(always)]
        pub const fn dc_1st_wd(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Data Cache 1st Way Disable"]
        #[inline(always)]
        pub const fn set_dc_1st_wd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Data Cache Write-Around"]
        #[must_use]
        #[inline(always)]
        pub const fn dc_warnd(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "Data Cache Write-Around"]
        #[inline(always)]
        pub const fn set_dc_warnd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "L2 Cache Write-Around"]
        #[must_use]
        #[inline(always)]
        pub const fn l2c_warnd(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x03;
            val as u8
        }
        #[doc = "L2 Cache Write-Around"]
        #[inline(always)]
        pub const fn set_l2c_warnd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 15usize)) | (((val as u32) & 0x03) << 15usize);
        }
        #[doc = "TLB ECC Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tlb_eccen(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "TLB ECC Enable"]
        #[inline(always)]
        pub const fn set_tlb_eccen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "Data Cache Coherence Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dc_cohen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Data Cache Coherence Enable"]
        #[inline(always)]
        pub const fn set_dc_cohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Data Cache Coherence State"]
        #[must_use]
        #[inline(always)]
        pub const fn dc_cohsta(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Data Cache Coherence State"]
        #[inline(always)]
        pub const fn set_dc_cohsta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Data Prefetch Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dpref_mode(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[doc = "Data Prefetch Mode"]
        #[inline(always)]
        pub const fn set_dpref_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
    }
    impl Default for McacheCtl {
        #[inline(always)]
        fn default() -> McacheCtl {
            McacheCtl(0)
        }
    }
    impl core::fmt::Debug for McacheCtl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("McacheCtl")
                .field("ic_en", &self.ic_en())
                .field("dc_en", &self.dc_en())
                .field("ic_eccen", &self.ic_eccen())
                .field("dc_eccen", &self.dc_eccen())
                .field("ic_rwecc", &self.ic_rwecc())
                .field("dc_rwecc", &self.dc_rwecc())
                .field("cctl_suen", &self.cctl_suen())
                .field("ipref_en", &self.ipref_en())
                .field("dpref_en", &self.dpref_en())
                .field("ic_1st_wd", &self.ic_1st_wd())
                .field("dc_1st_wd", &self.dc_1st_wd())
                .field("dc_warnd", &self.dc_warnd())
                .field("l2c_warnd", &self.l2c_warnd())
                .field("tlb_eccen", &self.tlb_eccen())
                .field("dc_cohen", &self.dc_cohen())
                .field("dc_cohsta", &self.dc_cohsta())
                .field("dpref_mode", &self.dpref_mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for McacheCtl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "McacheCtl {{ ic_en: {=bool:?}, dc_en: {=bool:?}, ic_eccen: {=u8:?}, dc_eccen: {=u8:?}, ic_rwecc: {=bool:?}, dc_rwecc: {=bool:?}, cctl_suen: {=bool:?}, ipref_en: {=bool:?}, dpref_en: {=bool:?}, ic_1st_wd: {=bool:?}, dc_1st_wd: {=bool:?}, dc_warnd: {=u8:?}, l2c_warnd: {=u8:?}, tlb_eccen: {=u8:?}, dc_cohen: {=bool:?}, dc_cohsta: {=bool:?}, dpref_mode: {=u8:?} }}",
                self.ic_en(),
                self.dc_en(),
                self.ic_eccen(),
                self.dc_eccen(),
                self.ic_rwecc(),
                self.dc_rwecc(),
                self.cctl_suen(),
                self.ipref_en(),
                self.dpref_en(),
                self.ic_1st_wd(),
                self.dc_1st_wd(),
                self.dc_warnd(),
                self.l2c_warnd(),
                self.tlb_eccen(),
                self.dc_cohen(),
                self.dc_cohsta(),
                self.dpref_mode()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7ca, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7ca, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7ca, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7ca, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> McacheCtl {
        unsafe { McacheCtl(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: McacheCtl) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut McacheCtl) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Instruction Cache Enable"]
    #[inline]
    pub unsafe fn set_ic_en() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Instruction Cache Enable"]
    #[inline]
    pub unsafe fn clear_ic_en() {
        unsafe {
            _clear(1usize);
        }
    }
    #[doc = "Data Cache Enable"]
    #[inline]
    pub unsafe fn set_dc_en() {
        unsafe {
            _set(2usize);
        }
    }
    #[doc = "Data Cache Enable"]
    #[inline]
    pub unsafe fn clear_dc_en() {
        unsafe {
            _clear(2usize);
        }
    }
    #[doc = "Instruction Cache ECC Enable"]
    #[inline]
    pub unsafe fn set_ic_eccen(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(3usize << 2usize);
        bits |= (val as usize & 3usize) << 2usize;
        unsafe {
            _write(bits);
        }
    }
    #[doc = "Data Cache ECC Enable"]
    #[inline]
    pub unsafe fn set_dc_eccen(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(3usize << 4usize);
        bits |= (val as usize & 3usize) << 4usize;
        unsafe {
            _write(bits);
        }
    }
    #[doc = "Instruction Cache Read/Write ECC"]
    #[inline]
    pub unsafe fn set_ic_rwecc() {
        unsafe {
            _set(64usize);
        }
    }
    #[doc = "Instruction Cache Read/Write ECC"]
    #[inline]
    pub unsafe fn clear_ic_rwecc() {
        unsafe {
            _clear(64usize);
        }
    }
    #[doc = "Data Cache Read/Write ECC"]
    #[inline]
    pub unsafe fn set_dc_rwecc() {
        unsafe {
            _set(128usize);
        }
    }
    #[doc = "Data Cache Read/Write ECC"]
    #[inline]
    pub unsafe fn clear_dc_rwecc() {
        unsafe {
            _clear(128usize);
        }
    }
    #[doc = "Cache Control SU Enable"]
    #[inline]
    pub unsafe fn set_cctl_suen() {
        unsafe {
            _set(256usize);
        }
    }
    #[doc = "Cache Control SU Enable"]
    #[inline]
    pub unsafe fn clear_cctl_suen() {
        unsafe {
            _clear(256usize);
        }
    }
    #[doc = "Instruction Prefetch Enable"]
    #[inline]
    pub unsafe fn set_ipref_en() {
        unsafe {
            _set(512usize);
        }
    }
    #[doc = "Instruction Prefetch Enable"]
    #[inline]
    pub unsafe fn clear_ipref_en() {
        unsafe {
            _clear(512usize);
        }
    }
    #[doc = "Data Prefetch Enable"]
    #[inline]
    pub unsafe fn set_dpref_en() {
        unsafe {
            _set(1024usize);
        }
    }
    #[doc = "Data Prefetch Enable"]
    #[inline]
    pub unsafe fn clear_dpref_en() {
        unsafe {
            _clear(1024usize);
        }
    }
    #[doc = "Instruction Cache 1st Way Disable"]
    #[inline]
    pub unsafe fn set_ic_1st_wd() {
        unsafe {
            _set(2048usize);
        }
    }
    #[doc = "Instruction Cache 1st Way Disable"]
    #[inline]
    pub unsafe fn clear_ic_1st_wd() {
        unsafe {
            _clear(2048usize);
        }
    }
    #[doc = "Data Cache 1st Way Disable"]
    #[inline]
    pub unsafe fn set_dc_1st_wd() {
        unsafe {
            _set(4096usize);
        }
    }
    #[doc = "Data Cache 1st Way Disable"]
    #[inline]
    pub unsafe fn clear_dc_1st_wd() {
        unsafe {
            _clear(4096usize);
        }
    }
    #[doc = "Data Cache Write-Around"]
    #[inline]
    pub unsafe fn set_dc_warnd(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(3usize << 13usize);
        bits |= (val as usize & 3usize) << 13usize;
        unsafe {
            _write(bits);
        }
    }
    #[doc = "L2 Cache Write-Around"]
    #[inline]
    pub unsafe fn set_l2c_warnd(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(3usize << 15usize);
        bits |= (val as usize & 3usize) << 15usize;
        unsafe {
            _write(bits);
        }
    }
    #[doc = "TLB ECC Enable"]
    #[inline]
    pub unsafe fn set_tlb_eccen(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(3usize << 17usize);
        bits |= (val as usize & 3usize) << 17usize;
        unsafe {
            _write(bits);
        }
    }
    #[doc = "Data Cache Coherence Enable"]
    #[inline]
    pub unsafe fn set_dc_cohen() {
        unsafe {
            _set(524288usize);
        }
    }
    #[doc = "Data Cache Coherence Enable"]
    #[inline]
    pub unsafe fn clear_dc_cohen() {
        unsafe {
            _clear(524288usize);
        }
    }
    #[doc = "Data Cache Coherence State"]
    #[inline]
    pub unsafe fn set_dc_cohsta() {
        unsafe {
            _set(1048576usize);
        }
    }
    #[doc = "Data Cache Coherence State"]
    #[inline]
    pub unsafe fn clear_dc_cohsta() {
        unsafe {
            _clear(1048576usize);
        }
    }
    #[doc = "Data Prefetch Mode"]
    #[inline]
    pub unsafe fn set_dpref_mode(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(3usize << 21usize);
        bits |= (val as usize & 3usize) << 21usize;
        unsafe {
            _write(bits);
        }
    }
}
#[doc = "CCTL begin address"]
pub mod mcctlbeginaddr {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7cb, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7cb, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7cb, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7cb, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "CCTL command"]
pub mod mcctlcommand {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7cc, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7cc, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7cc, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7cc, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "CCTL data"]
pub mod mcctldata {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7cd, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7cd, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7cd, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7cd, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "Counter write enable"]
pub mod mcounterwen {
    #[doc = "Machine Counter Fields"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McounterCommon(pub u32);
    impl McounterCommon {
        #[doc = "Cycle counter"]
        #[must_use]
        #[inline(always)]
        pub const fn cy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cycle counter"]
        #[inline(always)]
        pub const fn set_cy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Instruction retired counter"]
        #[must_use]
        #[inline(always)]
        pub const fn ir(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction retired counter"]
        #[inline(always)]
        pub const fn set_ir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[must_use]
        #[inline(always)]
        pub const fn hpm(&self, n: usize) -> bool {
            assert!(n < 29usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[inline(always)]
        pub const fn set_hpm(&mut self, n: usize, val: bool) {
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
    impl core::fmt::Debug for McounterCommon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("McounterCommon")
                .field("cy", &self.cy())
                .field("ir", &self.ir())
                .field("hpm[0]", &self.hpm(0usize))
                .field("hpm[1]", &self.hpm(1usize))
                .field("hpm[2]", &self.hpm(2usize))
                .field("hpm[3]", &self.hpm(3usize))
                .field("hpm[4]", &self.hpm(4usize))
                .field("hpm[5]", &self.hpm(5usize))
                .field("hpm[6]", &self.hpm(6usize))
                .field("hpm[7]", &self.hpm(7usize))
                .field("hpm[8]", &self.hpm(8usize))
                .field("hpm[9]", &self.hpm(9usize))
                .field("hpm[10]", &self.hpm(10usize))
                .field("hpm[11]", &self.hpm(11usize))
                .field("hpm[12]", &self.hpm(12usize))
                .field("hpm[13]", &self.hpm(13usize))
                .field("hpm[14]", &self.hpm(14usize))
                .field("hpm[15]", &self.hpm(15usize))
                .field("hpm[16]", &self.hpm(16usize))
                .field("hpm[17]", &self.hpm(17usize))
                .field("hpm[18]", &self.hpm(18usize))
                .field("hpm[19]", &self.hpm(19usize))
                .field("hpm[20]", &self.hpm(20usize))
                .field("hpm[21]", &self.hpm(21usize))
                .field("hpm[22]", &self.hpm(22usize))
                .field("hpm[23]", &self.hpm(23usize))
                .field("hpm[24]", &self.hpm(24usize))
                .field("hpm[25]", &self.hpm(25usize))
                .field("hpm[26]", &self.hpm(26usize))
                .field("hpm[27]", &self.hpm(27usize))
                .field("hpm[28]", &self.hpm(28usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for McounterCommon {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "McounterCommon {{ cy: {=bool:?}, ir: {=bool:?}, hpm[0]: {=bool:?}, hpm[1]: {=bool:?}, hpm[2]: {=bool:?}, hpm[3]: {=bool:?}, hpm[4]: {=bool:?}, hpm[5]: {=bool:?}, hpm[6]: {=bool:?}, hpm[7]: {=bool:?}, hpm[8]: {=bool:?}, hpm[9]: {=bool:?}, hpm[10]: {=bool:?}, hpm[11]: {=bool:?}, hpm[12]: {=bool:?}, hpm[13]: {=bool:?}, hpm[14]: {=bool:?}, hpm[15]: {=bool:?}, hpm[16]: {=bool:?}, hpm[17]: {=bool:?}, hpm[18]: {=bool:?}, hpm[19]: {=bool:?}, hpm[20]: {=bool:?}, hpm[21]: {=bool:?}, hpm[22]: {=bool:?}, hpm[23]: {=bool:?}, hpm[24]: {=bool:?}, hpm[25]: {=bool:?}, hpm[26]: {=bool:?}, hpm[27]: {=bool:?}, hpm[28]: {=bool:?} }}",
                self.cy(),
                self.ir(),
                self.hpm(0usize),
                self.hpm(1usize),
                self.hpm(2usize),
                self.hpm(3usize),
                self.hpm(4usize),
                self.hpm(5usize),
                self.hpm(6usize),
                self.hpm(7usize),
                self.hpm(8usize),
                self.hpm(9usize),
                self.hpm(10usize),
                self.hpm(11usize),
                self.hpm(12usize),
                self.hpm(13usize),
                self.hpm(14usize),
                self.hpm(15usize),
                self.hpm(16usize),
                self.hpm(17usize),
                self.hpm(18usize),
                self.hpm(19usize),
                self.hpm(20usize),
                self.hpm(21usize),
                self.hpm(22usize),
                self.hpm(23usize),
                self.hpm(24usize),
                self.hpm(25usize),
                self.hpm(26usize),
                self.hpm(27usize),
                self.hpm(28usize)
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7ce, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7ce, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7ce, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7ce, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> McounterCommon {
        unsafe { McounterCommon(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: McounterCommon) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut McounterCommon) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn set_cy() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn clear_cy() {
        unsafe {
            _clear(1usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn set_ir() {
        unsafe {
            _set(4usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn clear_ir() {
        unsafe {
            _clear(4usize);
        }
    }
}
#[doc = "Counter overflow interrupt enable"]
pub mod mcounterinten {
    #[doc = "Machine Counter Fields"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McounterCommon(pub u32);
    impl McounterCommon {
        #[doc = "Cycle counter"]
        #[must_use]
        #[inline(always)]
        pub const fn cy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cycle counter"]
        #[inline(always)]
        pub const fn set_cy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Instruction retired counter"]
        #[must_use]
        #[inline(always)]
        pub const fn ir(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction retired counter"]
        #[inline(always)]
        pub const fn set_ir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[must_use]
        #[inline(always)]
        pub const fn hpm(&self, n: usize) -> bool {
            assert!(n < 29usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[inline(always)]
        pub const fn set_hpm(&mut self, n: usize, val: bool) {
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
    impl core::fmt::Debug for McounterCommon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("McounterCommon")
                .field("cy", &self.cy())
                .field("ir", &self.ir())
                .field("hpm[0]", &self.hpm(0usize))
                .field("hpm[1]", &self.hpm(1usize))
                .field("hpm[2]", &self.hpm(2usize))
                .field("hpm[3]", &self.hpm(3usize))
                .field("hpm[4]", &self.hpm(4usize))
                .field("hpm[5]", &self.hpm(5usize))
                .field("hpm[6]", &self.hpm(6usize))
                .field("hpm[7]", &self.hpm(7usize))
                .field("hpm[8]", &self.hpm(8usize))
                .field("hpm[9]", &self.hpm(9usize))
                .field("hpm[10]", &self.hpm(10usize))
                .field("hpm[11]", &self.hpm(11usize))
                .field("hpm[12]", &self.hpm(12usize))
                .field("hpm[13]", &self.hpm(13usize))
                .field("hpm[14]", &self.hpm(14usize))
                .field("hpm[15]", &self.hpm(15usize))
                .field("hpm[16]", &self.hpm(16usize))
                .field("hpm[17]", &self.hpm(17usize))
                .field("hpm[18]", &self.hpm(18usize))
                .field("hpm[19]", &self.hpm(19usize))
                .field("hpm[20]", &self.hpm(20usize))
                .field("hpm[21]", &self.hpm(21usize))
                .field("hpm[22]", &self.hpm(22usize))
                .field("hpm[23]", &self.hpm(23usize))
                .field("hpm[24]", &self.hpm(24usize))
                .field("hpm[25]", &self.hpm(25usize))
                .field("hpm[26]", &self.hpm(26usize))
                .field("hpm[27]", &self.hpm(27usize))
                .field("hpm[28]", &self.hpm(28usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for McounterCommon {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "McounterCommon {{ cy: {=bool:?}, ir: {=bool:?}, hpm[0]: {=bool:?}, hpm[1]: {=bool:?}, hpm[2]: {=bool:?}, hpm[3]: {=bool:?}, hpm[4]: {=bool:?}, hpm[5]: {=bool:?}, hpm[6]: {=bool:?}, hpm[7]: {=bool:?}, hpm[8]: {=bool:?}, hpm[9]: {=bool:?}, hpm[10]: {=bool:?}, hpm[11]: {=bool:?}, hpm[12]: {=bool:?}, hpm[13]: {=bool:?}, hpm[14]: {=bool:?}, hpm[15]: {=bool:?}, hpm[16]: {=bool:?}, hpm[17]: {=bool:?}, hpm[18]: {=bool:?}, hpm[19]: {=bool:?}, hpm[20]: {=bool:?}, hpm[21]: {=bool:?}, hpm[22]: {=bool:?}, hpm[23]: {=bool:?}, hpm[24]: {=bool:?}, hpm[25]: {=bool:?}, hpm[26]: {=bool:?}, hpm[27]: {=bool:?}, hpm[28]: {=bool:?} }}",
                self.cy(),
                self.ir(),
                self.hpm(0usize),
                self.hpm(1usize),
                self.hpm(2usize),
                self.hpm(3usize),
                self.hpm(4usize),
                self.hpm(5usize),
                self.hpm(6usize),
                self.hpm(7usize),
                self.hpm(8usize),
                self.hpm(9usize),
                self.hpm(10usize),
                self.hpm(11usize),
                self.hpm(12usize),
                self.hpm(13usize),
                self.hpm(14usize),
                self.hpm(15usize),
                self.hpm(16usize),
                self.hpm(17usize),
                self.hpm(18usize),
                self.hpm(19usize),
                self.hpm(20usize),
                self.hpm(21usize),
                self.hpm(22usize),
                self.hpm(23usize),
                self.hpm(24usize),
                self.hpm(25usize),
                self.hpm(26usize),
                self.hpm(27usize),
                self.hpm(28usize)
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7cf, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7cf, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7cf, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7cf, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> McounterCommon {
        unsafe { McounterCommon(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: McounterCommon) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut McounterCommon) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn set_cy() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn clear_cy() {
        unsafe {
            _clear(1usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn set_ir() {
        unsafe {
            _set(4usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn clear_ir() {
        unsafe {
            _clear(4usize);
        }
    }
}
#[doc = "Miscellaneous control"]
pub mod mmisc_ctl {
    #[doc = "Machine Miscellaneous Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmiscCtl(pub u32);
    impl MmiscCtl {
        #[doc = "Andes Custom Extension (ACE) enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ace(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Andes Custom Extension (ACE) enable"]
        #[inline(always)]
        pub const fn set_ace(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Vectored external PLIC interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn vec_plic(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Vectored external PLIC interrupt enable"]
        #[inline(always)]
        pub const fn set_vec_plic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RV compatibility mode enable bit"]
        #[must_use]
        #[inline(always)]
        pub const fn rvcompm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RV compatibility mode enable bit"]
        #[inline(always)]
        pub const fn set_rvcompm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn brpe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_brpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aces(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_aces(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn msa_una(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_msa_una(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nbld_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_nbld_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn newnmi(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_newnmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vcgl1_en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vcgl1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vcgl2_en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vcgl2_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vcgl3_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_vcgl3_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "“Load to x0” exception generation control bit"]
        #[must_use]
        #[inline(always)]
        pub const fn ldx0nxp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "“Load to x0” exception generation control bit"]
        #[inline(always)]
        pub const fn set_ldx0nxp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for MmiscCtl {
        #[inline(always)]
        fn default() -> MmiscCtl {
            MmiscCtl(0)
        }
    }
    impl core::fmt::Debug for MmiscCtl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmiscCtl")
                .field("ace", &self.ace())
                .field("vec_plic", &self.vec_plic())
                .field("rvcompm", &self.rvcompm())
                .field("brpe", &self.brpe())
                .field("aces", &self.aces())
                .field("msa_una", &self.msa_una())
                .field("nbld_en", &self.nbld_en())
                .field("newnmi", &self.newnmi())
                .field("vcgl1_en", &self.vcgl1_en())
                .field("vcgl2_en", &self.vcgl2_en())
                .field("vcgl3_en", &self.vcgl3_en())
                .field("ldx0nxp", &self.ldx0nxp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmiscCtl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MmiscCtl {{ ace: {=bool:?}, vec_plic: {=bool:?}, rvcompm: {=bool:?}, brpe: {=bool:?}, aces: {=u8:?}, msa_una: {=bool:?}, nbld_en: {=bool:?}, newnmi: {=bool:?}, vcgl1_en: {=bool:?}, vcgl2_en: {=bool:?}, vcgl3_en: {=bool:?}, ldx0nxp: {=bool:?} }}",
                self.ace(),
                self.vec_plic(),
                self.rvcompm(),
                self.brpe(),
                self.aces(),
                self.msa_una(),
                self.nbld_en(),
                self.newnmi(),
                self.vcgl1_en(),
                self.vcgl2_en(),
                self.vcgl3_en(),
                self.ldx0nxp()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7d0, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7d0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7d0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7d0, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> MmiscCtl {
        unsafe { MmiscCtl(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: MmiscCtl) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut MmiscCtl) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Andes Custom Extension (ACE) enable"]
    #[inline]
    pub unsafe fn set_ace() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Andes Custom Extension (ACE) enable"]
    #[inline]
    pub unsafe fn clear_ace() {
        unsafe {
            _clear(1usize);
        }
    }
    #[doc = "Vectored external PLIC interrupt enable"]
    #[inline]
    pub unsafe fn set_vec_plic() {
        unsafe {
            _set(2usize);
        }
    }
    #[doc = "Vectored external PLIC interrupt enable"]
    #[inline]
    pub unsafe fn clear_vec_plic() {
        unsafe {
            _clear(2usize);
        }
    }
    #[doc = "RV compatibility mode enable bit"]
    #[inline]
    pub unsafe fn set_rvcompm() {
        unsafe {
            _set(4usize);
        }
    }
    #[doc = "RV compatibility mode enable bit"]
    #[inline]
    pub unsafe fn clear_rvcompm() {
        unsafe {
            _clear(4usize);
        }
    }
    #[inline]
    pub unsafe fn set_brpe() {
        unsafe {
            _set(8usize);
        }
    }
    #[inline]
    pub unsafe fn clear_brpe() {
        unsafe {
            _clear(8usize);
        }
    }
    #[inline]
    pub unsafe fn set_aces(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(3usize << 4usize);
        bits |= (val as usize & 3usize) << 4usize;
        unsafe {
            _write(bits);
        }
    }
    #[inline]
    pub unsafe fn set_msa_una() {
        unsafe {
            _set(64usize);
        }
    }
    #[inline]
    pub unsafe fn clear_msa_una() {
        unsafe {
            _clear(64usize);
        }
    }
    #[inline]
    pub unsafe fn set_nbld_en() {
        unsafe {
            _set(256usize);
        }
    }
    #[inline]
    pub unsafe fn clear_nbld_en() {
        unsafe {
            _clear(256usize);
        }
    }
    #[inline]
    pub unsafe fn set_newnmi() {
        unsafe {
            _set(512usize);
        }
    }
    #[inline]
    pub unsafe fn clear_newnmi() {
        unsafe {
            _clear(512usize);
        }
    }
    #[inline]
    pub unsafe fn set_vcgl1_en() {
        unsafe {
            _set(1024usize);
        }
    }
    #[inline]
    pub unsafe fn clear_vcgl1_en() {
        unsafe {
            _clear(1024usize);
        }
    }
    #[inline]
    pub unsafe fn set_vcgl2_en() {
        unsafe {
            _set(2048usize);
        }
    }
    #[inline]
    pub unsafe fn clear_vcgl2_en() {
        unsafe {
            _clear(2048usize);
        }
    }
    #[inline]
    pub unsafe fn set_vcgl3_en() {
        unsafe {
            _set(4096usize);
        }
    }
    #[inline]
    pub unsafe fn clear_vcgl3_en() {
        unsafe {
            _clear(4096usize);
        }
    }
    #[doc = "“Load to x0” exception generation control bit"]
    #[inline]
    pub unsafe fn set_ldx0nxp() {
        unsafe {
            _set(8192usize);
        }
    }
    #[doc = "“Load to x0” exception generation control bit"]
    #[inline]
    pub unsafe fn clear_ldx0nxp() {
        unsafe {
            _clear(8192usize);
        }
    }
}
#[doc = "Counter not counting in M-mode"]
pub mod mcountermask_m {
    #[doc = "Machine Counter Fields"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McounterCommon(pub u32);
    impl McounterCommon {
        #[doc = "Cycle counter"]
        #[must_use]
        #[inline(always)]
        pub const fn cy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cycle counter"]
        #[inline(always)]
        pub const fn set_cy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Instruction retired counter"]
        #[must_use]
        #[inline(always)]
        pub const fn ir(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction retired counter"]
        #[inline(always)]
        pub const fn set_ir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[must_use]
        #[inline(always)]
        pub const fn hpm(&self, n: usize) -> bool {
            assert!(n < 29usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[inline(always)]
        pub const fn set_hpm(&mut self, n: usize, val: bool) {
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
    impl core::fmt::Debug for McounterCommon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("McounterCommon")
                .field("cy", &self.cy())
                .field("ir", &self.ir())
                .field("hpm[0]", &self.hpm(0usize))
                .field("hpm[1]", &self.hpm(1usize))
                .field("hpm[2]", &self.hpm(2usize))
                .field("hpm[3]", &self.hpm(3usize))
                .field("hpm[4]", &self.hpm(4usize))
                .field("hpm[5]", &self.hpm(5usize))
                .field("hpm[6]", &self.hpm(6usize))
                .field("hpm[7]", &self.hpm(7usize))
                .field("hpm[8]", &self.hpm(8usize))
                .field("hpm[9]", &self.hpm(9usize))
                .field("hpm[10]", &self.hpm(10usize))
                .field("hpm[11]", &self.hpm(11usize))
                .field("hpm[12]", &self.hpm(12usize))
                .field("hpm[13]", &self.hpm(13usize))
                .field("hpm[14]", &self.hpm(14usize))
                .field("hpm[15]", &self.hpm(15usize))
                .field("hpm[16]", &self.hpm(16usize))
                .field("hpm[17]", &self.hpm(17usize))
                .field("hpm[18]", &self.hpm(18usize))
                .field("hpm[19]", &self.hpm(19usize))
                .field("hpm[20]", &self.hpm(20usize))
                .field("hpm[21]", &self.hpm(21usize))
                .field("hpm[22]", &self.hpm(22usize))
                .field("hpm[23]", &self.hpm(23usize))
                .field("hpm[24]", &self.hpm(24usize))
                .field("hpm[25]", &self.hpm(25usize))
                .field("hpm[26]", &self.hpm(26usize))
                .field("hpm[27]", &self.hpm(27usize))
                .field("hpm[28]", &self.hpm(28usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for McounterCommon {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "McounterCommon {{ cy: {=bool:?}, ir: {=bool:?}, hpm[0]: {=bool:?}, hpm[1]: {=bool:?}, hpm[2]: {=bool:?}, hpm[3]: {=bool:?}, hpm[4]: {=bool:?}, hpm[5]: {=bool:?}, hpm[6]: {=bool:?}, hpm[7]: {=bool:?}, hpm[8]: {=bool:?}, hpm[9]: {=bool:?}, hpm[10]: {=bool:?}, hpm[11]: {=bool:?}, hpm[12]: {=bool:?}, hpm[13]: {=bool:?}, hpm[14]: {=bool:?}, hpm[15]: {=bool:?}, hpm[16]: {=bool:?}, hpm[17]: {=bool:?}, hpm[18]: {=bool:?}, hpm[19]: {=bool:?}, hpm[20]: {=bool:?}, hpm[21]: {=bool:?}, hpm[22]: {=bool:?}, hpm[23]: {=bool:?}, hpm[24]: {=bool:?}, hpm[25]: {=bool:?}, hpm[26]: {=bool:?}, hpm[27]: {=bool:?}, hpm[28]: {=bool:?} }}",
                self.cy(),
                self.ir(),
                self.hpm(0usize),
                self.hpm(1usize),
                self.hpm(2usize),
                self.hpm(3usize),
                self.hpm(4usize),
                self.hpm(5usize),
                self.hpm(6usize),
                self.hpm(7usize),
                self.hpm(8usize),
                self.hpm(9usize),
                self.hpm(10usize),
                self.hpm(11usize),
                self.hpm(12usize),
                self.hpm(13usize),
                self.hpm(14usize),
                self.hpm(15usize),
                self.hpm(16usize),
                self.hpm(17usize),
                self.hpm(18usize),
                self.hpm(19usize),
                self.hpm(20usize),
                self.hpm(21usize),
                self.hpm(22usize),
                self.hpm(23usize),
                self.hpm(24usize),
                self.hpm(25usize),
                self.hpm(26usize),
                self.hpm(27usize),
                self.hpm(28usize)
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7d1, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7d1, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7d1, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7d1, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> McounterCommon {
        unsafe { McounterCommon(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: McounterCommon) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut McounterCommon) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn set_cy() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn clear_cy() {
        unsafe {
            _clear(1usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn set_ir() {
        unsafe {
            _set(4usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn clear_ir() {
        unsafe {
            _clear(4usize);
        }
    }
}
#[doc = "Counter not counting in S-mode"]
pub mod mcountermask_s {
    #[doc = "Machine Counter Fields"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McounterCommon(pub u32);
    impl McounterCommon {
        #[doc = "Cycle counter"]
        #[must_use]
        #[inline(always)]
        pub const fn cy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cycle counter"]
        #[inline(always)]
        pub const fn set_cy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Instruction retired counter"]
        #[must_use]
        #[inline(always)]
        pub const fn ir(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction retired counter"]
        #[inline(always)]
        pub const fn set_ir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[must_use]
        #[inline(always)]
        pub const fn hpm(&self, n: usize) -> bool {
            assert!(n < 29usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[inline(always)]
        pub const fn set_hpm(&mut self, n: usize, val: bool) {
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
    impl core::fmt::Debug for McounterCommon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("McounterCommon")
                .field("cy", &self.cy())
                .field("ir", &self.ir())
                .field("hpm[0]", &self.hpm(0usize))
                .field("hpm[1]", &self.hpm(1usize))
                .field("hpm[2]", &self.hpm(2usize))
                .field("hpm[3]", &self.hpm(3usize))
                .field("hpm[4]", &self.hpm(4usize))
                .field("hpm[5]", &self.hpm(5usize))
                .field("hpm[6]", &self.hpm(6usize))
                .field("hpm[7]", &self.hpm(7usize))
                .field("hpm[8]", &self.hpm(8usize))
                .field("hpm[9]", &self.hpm(9usize))
                .field("hpm[10]", &self.hpm(10usize))
                .field("hpm[11]", &self.hpm(11usize))
                .field("hpm[12]", &self.hpm(12usize))
                .field("hpm[13]", &self.hpm(13usize))
                .field("hpm[14]", &self.hpm(14usize))
                .field("hpm[15]", &self.hpm(15usize))
                .field("hpm[16]", &self.hpm(16usize))
                .field("hpm[17]", &self.hpm(17usize))
                .field("hpm[18]", &self.hpm(18usize))
                .field("hpm[19]", &self.hpm(19usize))
                .field("hpm[20]", &self.hpm(20usize))
                .field("hpm[21]", &self.hpm(21usize))
                .field("hpm[22]", &self.hpm(22usize))
                .field("hpm[23]", &self.hpm(23usize))
                .field("hpm[24]", &self.hpm(24usize))
                .field("hpm[25]", &self.hpm(25usize))
                .field("hpm[26]", &self.hpm(26usize))
                .field("hpm[27]", &self.hpm(27usize))
                .field("hpm[28]", &self.hpm(28usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for McounterCommon {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "McounterCommon {{ cy: {=bool:?}, ir: {=bool:?}, hpm[0]: {=bool:?}, hpm[1]: {=bool:?}, hpm[2]: {=bool:?}, hpm[3]: {=bool:?}, hpm[4]: {=bool:?}, hpm[5]: {=bool:?}, hpm[6]: {=bool:?}, hpm[7]: {=bool:?}, hpm[8]: {=bool:?}, hpm[9]: {=bool:?}, hpm[10]: {=bool:?}, hpm[11]: {=bool:?}, hpm[12]: {=bool:?}, hpm[13]: {=bool:?}, hpm[14]: {=bool:?}, hpm[15]: {=bool:?}, hpm[16]: {=bool:?}, hpm[17]: {=bool:?}, hpm[18]: {=bool:?}, hpm[19]: {=bool:?}, hpm[20]: {=bool:?}, hpm[21]: {=bool:?}, hpm[22]: {=bool:?}, hpm[23]: {=bool:?}, hpm[24]: {=bool:?}, hpm[25]: {=bool:?}, hpm[26]: {=bool:?}, hpm[27]: {=bool:?}, hpm[28]: {=bool:?} }}",
                self.cy(),
                self.ir(),
                self.hpm(0usize),
                self.hpm(1usize),
                self.hpm(2usize),
                self.hpm(3usize),
                self.hpm(4usize),
                self.hpm(5usize),
                self.hpm(6usize),
                self.hpm(7usize),
                self.hpm(8usize),
                self.hpm(9usize),
                self.hpm(10usize),
                self.hpm(11usize),
                self.hpm(12usize),
                self.hpm(13usize),
                self.hpm(14usize),
                self.hpm(15usize),
                self.hpm(16usize),
                self.hpm(17usize),
                self.hpm(18usize),
                self.hpm(19usize),
                self.hpm(20usize),
                self.hpm(21usize),
                self.hpm(22usize),
                self.hpm(23usize),
                self.hpm(24usize),
                self.hpm(25usize),
                self.hpm(26usize),
                self.hpm(27usize),
                self.hpm(28usize)
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7d2, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7d2, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7d2, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7d2, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> McounterCommon {
        unsafe { McounterCommon(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: McounterCommon) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut McounterCommon) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn set_cy() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn clear_cy() {
        unsafe {
            _clear(1usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn set_ir() {
        unsafe {
            _set(4usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn clear_ir() {
        unsafe {
            _clear(4usize);
        }
    }
}
#[doc = "Counter not counting in U-mode"]
pub mod mcountermask_u {
    #[doc = "Machine Counter Fields"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McounterCommon(pub u32);
    impl McounterCommon {
        #[doc = "Cycle counter"]
        #[must_use]
        #[inline(always)]
        pub const fn cy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cycle counter"]
        #[inline(always)]
        pub const fn set_cy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Instruction retired counter"]
        #[must_use]
        #[inline(always)]
        pub const fn ir(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction retired counter"]
        #[inline(always)]
        pub const fn set_ir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[must_use]
        #[inline(always)]
        pub const fn hpm(&self, n: usize) -> bool {
            assert!(n < 29usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[inline(always)]
        pub const fn set_hpm(&mut self, n: usize, val: bool) {
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
    impl core::fmt::Debug for McounterCommon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("McounterCommon")
                .field("cy", &self.cy())
                .field("ir", &self.ir())
                .field("hpm[0]", &self.hpm(0usize))
                .field("hpm[1]", &self.hpm(1usize))
                .field("hpm[2]", &self.hpm(2usize))
                .field("hpm[3]", &self.hpm(3usize))
                .field("hpm[4]", &self.hpm(4usize))
                .field("hpm[5]", &self.hpm(5usize))
                .field("hpm[6]", &self.hpm(6usize))
                .field("hpm[7]", &self.hpm(7usize))
                .field("hpm[8]", &self.hpm(8usize))
                .field("hpm[9]", &self.hpm(9usize))
                .field("hpm[10]", &self.hpm(10usize))
                .field("hpm[11]", &self.hpm(11usize))
                .field("hpm[12]", &self.hpm(12usize))
                .field("hpm[13]", &self.hpm(13usize))
                .field("hpm[14]", &self.hpm(14usize))
                .field("hpm[15]", &self.hpm(15usize))
                .field("hpm[16]", &self.hpm(16usize))
                .field("hpm[17]", &self.hpm(17usize))
                .field("hpm[18]", &self.hpm(18usize))
                .field("hpm[19]", &self.hpm(19usize))
                .field("hpm[20]", &self.hpm(20usize))
                .field("hpm[21]", &self.hpm(21usize))
                .field("hpm[22]", &self.hpm(22usize))
                .field("hpm[23]", &self.hpm(23usize))
                .field("hpm[24]", &self.hpm(24usize))
                .field("hpm[25]", &self.hpm(25usize))
                .field("hpm[26]", &self.hpm(26usize))
                .field("hpm[27]", &self.hpm(27usize))
                .field("hpm[28]", &self.hpm(28usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for McounterCommon {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "McounterCommon {{ cy: {=bool:?}, ir: {=bool:?}, hpm[0]: {=bool:?}, hpm[1]: {=bool:?}, hpm[2]: {=bool:?}, hpm[3]: {=bool:?}, hpm[4]: {=bool:?}, hpm[5]: {=bool:?}, hpm[6]: {=bool:?}, hpm[7]: {=bool:?}, hpm[8]: {=bool:?}, hpm[9]: {=bool:?}, hpm[10]: {=bool:?}, hpm[11]: {=bool:?}, hpm[12]: {=bool:?}, hpm[13]: {=bool:?}, hpm[14]: {=bool:?}, hpm[15]: {=bool:?}, hpm[16]: {=bool:?}, hpm[17]: {=bool:?}, hpm[18]: {=bool:?}, hpm[19]: {=bool:?}, hpm[20]: {=bool:?}, hpm[21]: {=bool:?}, hpm[22]: {=bool:?}, hpm[23]: {=bool:?}, hpm[24]: {=bool:?}, hpm[25]: {=bool:?}, hpm[26]: {=bool:?}, hpm[27]: {=bool:?}, hpm[28]: {=bool:?} }}",
                self.cy(),
                self.ir(),
                self.hpm(0usize),
                self.hpm(1usize),
                self.hpm(2usize),
                self.hpm(3usize),
                self.hpm(4usize),
                self.hpm(5usize),
                self.hpm(6usize),
                self.hpm(7usize),
                self.hpm(8usize),
                self.hpm(9usize),
                self.hpm(10usize),
                self.hpm(11usize),
                self.hpm(12usize),
                self.hpm(13usize),
                self.hpm(14usize),
                self.hpm(15usize),
                self.hpm(16usize),
                self.hpm(17usize),
                self.hpm(18usize),
                self.hpm(19usize),
                self.hpm(20usize),
                self.hpm(21usize),
                self.hpm(22usize),
                self.hpm(23usize),
                self.hpm(24usize),
                self.hpm(25usize),
                self.hpm(26usize),
                self.hpm(27usize),
                self.hpm(28usize)
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7d3, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7d3, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7d3, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7d3, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> McounterCommon {
        unsafe { McounterCommon(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: McounterCommon) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut McounterCommon) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn set_cy() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn clear_cy() {
        unsafe {
            _clear(1usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn set_ir() {
        unsafe {
            _set(4usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn clear_ir() {
        unsafe {
            _clear(4usize);
        }
    }
}
#[doc = "Counter overflow status"]
pub mod mcounterovf {
    #[doc = "Machine Counter Fields"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McounterCommon(pub u32);
    impl McounterCommon {
        #[doc = "Cycle counter"]
        #[must_use]
        #[inline(always)]
        pub const fn cy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cycle counter"]
        #[inline(always)]
        pub const fn set_cy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Instruction retired counter"]
        #[must_use]
        #[inline(always)]
        pub const fn ir(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction retired counter"]
        #[inline(always)]
        pub const fn set_ir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[must_use]
        #[inline(always)]
        pub const fn hpm(&self, n: usize) -> bool {
            assert!(n < 29usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[inline(always)]
        pub const fn set_hpm(&mut self, n: usize, val: bool) {
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
    impl core::fmt::Debug for McounterCommon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("McounterCommon")
                .field("cy", &self.cy())
                .field("ir", &self.ir())
                .field("hpm[0]", &self.hpm(0usize))
                .field("hpm[1]", &self.hpm(1usize))
                .field("hpm[2]", &self.hpm(2usize))
                .field("hpm[3]", &self.hpm(3usize))
                .field("hpm[4]", &self.hpm(4usize))
                .field("hpm[5]", &self.hpm(5usize))
                .field("hpm[6]", &self.hpm(6usize))
                .field("hpm[7]", &self.hpm(7usize))
                .field("hpm[8]", &self.hpm(8usize))
                .field("hpm[9]", &self.hpm(9usize))
                .field("hpm[10]", &self.hpm(10usize))
                .field("hpm[11]", &self.hpm(11usize))
                .field("hpm[12]", &self.hpm(12usize))
                .field("hpm[13]", &self.hpm(13usize))
                .field("hpm[14]", &self.hpm(14usize))
                .field("hpm[15]", &self.hpm(15usize))
                .field("hpm[16]", &self.hpm(16usize))
                .field("hpm[17]", &self.hpm(17usize))
                .field("hpm[18]", &self.hpm(18usize))
                .field("hpm[19]", &self.hpm(19usize))
                .field("hpm[20]", &self.hpm(20usize))
                .field("hpm[21]", &self.hpm(21usize))
                .field("hpm[22]", &self.hpm(22usize))
                .field("hpm[23]", &self.hpm(23usize))
                .field("hpm[24]", &self.hpm(24usize))
                .field("hpm[25]", &self.hpm(25usize))
                .field("hpm[26]", &self.hpm(26usize))
                .field("hpm[27]", &self.hpm(27usize))
                .field("hpm[28]", &self.hpm(28usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for McounterCommon {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "McounterCommon {{ cy: {=bool:?}, ir: {=bool:?}, hpm[0]: {=bool:?}, hpm[1]: {=bool:?}, hpm[2]: {=bool:?}, hpm[3]: {=bool:?}, hpm[4]: {=bool:?}, hpm[5]: {=bool:?}, hpm[6]: {=bool:?}, hpm[7]: {=bool:?}, hpm[8]: {=bool:?}, hpm[9]: {=bool:?}, hpm[10]: {=bool:?}, hpm[11]: {=bool:?}, hpm[12]: {=bool:?}, hpm[13]: {=bool:?}, hpm[14]: {=bool:?}, hpm[15]: {=bool:?}, hpm[16]: {=bool:?}, hpm[17]: {=bool:?}, hpm[18]: {=bool:?}, hpm[19]: {=bool:?}, hpm[20]: {=bool:?}, hpm[21]: {=bool:?}, hpm[22]: {=bool:?}, hpm[23]: {=bool:?}, hpm[24]: {=bool:?}, hpm[25]: {=bool:?}, hpm[26]: {=bool:?}, hpm[27]: {=bool:?}, hpm[28]: {=bool:?} }}",
                self.cy(),
                self.ir(),
                self.hpm(0usize),
                self.hpm(1usize),
                self.hpm(2usize),
                self.hpm(3usize),
                self.hpm(4usize),
                self.hpm(5usize),
                self.hpm(6usize),
                self.hpm(7usize),
                self.hpm(8usize),
                self.hpm(9usize),
                self.hpm(10usize),
                self.hpm(11usize),
                self.hpm(12usize),
                self.hpm(13usize),
                self.hpm(14usize),
                self.hpm(15usize),
                self.hpm(16usize),
                self.hpm(17usize),
                self.hpm(18usize),
                self.hpm(19usize),
                self.hpm(20usize),
                self.hpm(21usize),
                self.hpm(22usize),
                self.hpm(23usize),
                self.hpm(24usize),
                self.hpm(25usize),
                self.hpm(26usize),
                self.hpm(27usize),
                self.hpm(28usize)
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7d4, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7d4, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7d4, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7d4, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> McounterCommon {
        unsafe { McounterCommon(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: McounterCommon) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut McounterCommon) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn set_cy() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn clear_cy() {
        unsafe {
            _clear(1usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn set_ir() {
        unsafe {
            _set(4usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn clear_ir() {
        unsafe {
            _clear(4usize);
        }
    }
}
#[doc = "Supervisor local interrupt delegation"]
pub mod mslideleg {
    #[doc = "Machine Supervisor Local Interrupt Delegation Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mslideleg(pub u32);
    impl Mslideleg {
        #[must_use]
        #[inline(always)]
        pub const fn imecci(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_imecci(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bwei(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bwei(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pmovi(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pmovi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn imeccdmr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_imeccdmr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aceerr(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_aceerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Mslideleg {
        #[inline(always)]
        fn default() -> Mslideleg {
            Mslideleg(0)
        }
    }
    impl core::fmt::Debug for Mslideleg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mslideleg")
                .field("imecci", &self.imecci())
                .field("bwei", &self.bwei())
                .field("pmovi", &self.pmovi())
                .field("imeccdmr", &self.imeccdmr())
                .field("aceerr", &self.aceerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mslideleg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mslideleg {{ imecci: {=bool:?}, bwei: {=bool:?}, pmovi: {=bool:?}, imeccdmr: {=bool:?}, aceerr: {=bool:?} }}",
                self.imecci(),
                self.bwei(),
                self.pmovi(),
                self.imeccdmr(),
                self.aceerr()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7d5, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7d5, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7d5, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7d5, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Mslideleg {
        unsafe { Mslideleg(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Mslideleg) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Mslideleg) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[inline]
    pub unsafe fn set_imecci() {
        unsafe {
            _set(65536usize);
        }
    }
    #[inline]
    pub unsafe fn clear_imecci() {
        unsafe {
            _clear(65536usize);
        }
    }
    #[inline]
    pub unsafe fn set_bwei() {
        unsafe {
            _set(131072usize);
        }
    }
    #[inline]
    pub unsafe fn clear_bwei() {
        unsafe {
            _clear(131072usize);
        }
    }
    #[inline]
    pub unsafe fn set_pmovi() {
        unsafe {
            _set(262144usize);
        }
    }
    #[inline]
    pub unsafe fn clear_pmovi() {
        unsafe {
            _clear(262144usize);
        }
    }
    #[inline]
    pub unsafe fn set_imeccdmr() {
        unsafe {
            _set(524288usize);
        }
    }
    #[inline]
    pub unsafe fn clear_imeccdmr() {
        unsafe {
            _clear(524288usize);
        }
    }
    #[inline]
    pub unsafe fn set_aceerr() {
        unsafe {
            _set(16777216usize);
        }
    }
    #[inline]
    pub unsafe fn clear_aceerr() {
        unsafe {
            _clear(16777216usize);
        }
    }
}
#[doc = "Status save register (level 1 & level 2)"]
pub mod msavestatus {
    #[doc = "Machine Status Save Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Msavestatus(pub u32);
    impl Msavestatus {
        #[must_use]
        #[inline(always)]
        pub const fn mpie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mpp(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mpp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ppft_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ppft_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pime(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pime(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pdme(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pdme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ptyp(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for Msavestatus {
        #[inline(always)]
        fn default() -> Msavestatus {
            Msavestatus(0)
        }
    }
    impl core::fmt::Debug for Msavestatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Msavestatus")
                .field("mpie", &self.mpie())
                .field("mpp", &self.mpp())
                .field("ppft_en", &self.ppft_en())
                .field("pime", &self.pime())
                .field("pdme", &self.pdme())
                .field("ptyp", &self.ptyp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Msavestatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Msavestatus {{ mpie: {=bool:?}, mpp: {=u8:?}, ppft_en: {=bool:?}, pime: {=bool:?}, pdme: {=bool:?}, ptyp: {=u8:?} }}",
                self.mpie(),
                self.mpp(),
                self.ppft_en(),
                self.pime(),
                self.pdme(),
                self.ptyp()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7d6, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7d6, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7d6, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7d6, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Msavestatus {
        unsafe { Msavestatus(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Msavestatus) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Msavestatus) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[inline]
    pub unsafe fn set_mpie() {
        unsafe {
            _set(1usize);
        }
    }
    #[inline]
    pub unsafe fn clear_mpie() {
        unsafe {
            _clear(1usize);
        }
    }
    #[inline]
    pub unsafe fn set_mpp(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(3usize << 1usize);
        bits |= (val as usize & 3usize) << 1usize;
        unsafe {
            _write(bits);
        }
    }
    #[inline]
    pub unsafe fn set_ppft_en() {
        unsafe {
            _set(8usize);
        }
    }
    #[inline]
    pub unsafe fn clear_ppft_en() {
        unsafe {
            _clear(8usize);
        }
    }
    #[inline]
    pub unsafe fn set_pime() {
        unsafe {
            _set(16usize);
        }
    }
    #[inline]
    pub unsafe fn clear_pime() {
        unsafe {
            _clear(16usize);
        }
    }
    #[inline]
    pub unsafe fn set_pdme() {
        unsafe {
            _set(32usize);
        }
    }
    #[inline]
    pub unsafe fn clear_pdme() {
        unsafe {
            _clear(32usize);
        }
    }
    #[inline]
    pub unsafe fn set_ptyp(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(3usize << 6usize);
        bits |= (val as usize & 3usize) << 6usize;
        unsafe {
            _write(bits);
        }
    }
}
#[doc = "EPC save register (level 1)"]
pub mod msaveepc1 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7d7, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7d7, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7d7, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7d7, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "Exception cause save register (level 1)"]
pub mod msavecause1 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7d8, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7d8, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7d8, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7d8, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "EPC save register (level 2)"]
pub mod msaveepc2 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7d9, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7d9, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7d9, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7d9, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "Exception cause save register (level 2)"]
pub mod msavecause2 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7da, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7da, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7da, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7da, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "Detailed exception cause save (level 1)"]
pub mod msavedcause1 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7db, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7db, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7db, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7db, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "Detailed exception cause save (level 2)"]
pub mod msavedcause2 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7dc, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7dc, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7dc, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7dc, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "Clock control"]
pub mod mclk_ctl {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7df, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7df, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7df, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7df, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "Enable exception to enter Halt Mode."]
pub mod dexc2dbg {
    #[doc = "Exception Redirection Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dexc2dbg(pub u32);
    impl Dexc2dbg {
        #[must_use]
        #[inline(always)]
        pub const fn iam(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_iam(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn iaf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_iaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ii(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ii(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn nmi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_nmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lam(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lam(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn laf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_laf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sam(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sam(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn saf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_saf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn uec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_uec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sec(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hec(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mec(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn hsp(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_hsp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ace(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ace(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn slpecc(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_slpecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bwe(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ipf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ipf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lpf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn spf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_spf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pmov(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pmov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Dexc2dbg {
        #[inline(always)]
        fn default() -> Dexc2dbg {
            Dexc2dbg(0)
        }
    }
    impl core::fmt::Debug for Dexc2dbg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dexc2dbg")
                .field("iam", &self.iam())
                .field("iaf", &self.iaf())
                .field("ii", &self.ii())
                .field("nmi", &self.nmi())
                .field("lam", &self.lam())
                .field("laf", &self.laf())
                .field("sam", &self.sam())
                .field("saf", &self.saf())
                .field("uec", &self.uec())
                .field("sec", &self.sec())
                .field("hec", &self.hec())
                .field("mec", &self.mec())
                .field("hsp", &self.hsp())
                .field("ace", &self.ace())
                .field("slpecc", &self.slpecc())
                .field("bwe", &self.bwe())
                .field("ipf", &self.ipf())
                .field("lpf", &self.lpf())
                .field("spf", &self.spf())
                .field("pmov", &self.pmov())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dexc2dbg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dexc2dbg {{ iam: {=bool:?}, iaf: {=bool:?}, ii: {=bool:?}, nmi: {=bool:?}, lam: {=bool:?}, laf: {=bool:?}, sam: {=bool:?}, saf: {=bool:?}, uec: {=bool:?}, sec: {=bool:?}, hec: {=bool:?}, mec: {=bool:?}, hsp: {=bool:?}, ace: {=bool:?}, slpecc: {=bool:?}, bwe: {=bool:?}, ipf: {=bool:?}, lpf: {=bool:?}, spf: {=bool:?}, pmov: {=bool:?} }}",
                self.iam(),
                self.iaf(),
                self.ii(),
                self.nmi(),
                self.lam(),
                self.laf(),
                self.sam(),
                self.saf(),
                self.uec(),
                self.sec(),
                self.hec(),
                self.mec(),
                self.hsp(),
                self.ace(),
                self.slpecc(),
                self.bwe(),
                self.ipf(),
                self.lpf(),
                self.spf(),
                self.pmov()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7e0, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7e0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7e0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7e0, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Dexc2dbg {
        unsafe { Dexc2dbg(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Dexc2dbg) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Dexc2dbg) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[inline]
    pub unsafe fn set_iam() {
        unsafe {
            _set(1usize);
        }
    }
    #[inline]
    pub unsafe fn clear_iam() {
        unsafe {
            _clear(1usize);
        }
    }
    #[inline]
    pub unsafe fn set_iaf() {
        unsafe {
            _set(2usize);
        }
    }
    #[inline]
    pub unsafe fn clear_iaf() {
        unsafe {
            _clear(2usize);
        }
    }
    #[inline]
    pub unsafe fn set_ii() {
        unsafe {
            _set(4usize);
        }
    }
    #[inline]
    pub unsafe fn clear_ii() {
        unsafe {
            _clear(4usize);
        }
    }
    #[inline]
    pub unsafe fn set_nmi() {
        unsafe {
            _set(8usize);
        }
    }
    #[inline]
    pub unsafe fn clear_nmi() {
        unsafe {
            _clear(8usize);
        }
    }
    #[inline]
    pub unsafe fn set_lam() {
        unsafe {
            _set(16usize);
        }
    }
    #[inline]
    pub unsafe fn clear_lam() {
        unsafe {
            _clear(16usize);
        }
    }
    #[inline]
    pub unsafe fn set_laf() {
        unsafe {
            _set(32usize);
        }
    }
    #[inline]
    pub unsafe fn clear_laf() {
        unsafe {
            _clear(32usize);
        }
    }
    #[inline]
    pub unsafe fn set_sam() {
        unsafe {
            _set(64usize);
        }
    }
    #[inline]
    pub unsafe fn clear_sam() {
        unsafe {
            _clear(64usize);
        }
    }
    #[inline]
    pub unsafe fn set_saf() {
        unsafe {
            _set(128usize);
        }
    }
    #[inline]
    pub unsafe fn clear_saf() {
        unsafe {
            _clear(128usize);
        }
    }
    #[inline]
    pub unsafe fn set_uec() {
        unsafe {
            _set(256usize);
        }
    }
    #[inline]
    pub unsafe fn clear_uec() {
        unsafe {
            _clear(256usize);
        }
    }
    #[inline]
    pub unsafe fn set_sec() {
        unsafe {
            _set(512usize);
        }
    }
    #[inline]
    pub unsafe fn clear_sec() {
        unsafe {
            _clear(512usize);
        }
    }
    #[inline]
    pub unsafe fn set_hec() {
        unsafe {
            _set(1024usize);
        }
    }
    #[inline]
    pub unsafe fn clear_hec() {
        unsafe {
            _clear(1024usize);
        }
    }
    #[inline]
    pub unsafe fn set_mec() {
        unsafe {
            _set(2048usize);
        }
    }
    #[inline]
    pub unsafe fn clear_mec() {
        unsafe {
            _clear(2048usize);
        }
    }
    #[inline]
    pub unsafe fn set_hsp() {
        unsafe {
            _set(4096usize);
        }
    }
    #[inline]
    pub unsafe fn clear_hsp() {
        unsafe {
            _clear(4096usize);
        }
    }
    #[inline]
    pub unsafe fn set_ace() {
        unsafe {
            _set(8192usize);
        }
    }
    #[inline]
    pub unsafe fn clear_ace() {
        unsafe {
            _clear(8192usize);
        }
    }
    #[inline]
    pub unsafe fn set_slpecc() {
        unsafe {
            _set(16384usize);
        }
    }
    #[inline]
    pub unsafe fn clear_slpecc() {
        unsafe {
            _clear(16384usize);
        }
    }
    #[inline]
    pub unsafe fn set_bwe() {
        unsafe {
            _set(32768usize);
        }
    }
    #[inline]
    pub unsafe fn clear_bwe() {
        unsafe {
            _clear(32768usize);
        }
    }
    #[inline]
    pub unsafe fn set_ipf() {
        unsafe {
            _set(65536usize);
        }
    }
    #[inline]
    pub unsafe fn clear_ipf() {
        unsafe {
            _clear(65536usize);
        }
    }
    #[inline]
    pub unsafe fn set_lpf() {
        unsafe {
            _set(131072usize);
        }
    }
    #[inline]
    pub unsafe fn clear_lpf() {
        unsafe {
            _clear(131072usize);
        }
    }
    #[inline]
    pub unsafe fn set_spf() {
        unsafe {
            _set(262144usize);
        }
    }
    #[inline]
    pub unsafe fn clear_spf() {
        unsafe {
            _clear(262144usize);
        }
    }
    #[inline]
    pub unsafe fn set_pmov() {
        unsafe {
            _set(524288usize);
        }
    }
    #[inline]
    pub unsafe fn clear_pmov() {
        unsafe {
            _clear(524288usize);
        }
    }
}
#[doc = "Detailed exception type information when an exception enters Halt Mode."]
pub mod ddcause {
    #[doc = "Debug Detailed Cause Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ddcause(pub u32);
    impl Ddcause {
        #[doc = "Indicates the main types of a Debug Mode entrance. Its definition is listed below."]
        #[must_use]
        #[inline(always)]
        pub const fn maintype(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Indicates the main types of a Debug Mode entrance. Its definition is listed below."]
        #[inline(always)]
        pub const fn set_maintype(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Indicates the subtypes of a main type. Its definition is listed below. A main type may not have a subtype definition."]
        #[must_use]
        #[inline(always)]
        pub const fn subtype(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Indicates the subtypes of a main type. Its definition is listed below. A main type may not have a subtype definition."]
        #[inline(always)]
        pub const fn set_subtype(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Ddcause {
        #[inline(always)]
        fn default() -> Ddcause {
            Ddcause(0)
        }
    }
    impl core::fmt::Debug for Ddcause {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ddcause")
                .field("maintype", &self.maintype())
                .field("subtype", &self.subtype())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ddcause {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ddcause {{ maintype: {=u8:?}, subtype: {=u8:?} }}",
                self.maintype(),
                self.subtype()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7e1, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7e1, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7e1, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7e1, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Ddcause {
        unsafe { Ddcause(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Ddcause) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Ddcause) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Indicates the main types of a Debug Mode entrance. Its definition is listed below."]
    #[inline]
    pub unsafe fn set_maintype(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(255usize << 0usize);
        bits |= (val as usize & 255usize) << 0usize;
        unsafe {
            _write(bits);
        }
    }
    #[doc = "Indicates the subtypes of a main type. Its definition is listed below. A main type may not have a subtype definition."]
    #[inline]
    pub unsafe fn set_subtype(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(255usize << 8usize);
        bits |= (val as usize & 255usize) << 8usize;
        unsafe {
            _write(bits);
        }
    }
}
#[doc = "Store mxstatus to stack"]
pub mod pushmxstatus {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7eb, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7eb, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7eb, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7eb, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "Interrupt common entry point"]
pub mod mirq_entry {
    #[doc = "Machine Interrupt Common Entry Address Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MirqEntry(pub u32);
    impl MirqEntry {
        #[doc = "Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable"]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interrupt Common Entry Address"]
        #[must_use]
        #[inline(always)]
        pub const fn icea(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Interrupt Common Entry Address"]
        #[inline(always)]
        pub const fn set_icea(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for MirqEntry {
        #[inline(always)]
        fn default() -> MirqEntry {
            MirqEntry(0)
        }
    }
    impl core::fmt::Debug for MirqEntry {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MirqEntry")
                .field("en", &self.en())
                .field("icea", &self.icea())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MirqEntry {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MirqEntry {{ en: {=bool:?}, icea: {=u32:?} }}",
                self.en(),
                self.icea()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7ec, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7ec, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7ec, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7ec, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> MirqEntry {
        unsafe { MirqEntry(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: MirqEntry) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut MirqEntry) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Enable"]
    #[inline]
    pub unsafe fn set_en() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub unsafe fn clear_en() {
        unsafe {
            _clear(1usize);
        }
    }
    #[doc = "Interrupt Common Entry Address"]
    #[inline]
    pub unsafe fn set_icea(val: u32) {
        let mut bits = unsafe { _read() };
        bits &= !(2147483647usize << 1usize);
        bits |= (val as usize & 2147483647usize) << 1usize;
        unsafe {
            _write(bits);
        }
    }
}
#[doc = "Select interrupt and call ISR"]
pub mod mintsel_jal {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7ed, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7ed, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7ed, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7ed, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "Store mcause to stack"]
pub mod pushmcause {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7ee, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7ee, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7ee, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7ee, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "Store mepc to stack"]
pub mod pushmepc {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7ef, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7ef, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7ef, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7ef, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "Private peripheral interface base address"]
pub mod mppib {
    #[doc = "PPI (Private Peripheral Interface) Base Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mppib(pub u32);
    impl Mppib {
        #[doc = "Private Peripheral Interface enable bit"]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Private Peripheral Interface enable bit"]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Indicates the power-of-2 size of the PPI region"]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x1f;
            val as u8
        }
        #[doc = "Indicates the power-of-2 size of the PPI region"]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
        }
        #[doc = "Base Physical Address"]
        #[must_use]
        #[inline(always)]
        pub const fn bpa(&self) -> u32 {
            let val = (self.0 >> 10usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "Base Physical Address"]
        #[inline(always)]
        pub const fn set_bpa(&mut self, val: u32) {
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
    impl core::fmt::Debug for Mppib {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mppib")
                .field("en", &self.en())
                .field("size", &self.size())
                .field("bpa", &self.bpa())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mppib {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mppib {{ en: {=bool:?}, size: {=u8:?}, bpa: {=u32:?} }}",
                self.en(),
                self.size(),
                self.bpa()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7f0, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7f0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7f0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7f0, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Mppib {
        unsafe { Mppib(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Mppib) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Mppib) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Private Peripheral Interface enable bit"]
    #[inline]
    pub unsafe fn set_en() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Private Peripheral Interface enable bit"]
    #[inline]
    pub unsafe fn clear_en() {
        unsafe {
            _clear(1usize);
        }
    }
    #[doc = "Indicates the power-of-2 size of the PPI region"]
    #[inline]
    pub unsafe fn set_size(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(31usize << 1usize);
        bits |= (val as usize & 31usize) << 1usize;
        unsafe {
            _write(bits);
        }
    }
    #[doc = "Base Physical Address"]
    #[inline]
    pub unsafe fn set_bpa(val: u32) {
        let mut bits = unsafe { _read() };
        bits &= !(4194303usize << 10usize);
        bits |= (val as usize & 4194303usize) << 10usize;
        unsafe {
            _write(bits);
        }
    }
}
#[doc = "Fast IO interface base address"]
pub mod mfiob {
    #[doc = "FIO (Fast IO Interface) Base Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mfiob(pub u32);
    impl Mfiob {
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
        }
        #[doc = "Base Physical Address"]
        #[must_use]
        #[inline(always)]
        pub const fn bpa(&self) -> u32 {
            let val = (self.0 >> 10usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "Base Physical Address"]
        #[inline(always)]
        pub const fn set_bpa(&mut self, val: u32) {
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
    impl core::fmt::Debug for Mfiob {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mfiob")
                .field("en", &self.en())
                .field("size", &self.size())
                .field("bpa", &self.bpa())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mfiob {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mfiob {{ en: {=bool:?}, size: {=u8:?}, bpa: {=u32:?} }}",
                self.en(),
                self.size(),
                self.bpa()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x7f1, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x7f1, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x7f1, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x7f1, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Mfiob {
        unsafe { Mfiob(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Mfiob) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Mfiob) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[inline]
    pub unsafe fn set_en() {
        unsafe {
            _set(1usize);
        }
    }
    #[inline]
    pub unsafe fn clear_en() {
        unsafe {
            _clear(1usize);
        }
    }
    #[inline]
    pub unsafe fn set_size(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(31usize << 1usize);
        bits |= (val as usize & 31usize) << 1usize;
        unsafe {
            _write(bits);
        }
    }
    #[doc = "Base Physical Address"]
    #[inline]
    pub unsafe fn set_bpa(val: u32) {
        let mut bits = unsafe { _read() };
        bits &= !(4194303usize << 10usize);
        bits |= (val as usize & 4194303usize) << 10usize;
        unsafe {
            _write(bits);
        }
    }
}
#[doc = "Instruction table base address for CoDense extension."]
pub mod uitb {
    #[doc = "Instruction Table Base Address Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uitb(pub u32);
    impl Uitb {
        #[doc = "if the Xcodense instruction table is hardwired"]
        #[must_use]
        #[inline(always)]
        pub const fn hw(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "if the Xcodense instruction table is hardwired"]
        #[inline(always)]
        pub const fn set_hw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Uitb {
        #[inline(always)]
        fn default() -> Uitb {
            Uitb(0)
        }
    }
    impl core::fmt::Debug for Uitb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uitb")
                .field("hw", &self.hw())
                .field("addr", &self.addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Uitb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Uitb {{ hw: {=bool:?}, addr: {=u32:?} }}",
                self.hw(),
                self.addr()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x800, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x800, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x800, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x800, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Uitb {
        unsafe { Uitb(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Uitb) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Uitb) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "if the Xcodense instruction table is hardwired"]
    #[inline]
    pub unsafe fn set_hw() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "if the Xcodense instruction table is hardwired"]
    #[inline]
    pub unsafe fn clear_hw() {
        unsafe {
            _clear(1usize);
        }
    }
    #[inline]
    pub unsafe fn set_addr(val: u32) {
        let mut bits = unsafe { _read() };
        bits &= !(1073741823usize << 2usize);
        bits |= (val as usize & 1073741823usize) << 2usize;
        unsafe {
            _write(bits);
        }
    }
}
#[doc = "Contains overflow flag for DSP extension."]
pub mod ucode {
    #[doc = "Code Register - Stores the overflow flag of the DSP extension"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ucode(pub u32);
    impl Ucode {
        #[doc = "Overflow flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ov(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow flag"]
        #[inline(always)]
        pub const fn set_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ucode {
        #[inline(always)]
        fn default() -> Ucode {
            Ucode(0)
        }
    }
    impl core::fmt::Debug for Ucode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ucode").field("ov", &self.ov()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ucode {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ucode {{ ov: {=bool:?} }}", self.ov())
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x801, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x801, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x801, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x801, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Ucode {
        unsafe { Ucode(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Ucode) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Ucode) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Overflow flag"]
    #[inline]
    pub unsafe fn set_ov() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Overflow flag"]
    #[inline]
    pub unsafe fn clear_ov() {
        unsafe {
            _clear(1usize);
        }
    }
}
#[doc = "User detailed trap cause"]
pub mod udcause {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x809, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x809, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x809, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x809, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "CCTL begin address"]
pub mod ucctlbeginaddr {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x80b, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x80b, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x80b, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x80b, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "CCTL command"]
pub mod ucctlcommand {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x80c, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x80c, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x80c, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x80c, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "Wait for event control"]
pub mod wfe {
    #[doc = "Wait for Event Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wfe(pub u32);
    impl Wfe {
        #[doc = "Wait for event"]
        #[must_use]
        #[inline(always)]
        pub const fn wfe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Wait for event"]
        #[inline(always)]
        pub const fn set_wfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Wfe {
        #[inline(always)]
        fn default() -> Wfe {
            Wfe(0)
        }
    }
    impl core::fmt::Debug for Wfe {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wfe").field("wfe", &self.wfe()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wfe {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wfe {{ wfe: {=bool:?} }}", self.wfe())
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x810, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x810, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x810, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x810, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Wfe {
        unsafe { Wfe(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Wfe) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Wfe) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Wait for event"]
    #[inline]
    pub unsafe fn set_wfe() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Wait for event"]
    #[inline]
    pub unsafe fn clear_wfe() {
        unsafe {
            _clear(1usize);
        }
    }
}
#[doc = "Sleep value"]
pub mod sleepvalue {
    #[doc = "Sleep Value Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sleepvalue(pub u32);
    impl Sleepvalue {
        #[doc = "Sleep value"]
        #[must_use]
        #[inline(always)]
        pub const fn sv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sleep value"]
        #[inline(always)]
        pub const fn set_sv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Sleepvalue {
        #[inline(always)]
        fn default() -> Sleepvalue {
            Sleepvalue(0)
        }
    }
    impl core::fmt::Debug for Sleepvalue {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sleepvalue")
                .field("sv", &self.sv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sleepvalue {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sleepvalue {{ sv: {=bool:?} }}", self.sv())
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x811, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x811, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x811, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x811, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Sleepvalue {
        unsafe { Sleepvalue(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Sleepvalue) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Sleepvalue) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Sleep value"]
    #[inline]
    pub unsafe fn set_sv() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Sleep value"]
    #[inline]
    pub unsafe fn clear_sv() {
        unsafe {
            _clear(1usize);
        }
    }
}
#[doc = "Transmit event"]
pub mod txevt {
    #[doc = "Transmit Event Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txevt(pub u32);
    impl Txevt {
        #[doc = "Event output"]
        #[must_use]
        #[inline(always)]
        pub const fn evto(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Event output"]
        #[inline(always)]
        pub const fn set_evto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Txevt {
        #[inline(always)]
        fn default() -> Txevt {
            Txevt(0)
        }
    }
    impl core::fmt::Debug for Txevt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txevt").field("evto", &self.evto()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txevt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txevt {{ evto: {=bool:?} }}", self.evto())
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x812, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x812, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x812, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x812, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Txevt {
        unsafe { Txevt(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Txevt) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Txevt) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Event output"]
    #[inline]
    pub unsafe fn set_evto() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Event output"]
    #[inline]
    pub unsafe fn clear_evto() {
        unsafe {
            _clear(1usize);
        }
    }
}
#[doc = "Supervisor local interrupt enable"]
pub mod slie {
    #[doc = "Supervisor Local Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Slie(pub u32);
    impl Slie {
        #[doc = "Enable S-mode slave-port ECC error local interrupt"]
        #[must_use]
        #[inline(always)]
        pub const fn imecci(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Enable S-mode slave-port ECC error local interrupt"]
        #[inline(always)]
        pub const fn set_imecci(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Enable S-mode bus read/write transaction error local interrupt"]
        #[must_use]
        #[inline(always)]
        pub const fn bwei(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Enable S-mode bus read/write transaction error local interrupt"]
        #[inline(always)]
        pub const fn set_bwei(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Enable S-mode performance monitor overflow local interrupt"]
        #[must_use]
        #[inline(always)]
        pub const fn pmovi(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Enable S-mode performance monitor overflow local interrupt"]
        #[inline(always)]
        pub const fn set_pmovi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Enable S-mode ECC DMR error local interrupt"]
        #[must_use]
        #[inline(always)]
        pub const fn imeccdmr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Enable S-mode ECC DMR error local interrupt"]
        #[inline(always)]
        pub const fn set_imeccdmr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Enable S-mode ACE error local interrupt"]
        #[must_use]
        #[inline(always)]
        pub const fn aceerr(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Enable S-mode ACE error local interrupt"]
        #[inline(always)]
        pub const fn set_aceerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Slie {
        #[inline(always)]
        fn default() -> Slie {
            Slie(0)
        }
    }
    impl core::fmt::Debug for Slie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Slie")
                .field("imecci", &self.imecci())
                .field("bwei", &self.bwei())
                .field("pmovi", &self.pmovi())
                .field("imeccdmr", &self.imeccdmr())
                .field("aceerr", &self.aceerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Slie {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Slie {{ imecci: {=bool:?}, bwei: {=bool:?}, pmovi: {=bool:?}, imeccdmr: {=bool:?}, aceerr: {=bool:?} }}",
                self.imecci(),
                self.bwei(),
                self.pmovi(),
                self.imeccdmr(),
                self.aceerr()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x9c4, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x9c4, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x9c4, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x9c4, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Slie {
        unsafe { Slie(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Slie) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Slie) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Enable S-mode slave-port ECC error local interrupt"]
    #[inline]
    pub unsafe fn set_imecci() {
        unsafe {
            _set(65536usize);
        }
    }
    #[doc = "Enable S-mode slave-port ECC error local interrupt"]
    #[inline]
    pub unsafe fn clear_imecci() {
        unsafe {
            _clear(65536usize);
        }
    }
    #[doc = "Enable S-mode bus read/write transaction error local interrupt"]
    #[inline]
    pub unsafe fn set_bwei() {
        unsafe {
            _set(131072usize);
        }
    }
    #[doc = "Enable S-mode bus read/write transaction error local interrupt"]
    #[inline]
    pub unsafe fn clear_bwei() {
        unsafe {
            _clear(131072usize);
        }
    }
    #[doc = "Enable S-mode performance monitor overflow local interrupt"]
    #[inline]
    pub unsafe fn set_pmovi() {
        unsafe {
            _set(262144usize);
        }
    }
    #[doc = "Enable S-mode performance monitor overflow local interrupt"]
    #[inline]
    pub unsafe fn clear_pmovi() {
        unsafe {
            _clear(262144usize);
        }
    }
    #[doc = "Enable S-mode ECC DMR error local interrupt"]
    #[inline]
    pub unsafe fn set_imeccdmr() {
        unsafe {
            _set(524288usize);
        }
    }
    #[doc = "Enable S-mode ECC DMR error local interrupt"]
    #[inline]
    pub unsafe fn clear_imeccdmr() {
        unsafe {
            _clear(524288usize);
        }
    }
    #[doc = "Enable S-mode ACE error local interrupt"]
    #[inline]
    pub unsafe fn set_aceerr() {
        unsafe {
            _set(16777216usize);
        }
    }
    #[doc = "Enable S-mode ACE error local interrupt"]
    #[inline]
    pub unsafe fn clear_aceerr() {
        unsafe {
            _clear(16777216usize);
        }
    }
}
#[doc = "Supervisor local interrupt pending"]
pub mod slip {
    #[doc = "Supervisor Local Interrupt Pending"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Slip(pub u32);
    impl Slip {
        #[must_use]
        #[inline(always)]
        pub const fn imecci(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_imecci(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn bwei(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_bwei(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pmovi(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pmovi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn imeccdmr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_imeccdmr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn aceerr(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_aceerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Slip {
        #[inline(always)]
        fn default() -> Slip {
            Slip(0)
        }
    }
    impl core::fmt::Debug for Slip {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Slip")
                .field("imecci", &self.imecci())
                .field("bwei", &self.bwei())
                .field("pmovi", &self.pmovi())
                .field("imeccdmr", &self.imeccdmr())
                .field("aceerr", &self.aceerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Slip {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Slip {{ imecci: {=bool:?}, bwei: {=bool:?}, pmovi: {=bool:?}, imeccdmr: {=bool:?}, aceerr: {=bool:?} }}",
                self.imecci(),
                self.bwei(),
                self.pmovi(),
                self.imeccdmr(),
                self.aceerr()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x9c5, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x9c5, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x9c5, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x9c5, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Slip {
        unsafe { Slip(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Slip) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Slip) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[inline]
    pub unsafe fn set_imecci() {
        unsafe {
            _set(65536usize);
        }
    }
    #[inline]
    pub unsafe fn clear_imecci() {
        unsafe {
            _clear(65536usize);
        }
    }
    #[inline]
    pub unsafe fn set_bwei() {
        unsafe {
            _set(131072usize);
        }
    }
    #[inline]
    pub unsafe fn clear_bwei() {
        unsafe {
            _clear(131072usize);
        }
    }
    #[inline]
    pub unsafe fn set_pmovi() {
        unsafe {
            _set(262144usize);
        }
    }
    #[inline]
    pub unsafe fn clear_pmovi() {
        unsafe {
            _clear(262144usize);
        }
    }
    #[inline]
    pub unsafe fn set_imeccdmr() {
        unsafe {
            _set(524288usize);
        }
    }
    #[inline]
    pub unsafe fn clear_imeccdmr() {
        unsafe {
            _clear(524288usize);
        }
    }
    #[inline]
    pub unsafe fn set_aceerr() {
        unsafe {
            _set(16777216usize);
        }
    }
    #[inline]
    pub unsafe fn clear_aceerr() {
        unsafe {
            _clear(16777216usize);
        }
    }
}
#[doc = "Detailed exception cause"]
pub mod sdcause {
    #[doc = "Supervisor Detailed Cause (for imprecise exception/interrupt)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdcause(pub u32);
    impl Sdcause {
        #[doc = "Detailed cause for imprecise exception"]
        #[must_use]
        #[inline(always)]
        pub const fn sdcause(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Detailed cause for imprecise exception"]
        #[inline(always)]
        pub const fn set_sdcause(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Privilege Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn pm(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Privilege Mode"]
        #[inline(always)]
        pub const fn set_pm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
    }
    impl Default for Sdcause {
        #[inline(always)]
        fn default() -> Sdcause {
            Sdcause(0)
        }
    }
    impl core::fmt::Debug for Sdcause {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sdcause")
                .field("sdcause", &self.sdcause())
                .field("pm", &self.pm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sdcause {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sdcause {{ sdcause: {=u8:?}, pm: {=u8:?} }}",
                self.sdcause(),
                self.pm()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x9c9, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x9c9, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x9c9, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x9c9, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Sdcause {
        unsafe { Sdcause(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Sdcause) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Sdcause) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Detailed cause for imprecise exception"]
    #[inline]
    pub unsafe fn set_sdcause(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(31usize << 0usize);
        bits |= (val as usize & 31usize) << 0usize;
        unsafe {
            _write(bits);
        }
    }
    #[doc = "Privilege Mode"]
    #[inline]
    pub unsafe fn set_pm(val: u8) {
        let mut bits = unsafe { _read() };
        bits &= !(3usize << 5usize);
        bits |= (val as usize & 3usize) << 5usize;
        unsafe {
            _write(bits);
        }
    }
}
#[doc = "CCTL data"]
pub mod scctldata {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x9cd, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x9cd, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x9cd, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x9cd, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "Counter overflow interrupt enable"]
pub mod scounterinten {
    #[doc = "Machine Counter Fields"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ScounterCommon(pub u32);
    impl ScounterCommon {
        #[doc = "Cycle counter"]
        #[must_use]
        #[inline(always)]
        pub const fn cy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cycle counter"]
        #[inline(always)]
        pub const fn set_cy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Instruction retired counter"]
        #[must_use]
        #[inline(always)]
        pub const fn ir(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction retired counter"]
        #[inline(always)]
        pub const fn set_ir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[must_use]
        #[inline(always)]
        pub const fn hpm(&self, n: usize) -> bool {
            assert!(n < 29usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[inline(always)]
        pub const fn set_hpm(&mut self, n: usize, val: bool) {
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
    impl core::fmt::Debug for ScounterCommon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ScounterCommon")
                .field("cy", &self.cy())
                .field("ir", &self.ir())
                .field("hpm[0]", &self.hpm(0usize))
                .field("hpm[1]", &self.hpm(1usize))
                .field("hpm[2]", &self.hpm(2usize))
                .field("hpm[3]", &self.hpm(3usize))
                .field("hpm[4]", &self.hpm(4usize))
                .field("hpm[5]", &self.hpm(5usize))
                .field("hpm[6]", &self.hpm(6usize))
                .field("hpm[7]", &self.hpm(7usize))
                .field("hpm[8]", &self.hpm(8usize))
                .field("hpm[9]", &self.hpm(9usize))
                .field("hpm[10]", &self.hpm(10usize))
                .field("hpm[11]", &self.hpm(11usize))
                .field("hpm[12]", &self.hpm(12usize))
                .field("hpm[13]", &self.hpm(13usize))
                .field("hpm[14]", &self.hpm(14usize))
                .field("hpm[15]", &self.hpm(15usize))
                .field("hpm[16]", &self.hpm(16usize))
                .field("hpm[17]", &self.hpm(17usize))
                .field("hpm[18]", &self.hpm(18usize))
                .field("hpm[19]", &self.hpm(19usize))
                .field("hpm[20]", &self.hpm(20usize))
                .field("hpm[21]", &self.hpm(21usize))
                .field("hpm[22]", &self.hpm(22usize))
                .field("hpm[23]", &self.hpm(23usize))
                .field("hpm[24]", &self.hpm(24usize))
                .field("hpm[25]", &self.hpm(25usize))
                .field("hpm[26]", &self.hpm(26usize))
                .field("hpm[27]", &self.hpm(27usize))
                .field("hpm[28]", &self.hpm(28usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ScounterCommon {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ScounterCommon {{ cy: {=bool:?}, ir: {=bool:?}, hpm[0]: {=bool:?}, hpm[1]: {=bool:?}, hpm[2]: {=bool:?}, hpm[3]: {=bool:?}, hpm[4]: {=bool:?}, hpm[5]: {=bool:?}, hpm[6]: {=bool:?}, hpm[7]: {=bool:?}, hpm[8]: {=bool:?}, hpm[9]: {=bool:?}, hpm[10]: {=bool:?}, hpm[11]: {=bool:?}, hpm[12]: {=bool:?}, hpm[13]: {=bool:?}, hpm[14]: {=bool:?}, hpm[15]: {=bool:?}, hpm[16]: {=bool:?}, hpm[17]: {=bool:?}, hpm[18]: {=bool:?}, hpm[19]: {=bool:?}, hpm[20]: {=bool:?}, hpm[21]: {=bool:?}, hpm[22]: {=bool:?}, hpm[23]: {=bool:?}, hpm[24]: {=bool:?}, hpm[25]: {=bool:?}, hpm[26]: {=bool:?}, hpm[27]: {=bool:?}, hpm[28]: {=bool:?} }}",
                self.cy(),
                self.ir(),
                self.hpm(0usize),
                self.hpm(1usize),
                self.hpm(2usize),
                self.hpm(3usize),
                self.hpm(4usize),
                self.hpm(5usize),
                self.hpm(6usize),
                self.hpm(7usize),
                self.hpm(8usize),
                self.hpm(9usize),
                self.hpm(10usize),
                self.hpm(11usize),
                self.hpm(12usize),
                self.hpm(13usize),
                self.hpm(14usize),
                self.hpm(15usize),
                self.hpm(16usize),
                self.hpm(17usize),
                self.hpm(18usize),
                self.hpm(19usize),
                self.hpm(20usize),
                self.hpm(21usize),
                self.hpm(22usize),
                self.hpm(23usize),
                self.hpm(24usize),
                self.hpm(25usize),
                self.hpm(26usize),
                self.hpm(27usize),
                self.hpm(28usize)
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x9cf, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x9cf, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x9cf, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x9cf, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> ScounterCommon {
        unsafe { ScounterCommon(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: ScounterCommon) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut ScounterCommon) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn set_cy() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn clear_cy() {
        unsafe {
            _clear(1usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn set_ir() {
        unsafe {
            _set(4usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn clear_ir() {
        unsafe {
            _clear(4usize);
        }
    }
}
#[doc = "Miscellaneous control"]
pub mod smisc_ctl {
    #[doc = "Supervisor Miscellaneous Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SmiscCtl(pub u32);
    impl SmiscCtl {
        #[doc = "ACE (Access Control Extension) Status"]
        #[must_use]
        #[inline(always)]
        pub const fn aces(&self) -> super::vals::Aces {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Aces::from_bits(val as u8)
        }
        #[doc = "ACE (Access Control Extension) Status"]
        #[inline(always)]
        pub const fn set_aces(&mut self, val: super::vals::Aces) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
    }
    impl Default for SmiscCtl {
        #[inline(always)]
        fn default() -> SmiscCtl {
            SmiscCtl(0)
        }
    }
    impl core::fmt::Debug for SmiscCtl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SmiscCtl")
                .field("aces", &self.aces())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SmiscCtl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SmiscCtl {{ aces: {:?} }}", self.aces())
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x9d0, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x9d0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x9d0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x9d0, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> SmiscCtl {
        unsafe { SmiscCtl(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: SmiscCtl) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut SmiscCtl) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "ACE (Access Control Extension) Status"]
    #[inline]
    pub unsafe fn set_aces(val: super::vals::Aces) {
        let mut bits = unsafe { _read() };
        bits &= !(3usize << 4usize);
        bits |= (val.to_bits() as usize & 3usize) << 4usize;
        unsafe {
            _write(bits);
        }
    }
}
#[doc = "Counter mask for M-mode"]
pub mod scountermask_m {
    #[doc = "Machine Counter Fields"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ScounterCommon(pub u32);
    impl ScounterCommon {
        #[doc = "Cycle counter"]
        #[must_use]
        #[inline(always)]
        pub const fn cy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cycle counter"]
        #[inline(always)]
        pub const fn set_cy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Instruction retired counter"]
        #[must_use]
        #[inline(always)]
        pub const fn ir(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction retired counter"]
        #[inline(always)]
        pub const fn set_ir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[must_use]
        #[inline(always)]
        pub const fn hpm(&self, n: usize) -> bool {
            assert!(n < 29usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[inline(always)]
        pub const fn set_hpm(&mut self, n: usize, val: bool) {
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
    impl core::fmt::Debug for ScounterCommon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ScounterCommon")
                .field("cy", &self.cy())
                .field("ir", &self.ir())
                .field("hpm[0]", &self.hpm(0usize))
                .field("hpm[1]", &self.hpm(1usize))
                .field("hpm[2]", &self.hpm(2usize))
                .field("hpm[3]", &self.hpm(3usize))
                .field("hpm[4]", &self.hpm(4usize))
                .field("hpm[5]", &self.hpm(5usize))
                .field("hpm[6]", &self.hpm(6usize))
                .field("hpm[7]", &self.hpm(7usize))
                .field("hpm[8]", &self.hpm(8usize))
                .field("hpm[9]", &self.hpm(9usize))
                .field("hpm[10]", &self.hpm(10usize))
                .field("hpm[11]", &self.hpm(11usize))
                .field("hpm[12]", &self.hpm(12usize))
                .field("hpm[13]", &self.hpm(13usize))
                .field("hpm[14]", &self.hpm(14usize))
                .field("hpm[15]", &self.hpm(15usize))
                .field("hpm[16]", &self.hpm(16usize))
                .field("hpm[17]", &self.hpm(17usize))
                .field("hpm[18]", &self.hpm(18usize))
                .field("hpm[19]", &self.hpm(19usize))
                .field("hpm[20]", &self.hpm(20usize))
                .field("hpm[21]", &self.hpm(21usize))
                .field("hpm[22]", &self.hpm(22usize))
                .field("hpm[23]", &self.hpm(23usize))
                .field("hpm[24]", &self.hpm(24usize))
                .field("hpm[25]", &self.hpm(25usize))
                .field("hpm[26]", &self.hpm(26usize))
                .field("hpm[27]", &self.hpm(27usize))
                .field("hpm[28]", &self.hpm(28usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ScounterCommon {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ScounterCommon {{ cy: {=bool:?}, ir: {=bool:?}, hpm[0]: {=bool:?}, hpm[1]: {=bool:?}, hpm[2]: {=bool:?}, hpm[3]: {=bool:?}, hpm[4]: {=bool:?}, hpm[5]: {=bool:?}, hpm[6]: {=bool:?}, hpm[7]: {=bool:?}, hpm[8]: {=bool:?}, hpm[9]: {=bool:?}, hpm[10]: {=bool:?}, hpm[11]: {=bool:?}, hpm[12]: {=bool:?}, hpm[13]: {=bool:?}, hpm[14]: {=bool:?}, hpm[15]: {=bool:?}, hpm[16]: {=bool:?}, hpm[17]: {=bool:?}, hpm[18]: {=bool:?}, hpm[19]: {=bool:?}, hpm[20]: {=bool:?}, hpm[21]: {=bool:?}, hpm[22]: {=bool:?}, hpm[23]: {=bool:?}, hpm[24]: {=bool:?}, hpm[25]: {=bool:?}, hpm[26]: {=bool:?}, hpm[27]: {=bool:?}, hpm[28]: {=bool:?} }}",
                self.cy(),
                self.ir(),
                self.hpm(0usize),
                self.hpm(1usize),
                self.hpm(2usize),
                self.hpm(3usize),
                self.hpm(4usize),
                self.hpm(5usize),
                self.hpm(6usize),
                self.hpm(7usize),
                self.hpm(8usize),
                self.hpm(9usize),
                self.hpm(10usize),
                self.hpm(11usize),
                self.hpm(12usize),
                self.hpm(13usize),
                self.hpm(14usize),
                self.hpm(15usize),
                self.hpm(16usize),
                self.hpm(17usize),
                self.hpm(18usize),
                self.hpm(19usize),
                self.hpm(20usize),
                self.hpm(21usize),
                self.hpm(22usize),
                self.hpm(23usize),
                self.hpm(24usize),
                self.hpm(25usize),
                self.hpm(26usize),
                self.hpm(27usize),
                self.hpm(28usize)
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x9d1, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x9d1, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x9d1, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x9d1, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> ScounterCommon {
        unsafe { ScounterCommon(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: ScounterCommon) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut ScounterCommon) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn set_cy() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn clear_cy() {
        unsafe {
            _clear(1usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn set_ir() {
        unsafe {
            _set(4usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn clear_ir() {
        unsafe {
            _clear(4usize);
        }
    }
}
#[doc = "Counter mask for S-mode"]
pub mod scountermask_s {
    #[doc = "Machine Counter Fields"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ScounterCommon(pub u32);
    impl ScounterCommon {
        #[doc = "Cycle counter"]
        #[must_use]
        #[inline(always)]
        pub const fn cy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cycle counter"]
        #[inline(always)]
        pub const fn set_cy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Instruction retired counter"]
        #[must_use]
        #[inline(always)]
        pub const fn ir(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction retired counter"]
        #[inline(always)]
        pub const fn set_ir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[must_use]
        #[inline(always)]
        pub const fn hpm(&self, n: usize) -> bool {
            assert!(n < 29usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[inline(always)]
        pub const fn set_hpm(&mut self, n: usize, val: bool) {
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
    impl core::fmt::Debug for ScounterCommon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ScounterCommon")
                .field("cy", &self.cy())
                .field("ir", &self.ir())
                .field("hpm[0]", &self.hpm(0usize))
                .field("hpm[1]", &self.hpm(1usize))
                .field("hpm[2]", &self.hpm(2usize))
                .field("hpm[3]", &self.hpm(3usize))
                .field("hpm[4]", &self.hpm(4usize))
                .field("hpm[5]", &self.hpm(5usize))
                .field("hpm[6]", &self.hpm(6usize))
                .field("hpm[7]", &self.hpm(7usize))
                .field("hpm[8]", &self.hpm(8usize))
                .field("hpm[9]", &self.hpm(9usize))
                .field("hpm[10]", &self.hpm(10usize))
                .field("hpm[11]", &self.hpm(11usize))
                .field("hpm[12]", &self.hpm(12usize))
                .field("hpm[13]", &self.hpm(13usize))
                .field("hpm[14]", &self.hpm(14usize))
                .field("hpm[15]", &self.hpm(15usize))
                .field("hpm[16]", &self.hpm(16usize))
                .field("hpm[17]", &self.hpm(17usize))
                .field("hpm[18]", &self.hpm(18usize))
                .field("hpm[19]", &self.hpm(19usize))
                .field("hpm[20]", &self.hpm(20usize))
                .field("hpm[21]", &self.hpm(21usize))
                .field("hpm[22]", &self.hpm(22usize))
                .field("hpm[23]", &self.hpm(23usize))
                .field("hpm[24]", &self.hpm(24usize))
                .field("hpm[25]", &self.hpm(25usize))
                .field("hpm[26]", &self.hpm(26usize))
                .field("hpm[27]", &self.hpm(27usize))
                .field("hpm[28]", &self.hpm(28usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ScounterCommon {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ScounterCommon {{ cy: {=bool:?}, ir: {=bool:?}, hpm[0]: {=bool:?}, hpm[1]: {=bool:?}, hpm[2]: {=bool:?}, hpm[3]: {=bool:?}, hpm[4]: {=bool:?}, hpm[5]: {=bool:?}, hpm[6]: {=bool:?}, hpm[7]: {=bool:?}, hpm[8]: {=bool:?}, hpm[9]: {=bool:?}, hpm[10]: {=bool:?}, hpm[11]: {=bool:?}, hpm[12]: {=bool:?}, hpm[13]: {=bool:?}, hpm[14]: {=bool:?}, hpm[15]: {=bool:?}, hpm[16]: {=bool:?}, hpm[17]: {=bool:?}, hpm[18]: {=bool:?}, hpm[19]: {=bool:?}, hpm[20]: {=bool:?}, hpm[21]: {=bool:?}, hpm[22]: {=bool:?}, hpm[23]: {=bool:?}, hpm[24]: {=bool:?}, hpm[25]: {=bool:?}, hpm[26]: {=bool:?}, hpm[27]: {=bool:?}, hpm[28]: {=bool:?} }}",
                self.cy(),
                self.ir(),
                self.hpm(0usize),
                self.hpm(1usize),
                self.hpm(2usize),
                self.hpm(3usize),
                self.hpm(4usize),
                self.hpm(5usize),
                self.hpm(6usize),
                self.hpm(7usize),
                self.hpm(8usize),
                self.hpm(9usize),
                self.hpm(10usize),
                self.hpm(11usize),
                self.hpm(12usize),
                self.hpm(13usize),
                self.hpm(14usize),
                self.hpm(15usize),
                self.hpm(16usize),
                self.hpm(17usize),
                self.hpm(18usize),
                self.hpm(19usize),
                self.hpm(20usize),
                self.hpm(21usize),
                self.hpm(22usize),
                self.hpm(23usize),
                self.hpm(24usize),
                self.hpm(25usize),
                self.hpm(26usize),
                self.hpm(27usize),
                self.hpm(28usize)
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x9d2, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x9d2, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x9d2, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x9d2, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> ScounterCommon {
        unsafe { ScounterCommon(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: ScounterCommon) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut ScounterCommon) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn set_cy() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn clear_cy() {
        unsafe {
            _clear(1usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn set_ir() {
        unsafe {
            _set(4usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn clear_ir() {
        unsafe {
            _clear(4usize);
        }
    }
}
#[doc = "Counter mask for U-mode"]
pub mod scountermask_u {
    #[doc = "Machine Counter Fields"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ScounterCommon(pub u32);
    impl ScounterCommon {
        #[doc = "Cycle counter"]
        #[must_use]
        #[inline(always)]
        pub const fn cy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cycle counter"]
        #[inline(always)]
        pub const fn set_cy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Instruction retired counter"]
        #[must_use]
        #[inline(always)]
        pub const fn ir(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction retired counter"]
        #[inline(always)]
        pub const fn set_ir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[must_use]
        #[inline(always)]
        pub const fn hpm(&self, n: usize) -> bool {
            assert!(n < 29usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[inline(always)]
        pub const fn set_hpm(&mut self, n: usize, val: bool) {
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
    impl core::fmt::Debug for ScounterCommon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ScounterCommon")
                .field("cy", &self.cy())
                .field("ir", &self.ir())
                .field("hpm[0]", &self.hpm(0usize))
                .field("hpm[1]", &self.hpm(1usize))
                .field("hpm[2]", &self.hpm(2usize))
                .field("hpm[3]", &self.hpm(3usize))
                .field("hpm[4]", &self.hpm(4usize))
                .field("hpm[5]", &self.hpm(5usize))
                .field("hpm[6]", &self.hpm(6usize))
                .field("hpm[7]", &self.hpm(7usize))
                .field("hpm[8]", &self.hpm(8usize))
                .field("hpm[9]", &self.hpm(9usize))
                .field("hpm[10]", &self.hpm(10usize))
                .field("hpm[11]", &self.hpm(11usize))
                .field("hpm[12]", &self.hpm(12usize))
                .field("hpm[13]", &self.hpm(13usize))
                .field("hpm[14]", &self.hpm(14usize))
                .field("hpm[15]", &self.hpm(15usize))
                .field("hpm[16]", &self.hpm(16usize))
                .field("hpm[17]", &self.hpm(17usize))
                .field("hpm[18]", &self.hpm(18usize))
                .field("hpm[19]", &self.hpm(19usize))
                .field("hpm[20]", &self.hpm(20usize))
                .field("hpm[21]", &self.hpm(21usize))
                .field("hpm[22]", &self.hpm(22usize))
                .field("hpm[23]", &self.hpm(23usize))
                .field("hpm[24]", &self.hpm(24usize))
                .field("hpm[25]", &self.hpm(25usize))
                .field("hpm[26]", &self.hpm(26usize))
                .field("hpm[27]", &self.hpm(27usize))
                .field("hpm[28]", &self.hpm(28usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ScounterCommon {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ScounterCommon {{ cy: {=bool:?}, ir: {=bool:?}, hpm[0]: {=bool:?}, hpm[1]: {=bool:?}, hpm[2]: {=bool:?}, hpm[3]: {=bool:?}, hpm[4]: {=bool:?}, hpm[5]: {=bool:?}, hpm[6]: {=bool:?}, hpm[7]: {=bool:?}, hpm[8]: {=bool:?}, hpm[9]: {=bool:?}, hpm[10]: {=bool:?}, hpm[11]: {=bool:?}, hpm[12]: {=bool:?}, hpm[13]: {=bool:?}, hpm[14]: {=bool:?}, hpm[15]: {=bool:?}, hpm[16]: {=bool:?}, hpm[17]: {=bool:?}, hpm[18]: {=bool:?}, hpm[19]: {=bool:?}, hpm[20]: {=bool:?}, hpm[21]: {=bool:?}, hpm[22]: {=bool:?}, hpm[23]: {=bool:?}, hpm[24]: {=bool:?}, hpm[25]: {=bool:?}, hpm[26]: {=bool:?}, hpm[27]: {=bool:?}, hpm[28]: {=bool:?} }}",
                self.cy(),
                self.ir(),
                self.hpm(0usize),
                self.hpm(1usize),
                self.hpm(2usize),
                self.hpm(3usize),
                self.hpm(4usize),
                self.hpm(5usize),
                self.hpm(6usize),
                self.hpm(7usize),
                self.hpm(8usize),
                self.hpm(9usize),
                self.hpm(10usize),
                self.hpm(11usize),
                self.hpm(12usize),
                self.hpm(13usize),
                self.hpm(14usize),
                self.hpm(15usize),
                self.hpm(16usize),
                self.hpm(17usize),
                self.hpm(18usize),
                self.hpm(19usize),
                self.hpm(20usize),
                self.hpm(21usize),
                self.hpm(22usize),
                self.hpm(23usize),
                self.hpm(24usize),
                self.hpm(25usize),
                self.hpm(26usize),
                self.hpm(27usize),
                self.hpm(28usize)
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x9d3, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x9d3, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x9d3, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x9d3, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> ScounterCommon {
        unsafe { ScounterCommon(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: ScounterCommon) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut ScounterCommon) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn set_cy() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn clear_cy() {
        unsafe {
            _clear(1usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn set_ir() {
        unsafe {
            _set(4usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn clear_ir() {
        unsafe {
            _clear(4usize);
        }
    }
}
#[doc = "Counter overflow status"]
pub mod scounterovf {
    #[doc = "Machine Counter Fields"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ScounterCommon(pub u32);
    impl ScounterCommon {
        #[doc = "Cycle counter"]
        #[must_use]
        #[inline(always)]
        pub const fn cy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cycle counter"]
        #[inline(always)]
        pub const fn set_cy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Instruction retired counter"]
        #[must_use]
        #[inline(always)]
        pub const fn ir(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction retired counter"]
        #[inline(always)]
        pub const fn set_ir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[must_use]
        #[inline(always)]
        pub const fn hpm(&self, n: usize) -> bool {
            assert!(n < 29usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[inline(always)]
        pub const fn set_hpm(&mut self, n: usize, val: bool) {
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
    impl core::fmt::Debug for ScounterCommon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ScounterCommon")
                .field("cy", &self.cy())
                .field("ir", &self.ir())
                .field("hpm[0]", &self.hpm(0usize))
                .field("hpm[1]", &self.hpm(1usize))
                .field("hpm[2]", &self.hpm(2usize))
                .field("hpm[3]", &self.hpm(3usize))
                .field("hpm[4]", &self.hpm(4usize))
                .field("hpm[5]", &self.hpm(5usize))
                .field("hpm[6]", &self.hpm(6usize))
                .field("hpm[7]", &self.hpm(7usize))
                .field("hpm[8]", &self.hpm(8usize))
                .field("hpm[9]", &self.hpm(9usize))
                .field("hpm[10]", &self.hpm(10usize))
                .field("hpm[11]", &self.hpm(11usize))
                .field("hpm[12]", &self.hpm(12usize))
                .field("hpm[13]", &self.hpm(13usize))
                .field("hpm[14]", &self.hpm(14usize))
                .field("hpm[15]", &self.hpm(15usize))
                .field("hpm[16]", &self.hpm(16usize))
                .field("hpm[17]", &self.hpm(17usize))
                .field("hpm[18]", &self.hpm(18usize))
                .field("hpm[19]", &self.hpm(19usize))
                .field("hpm[20]", &self.hpm(20usize))
                .field("hpm[21]", &self.hpm(21usize))
                .field("hpm[22]", &self.hpm(22usize))
                .field("hpm[23]", &self.hpm(23usize))
                .field("hpm[24]", &self.hpm(24usize))
                .field("hpm[25]", &self.hpm(25usize))
                .field("hpm[26]", &self.hpm(26usize))
                .field("hpm[27]", &self.hpm(27usize))
                .field("hpm[28]", &self.hpm(28usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ScounterCommon {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ScounterCommon {{ cy: {=bool:?}, ir: {=bool:?}, hpm[0]: {=bool:?}, hpm[1]: {=bool:?}, hpm[2]: {=bool:?}, hpm[3]: {=bool:?}, hpm[4]: {=bool:?}, hpm[5]: {=bool:?}, hpm[6]: {=bool:?}, hpm[7]: {=bool:?}, hpm[8]: {=bool:?}, hpm[9]: {=bool:?}, hpm[10]: {=bool:?}, hpm[11]: {=bool:?}, hpm[12]: {=bool:?}, hpm[13]: {=bool:?}, hpm[14]: {=bool:?}, hpm[15]: {=bool:?}, hpm[16]: {=bool:?}, hpm[17]: {=bool:?}, hpm[18]: {=bool:?}, hpm[19]: {=bool:?}, hpm[20]: {=bool:?}, hpm[21]: {=bool:?}, hpm[22]: {=bool:?}, hpm[23]: {=bool:?}, hpm[24]: {=bool:?}, hpm[25]: {=bool:?}, hpm[26]: {=bool:?}, hpm[27]: {=bool:?}, hpm[28]: {=bool:?} }}",
                self.cy(),
                self.ir(),
                self.hpm(0usize),
                self.hpm(1usize),
                self.hpm(2usize),
                self.hpm(3usize),
                self.hpm(4usize),
                self.hpm(5usize),
                self.hpm(6usize),
                self.hpm(7usize),
                self.hpm(8usize),
                self.hpm(9usize),
                self.hpm(10usize),
                self.hpm(11usize),
                self.hpm(12usize),
                self.hpm(13usize),
                self.hpm(14usize),
                self.hpm(15usize),
                self.hpm(16usize),
                self.hpm(17usize),
                self.hpm(18usize),
                self.hpm(19usize),
                self.hpm(20usize),
                self.hpm(21usize),
                self.hpm(22usize),
                self.hpm(23usize),
                self.hpm(24usize),
                self.hpm(25usize),
                self.hpm(26usize),
                self.hpm(27usize),
                self.hpm(28usize)
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x9d4, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x9d4, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x9d4, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x9d4, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> ScounterCommon {
        unsafe { ScounterCommon(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: ScounterCommon) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut ScounterCommon) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn set_cy() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn clear_cy() {
        unsafe {
            _clear(1usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn set_ir() {
        unsafe {
            _set(4usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn clear_ir() {
        unsafe {
            _clear(4usize);
        }
    }
}
#[doc = "Supervisor counter inhibit"]
pub mod scountinhibit {
    #[doc = "Machine Counter Fields"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ScounterCommon(pub u32);
    impl ScounterCommon {
        #[doc = "Cycle counter"]
        #[must_use]
        #[inline(always)]
        pub const fn cy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cycle counter"]
        #[inline(always)]
        pub const fn set_cy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Instruction retired counter"]
        #[must_use]
        #[inline(always)]
        pub const fn ir(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction retired counter"]
        #[inline(always)]
        pub const fn set_ir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[must_use]
        #[inline(always)]
        pub const fn hpm(&self, n: usize) -> bool {
            assert!(n < 29usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Hardware performance monitor 3 to 31"]
        #[inline(always)]
        pub const fn set_hpm(&mut self, n: usize, val: bool) {
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
    impl core::fmt::Debug for ScounterCommon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ScounterCommon")
                .field("cy", &self.cy())
                .field("ir", &self.ir())
                .field("hpm[0]", &self.hpm(0usize))
                .field("hpm[1]", &self.hpm(1usize))
                .field("hpm[2]", &self.hpm(2usize))
                .field("hpm[3]", &self.hpm(3usize))
                .field("hpm[4]", &self.hpm(4usize))
                .field("hpm[5]", &self.hpm(5usize))
                .field("hpm[6]", &self.hpm(6usize))
                .field("hpm[7]", &self.hpm(7usize))
                .field("hpm[8]", &self.hpm(8usize))
                .field("hpm[9]", &self.hpm(9usize))
                .field("hpm[10]", &self.hpm(10usize))
                .field("hpm[11]", &self.hpm(11usize))
                .field("hpm[12]", &self.hpm(12usize))
                .field("hpm[13]", &self.hpm(13usize))
                .field("hpm[14]", &self.hpm(14usize))
                .field("hpm[15]", &self.hpm(15usize))
                .field("hpm[16]", &self.hpm(16usize))
                .field("hpm[17]", &self.hpm(17usize))
                .field("hpm[18]", &self.hpm(18usize))
                .field("hpm[19]", &self.hpm(19usize))
                .field("hpm[20]", &self.hpm(20usize))
                .field("hpm[21]", &self.hpm(21usize))
                .field("hpm[22]", &self.hpm(22usize))
                .field("hpm[23]", &self.hpm(23usize))
                .field("hpm[24]", &self.hpm(24usize))
                .field("hpm[25]", &self.hpm(25usize))
                .field("hpm[26]", &self.hpm(26usize))
                .field("hpm[27]", &self.hpm(27usize))
                .field("hpm[28]", &self.hpm(28usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ScounterCommon {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ScounterCommon {{ cy: {=bool:?}, ir: {=bool:?}, hpm[0]: {=bool:?}, hpm[1]: {=bool:?}, hpm[2]: {=bool:?}, hpm[3]: {=bool:?}, hpm[4]: {=bool:?}, hpm[5]: {=bool:?}, hpm[6]: {=bool:?}, hpm[7]: {=bool:?}, hpm[8]: {=bool:?}, hpm[9]: {=bool:?}, hpm[10]: {=bool:?}, hpm[11]: {=bool:?}, hpm[12]: {=bool:?}, hpm[13]: {=bool:?}, hpm[14]: {=bool:?}, hpm[15]: {=bool:?}, hpm[16]: {=bool:?}, hpm[17]: {=bool:?}, hpm[18]: {=bool:?}, hpm[19]: {=bool:?}, hpm[20]: {=bool:?}, hpm[21]: {=bool:?}, hpm[22]: {=bool:?}, hpm[23]: {=bool:?}, hpm[24]: {=bool:?}, hpm[25]: {=bool:?}, hpm[26]: {=bool:?}, hpm[27]: {=bool:?}, hpm[28]: {=bool:?} }}",
                self.cy(),
                self.ir(),
                self.hpm(0usize),
                self.hpm(1usize),
                self.hpm(2usize),
                self.hpm(3usize),
                self.hpm(4usize),
                self.hpm(5usize),
                self.hpm(6usize),
                self.hpm(7usize),
                self.hpm(8usize),
                self.hpm(9usize),
                self.hpm(10usize),
                self.hpm(11usize),
                self.hpm(12usize),
                self.hpm(13usize),
                self.hpm(14usize),
                self.hpm(15usize),
                self.hpm(16usize),
                self.hpm(17usize),
                self.hpm(18usize),
                self.hpm(19usize),
                self.hpm(20usize),
                self.hpm(21usize),
                self.hpm(22usize),
                self.hpm(23usize),
                self.hpm(24usize),
                self.hpm(25usize),
                self.hpm(26usize),
                self.hpm(27usize),
                self.hpm(28usize)
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x9e0, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x9e0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x9e0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x9e0, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> ScounterCommon {
        unsafe { ScounterCommon(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: ScounterCommon) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut ScounterCommon) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn set_cy() {
        unsafe {
            _set(1usize);
        }
    }
    #[doc = "Cycle counter"]
    #[inline]
    pub unsafe fn clear_cy() {
        unsafe {
            _clear(1usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn set_ir() {
        unsafe {
            _set(4usize);
        }
    }
    #[doc = "Instruction retired counter"]
    #[inline]
    pub unsafe fn clear_ir() {
        unsafe {
            _clear(4usize);
        }
    }
}
#[doc = "Performance monitoring event selection"]
pub mod shpmevent3 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x9e3, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x9e3, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x9e3, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x9e3, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "Performance monitoring event selection"]
pub mod shpmevent4 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x9e4, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x9e4, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x9e4, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x9e4, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "Performance monitoring event selection"]
pub mod shpmevent5 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x9e5, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x9e5, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x9e5, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x9e5, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "Performance monitoring event selection"]
pub mod shpmevent6 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0x9e6, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0x9e6, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0x9e6, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0x9e6, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "PMA configuration"]
pub mod pmacfg0 {
    #[doc = "PMA Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmacfg(pub u32);
    impl Pmacfg {
        #[doc = "Entry address matching mode"]
        #[must_use]
        #[inline(always)]
        pub const fn etyp(&self, n: usize) -> super::vals::EntryType {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::EntryType::from_bits(val as u8)
        }
        #[doc = "Entry address matching mode"]
        #[inline(always)]
        pub const fn set_etyp(&mut self, n: usize, val: super::vals::EntryType) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Memory type attribute"]
        #[must_use]
        #[inline(always)]
        pub const fn mtyp(&self, n: usize) -> super::vals::MemoryType {
            assert!(n < 4usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x0f;
            super::vals::MemoryType::from_bits(val as u8)
        }
        #[doc = "Memory type attribute"]
        #[inline(always)]
        pub const fn set_mtyp(&mut self, n: usize, val: super::vals::MemoryType) {
            assert!(n < 4usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val.to_bits() as u32) & 0x0f) << offs);
        }
        #[doc = "Indicate whether Atomic Memory Operation instructions (including LR/SC) are not supported in this region"]
        #[must_use]
        #[inline(always)]
        pub const fn namo(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 6usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Indicate whether Atomic Memory Operation instructions (including LR/SC) are not supported in this region"]
        #[inline(always)]
        pub const fn set_namo(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 6usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Pmacfg {
        #[inline(always)]
        fn default() -> Pmacfg {
            Pmacfg(0)
        }
    }
    impl core::fmt::Debug for Pmacfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pmacfg")
                .field("etyp[0]", &self.etyp(0usize))
                .field("etyp[1]", &self.etyp(1usize))
                .field("etyp[2]", &self.etyp(2usize))
                .field("etyp[3]", &self.etyp(3usize))
                .field("mtyp[0]", &self.mtyp(0usize))
                .field("mtyp[1]", &self.mtyp(1usize))
                .field("mtyp[2]", &self.mtyp(2usize))
                .field("mtyp[3]", &self.mtyp(3usize))
                .field("namo[0]", &self.namo(0usize))
                .field("namo[1]", &self.namo(1usize))
                .field("namo[2]", &self.namo(2usize))
                .field("namo[3]", &self.namo(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pmacfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pmacfg {{ etyp[0]: {:?}, etyp[1]: {:?}, etyp[2]: {:?}, etyp[3]: {:?}, mtyp[0]: {:?}, mtyp[1]: {:?}, mtyp[2]: {:?}, mtyp[3]: {:?}, namo[0]: {=bool:?}, namo[1]: {=bool:?}, namo[2]: {=bool:?}, namo[3]: {=bool:?} }}",
                self.etyp(0usize),
                self.etyp(1usize),
                self.etyp(2usize),
                self.etyp(3usize),
                self.mtyp(0usize),
                self.mtyp(1usize),
                self.mtyp(2usize),
                self.mtyp(3usize),
                self.namo(0usize),
                self.namo(1usize),
                self.namo(2usize),
                self.namo(3usize)
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbc0, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbc0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbc0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbc0, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Pmacfg {
        unsafe { Pmacfg(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Pmacfg) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Pmacfg) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
}
#[doc = "PMA configuration"]
pub mod pmacfg1 {
    #[doc = "PMA Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmacfg(pub u32);
    impl Pmacfg {
        #[doc = "Entry address matching mode"]
        #[must_use]
        #[inline(always)]
        pub const fn etyp(&self, n: usize) -> super::vals::EntryType {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::EntryType::from_bits(val as u8)
        }
        #[doc = "Entry address matching mode"]
        #[inline(always)]
        pub const fn set_etyp(&mut self, n: usize, val: super::vals::EntryType) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Memory type attribute"]
        #[must_use]
        #[inline(always)]
        pub const fn mtyp(&self, n: usize) -> super::vals::MemoryType {
            assert!(n < 4usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x0f;
            super::vals::MemoryType::from_bits(val as u8)
        }
        #[doc = "Memory type attribute"]
        #[inline(always)]
        pub const fn set_mtyp(&mut self, n: usize, val: super::vals::MemoryType) {
            assert!(n < 4usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val.to_bits() as u32) & 0x0f) << offs);
        }
        #[doc = "Indicate whether Atomic Memory Operation instructions (including LR/SC) are not supported in this region"]
        #[must_use]
        #[inline(always)]
        pub const fn namo(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 6usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Indicate whether Atomic Memory Operation instructions (including LR/SC) are not supported in this region"]
        #[inline(always)]
        pub const fn set_namo(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 6usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Pmacfg {
        #[inline(always)]
        fn default() -> Pmacfg {
            Pmacfg(0)
        }
    }
    impl core::fmt::Debug for Pmacfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pmacfg")
                .field("etyp[0]", &self.etyp(0usize))
                .field("etyp[1]", &self.etyp(1usize))
                .field("etyp[2]", &self.etyp(2usize))
                .field("etyp[3]", &self.etyp(3usize))
                .field("mtyp[0]", &self.mtyp(0usize))
                .field("mtyp[1]", &self.mtyp(1usize))
                .field("mtyp[2]", &self.mtyp(2usize))
                .field("mtyp[3]", &self.mtyp(3usize))
                .field("namo[0]", &self.namo(0usize))
                .field("namo[1]", &self.namo(1usize))
                .field("namo[2]", &self.namo(2usize))
                .field("namo[3]", &self.namo(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pmacfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pmacfg {{ etyp[0]: {:?}, etyp[1]: {:?}, etyp[2]: {:?}, etyp[3]: {:?}, mtyp[0]: {:?}, mtyp[1]: {:?}, mtyp[2]: {:?}, mtyp[3]: {:?}, namo[0]: {=bool:?}, namo[1]: {=bool:?}, namo[2]: {=bool:?}, namo[3]: {=bool:?} }}",
                self.etyp(0usize),
                self.etyp(1usize),
                self.etyp(2usize),
                self.etyp(3usize),
                self.mtyp(0usize),
                self.mtyp(1usize),
                self.mtyp(2usize),
                self.mtyp(3usize),
                self.namo(0usize),
                self.namo(1usize),
                self.namo(2usize),
                self.namo(3usize)
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbc0, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbc0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbc0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbc0, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Pmacfg {
        unsafe { Pmacfg(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Pmacfg) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Pmacfg) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
}
#[doc = "PMA configuration"]
pub mod pmacfg2 {
    #[doc = "PMA Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmacfg(pub u32);
    impl Pmacfg {
        #[doc = "Entry address matching mode"]
        #[must_use]
        #[inline(always)]
        pub const fn etyp(&self, n: usize) -> super::vals::EntryType {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::EntryType::from_bits(val as u8)
        }
        #[doc = "Entry address matching mode"]
        #[inline(always)]
        pub const fn set_etyp(&mut self, n: usize, val: super::vals::EntryType) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Memory type attribute"]
        #[must_use]
        #[inline(always)]
        pub const fn mtyp(&self, n: usize) -> super::vals::MemoryType {
            assert!(n < 4usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x0f;
            super::vals::MemoryType::from_bits(val as u8)
        }
        #[doc = "Memory type attribute"]
        #[inline(always)]
        pub const fn set_mtyp(&mut self, n: usize, val: super::vals::MemoryType) {
            assert!(n < 4usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val.to_bits() as u32) & 0x0f) << offs);
        }
        #[doc = "Indicate whether Atomic Memory Operation instructions (including LR/SC) are not supported in this region"]
        #[must_use]
        #[inline(always)]
        pub const fn namo(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 6usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Indicate whether Atomic Memory Operation instructions (including LR/SC) are not supported in this region"]
        #[inline(always)]
        pub const fn set_namo(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 6usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Pmacfg {
        #[inline(always)]
        fn default() -> Pmacfg {
            Pmacfg(0)
        }
    }
    impl core::fmt::Debug for Pmacfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pmacfg")
                .field("etyp[0]", &self.etyp(0usize))
                .field("etyp[1]", &self.etyp(1usize))
                .field("etyp[2]", &self.etyp(2usize))
                .field("etyp[3]", &self.etyp(3usize))
                .field("mtyp[0]", &self.mtyp(0usize))
                .field("mtyp[1]", &self.mtyp(1usize))
                .field("mtyp[2]", &self.mtyp(2usize))
                .field("mtyp[3]", &self.mtyp(3usize))
                .field("namo[0]", &self.namo(0usize))
                .field("namo[1]", &self.namo(1usize))
                .field("namo[2]", &self.namo(2usize))
                .field("namo[3]", &self.namo(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pmacfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pmacfg {{ etyp[0]: {:?}, etyp[1]: {:?}, etyp[2]: {:?}, etyp[3]: {:?}, mtyp[0]: {:?}, mtyp[1]: {:?}, mtyp[2]: {:?}, mtyp[3]: {:?}, namo[0]: {=bool:?}, namo[1]: {=bool:?}, namo[2]: {=bool:?}, namo[3]: {=bool:?} }}",
                self.etyp(0usize),
                self.etyp(1usize),
                self.etyp(2usize),
                self.etyp(3usize),
                self.mtyp(0usize),
                self.mtyp(1usize),
                self.mtyp(2usize),
                self.mtyp(3usize),
                self.namo(0usize),
                self.namo(1usize),
                self.namo(2usize),
                self.namo(3usize)
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbc0, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbc0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbc0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbc0, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Pmacfg {
        unsafe { Pmacfg(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Pmacfg) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Pmacfg) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
}
#[doc = "PMA configuration"]
pub mod pmacfg3 {
    #[doc = "PMA Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmacfg(pub u32);
    impl Pmacfg {
        #[doc = "Entry address matching mode"]
        #[must_use]
        #[inline(always)]
        pub const fn etyp(&self, n: usize) -> super::vals::EntryType {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::EntryType::from_bits(val as u8)
        }
        #[doc = "Entry address matching mode"]
        #[inline(always)]
        pub const fn set_etyp(&mut self, n: usize, val: super::vals::EntryType) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Memory type attribute"]
        #[must_use]
        #[inline(always)]
        pub const fn mtyp(&self, n: usize) -> super::vals::MemoryType {
            assert!(n < 4usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x0f;
            super::vals::MemoryType::from_bits(val as u8)
        }
        #[doc = "Memory type attribute"]
        #[inline(always)]
        pub const fn set_mtyp(&mut self, n: usize, val: super::vals::MemoryType) {
            assert!(n < 4usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val.to_bits() as u32) & 0x0f) << offs);
        }
        #[doc = "Indicate whether Atomic Memory Operation instructions (including LR/SC) are not supported in this region"]
        #[must_use]
        #[inline(always)]
        pub const fn namo(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 6usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Indicate whether Atomic Memory Operation instructions (including LR/SC) are not supported in this region"]
        #[inline(always)]
        pub const fn set_namo(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 6usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Pmacfg {
        #[inline(always)]
        fn default() -> Pmacfg {
            Pmacfg(0)
        }
    }
    impl core::fmt::Debug for Pmacfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pmacfg")
                .field("etyp[0]", &self.etyp(0usize))
                .field("etyp[1]", &self.etyp(1usize))
                .field("etyp[2]", &self.etyp(2usize))
                .field("etyp[3]", &self.etyp(3usize))
                .field("mtyp[0]", &self.mtyp(0usize))
                .field("mtyp[1]", &self.mtyp(1usize))
                .field("mtyp[2]", &self.mtyp(2usize))
                .field("mtyp[3]", &self.mtyp(3usize))
                .field("namo[0]", &self.namo(0usize))
                .field("namo[1]", &self.namo(1usize))
                .field("namo[2]", &self.namo(2usize))
                .field("namo[3]", &self.namo(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pmacfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pmacfg {{ etyp[0]: {:?}, etyp[1]: {:?}, etyp[2]: {:?}, etyp[3]: {:?}, mtyp[0]: {:?}, mtyp[1]: {:?}, mtyp[2]: {:?}, mtyp[3]: {:?}, namo[0]: {=bool:?}, namo[1]: {=bool:?}, namo[2]: {=bool:?}, namo[3]: {=bool:?} }}",
                self.etyp(0usize),
                self.etyp(1usize),
                self.etyp(2usize),
                self.etyp(3usize),
                self.mtyp(0usize),
                self.mtyp(1usize),
                self.mtyp(2usize),
                self.mtyp(3usize),
                self.namo(0usize),
                self.namo(1usize),
                self.namo(2usize),
                self.namo(3usize)
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbc0, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbc0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbc0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbc0, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> Pmacfg {
        unsafe { Pmacfg(_read() as u32) }
    }
    #[doc = r" Write the CSR value."]
    #[inline]
    pub unsafe fn write(val: Pmacfg) {
        unsafe {
            _write(val.0 as usize);
        }
    }
    #[doc = r" Read-modify-write the CSR."]
    #[inline]
    pub unsafe fn modify<R>(f: impl FnOnce(&mut Pmacfg) -> R) -> R {
        let mut val = read();
        let res = f(&mut val);
        unsafe {
            write(val);
        }
        res
    }
}
#[doc = "PMA address"]
pub mod pmaaddr0 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbd0, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbd0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbd0, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbd0, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "PMA address"]
pub mod pmaaddr1 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbd1, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbd1, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbd1, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbd1, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "PMA address"]
pub mod pmaaddr2 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbd2, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbd2, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbd2, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbd2, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "PMA address"]
pub mod pmaaddr3 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbd3, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbd3, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbd3, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbd3, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "PMA address"]
pub mod pmaaddr4 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbd4, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbd4, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbd4, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbd4, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "PMA address"]
pub mod pmaaddr5 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbd5, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbd5, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbd5, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbd5, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "PMA address"]
pub mod pmaaddr6 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbd6, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbd6, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbd6, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbd6, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "PMA address"]
pub mod pmaaddr7 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbd7, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbd7, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbd7, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbd7, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "PMA address"]
pub mod pmaaddr8 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbd8, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbd8, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbd8, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbd8, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "PMA address"]
pub mod pmaaddr9 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbd9, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbd9, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbd9, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbd9, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "PMA address"]
pub mod pmaaddr10 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbda, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbda, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbda, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbda, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "PMA address"]
pub mod pmaaddr11 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbdb, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbdb, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbdb, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbdb, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "PMA address"]
pub mod pmaaddr12 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbdc, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbdc, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbdc, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbdc, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "PMA address"]
pub mod pmaaddr13 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbdd, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbdd, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbdd, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbdd, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "PMA address"]
pub mod pmaaddr14 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbde, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbde, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbde, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbde, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "PMA address"]
pub mod pmaaddr15 {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xbdf, x0" , out (reg) r);
        }
        r
    }
    #[inline(always)]
    unsafe fn _write(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrw x0, 0xbdf, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _set(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrs x0, 0xbdf, {0}" , in (reg) bits);
        }
    }
    #[inline(always)]
    unsafe fn _clear(bits: usize) {
        unsafe {
            core :: arch :: asm ! ("csrrc x0, 0xbdf, {0}" , in (reg) bits);
        }
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
    #[doc = r" Write the CSR value as raw usize."]
    #[inline]
    pub unsafe fn write(val: usize) {
        unsafe {
            _write(val);
        }
    }
}
#[doc = "Instruction cache/memory configuration"]
pub mod micm_cfg {
    #[doc = "Instruction Cache/Memory Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MicmCfg(pub u32);
    impl MicmCfg {
        #[must_use]
        #[inline(always)]
        pub const fn iset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_iset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn iway(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_iway(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn isz(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_isz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ilck(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ilck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ic_ecc(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ic_ecc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ilmb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ilmb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ilmsz(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ilmsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ulm_2bank(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ulm_2bank(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ilm_ecc(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ilm_ecc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ilm_xonly(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ilm_xonly(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn seth(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_seth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ic_repl(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ic_repl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
    }
    impl Default for MicmCfg {
        #[inline(always)]
        fn default() -> MicmCfg {
            MicmCfg(0)
        }
    }
    impl core::fmt::Debug for MicmCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MicmCfg")
                .field("iset", &self.iset())
                .field("iway", &self.iway())
                .field("isz", &self.isz())
                .field("ilck", &self.ilck())
                .field("ic_ecc", &self.ic_ecc())
                .field("ilmb", &self.ilmb())
                .field("ilmsz", &self.ilmsz())
                .field("ulm_2bank", &self.ulm_2bank())
                .field("ilm_ecc", &self.ilm_ecc())
                .field("ilm_xonly", &self.ilm_xonly())
                .field("seth", &self.seth())
                .field("ic_repl", &self.ic_repl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MicmCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MicmCfg {{ iset: {=u8:?}, iway: {=u8:?}, isz: {=u8:?}, ilck: {=bool:?}, ic_ecc: {=u8:?}, ilmb: {=u8:?}, ilmsz: {=u8:?}, ulm_2bank: {=bool:?}, ilm_ecc: {=u8:?}, ilm_xonly: {=bool:?}, seth: {=bool:?}, ic_repl: {=u8:?} }}",
                self.iset(),
                self.iway(),
                self.isz(),
                self.ilck(),
                self.ic_ecc(),
                self.ilmb(),
                self.ilmsz(),
                self.ulm_2bank(),
                self.ilm_ecc(),
                self.ilm_xonly(),
                self.seth(),
                self.ic_repl()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xfc0, x0" , out (reg) r);
        }
        r
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> MicmCfg {
        unsafe { MicmCfg(_read() as u32) }
    }
}
#[doc = "Data cache/memory configuration"]
pub mod mdcm_cfg {
    #[doc = "Data Cache/Memory Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MdcmCfg(pub u32);
    impl MdcmCfg {
        #[must_use]
        #[inline(always)]
        pub const fn dset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dway(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dway(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dsz(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dlck(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dlck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dc_ecc(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dc_ecc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dlmb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dlmb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dlmsz(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dlmsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ulm_2bank(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ulm_2bank(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dlm_ecc(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dlm_ecc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn seth(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_seth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dc_repl(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dc_repl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
    }
    impl Default for MdcmCfg {
        #[inline(always)]
        fn default() -> MdcmCfg {
            MdcmCfg(0)
        }
    }
    impl core::fmt::Debug for MdcmCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MdcmCfg")
                .field("dset", &self.dset())
                .field("dway", &self.dway())
                .field("dsz", &self.dsz())
                .field("dlck", &self.dlck())
                .field("dc_ecc", &self.dc_ecc())
                .field("dlmb", &self.dlmb())
                .field("dlmsz", &self.dlmsz())
                .field("ulm_2bank", &self.ulm_2bank())
                .field("dlm_ecc", &self.dlm_ecc())
                .field("seth", &self.seth())
                .field("dc_repl", &self.dc_repl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MdcmCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MdcmCfg {{ dset: {=u8:?}, dway: {=u8:?}, dsz: {=u8:?}, dlck: {=bool:?}, dc_ecc: {=u8:?}, dlmb: {=u8:?}, dlmsz: {=u8:?}, ulm_2bank: {=bool:?}, dlm_ecc: {=u8:?}, seth: {=bool:?}, dc_repl: {=u8:?} }}",
                self.dset(),
                self.dway(),
                self.dsz(),
                self.dlck(),
                self.dc_ecc(),
                self.dlmb(),
                self.dlmsz(),
                self.ulm_2bank(),
                self.dlm_ecc(),
                self.seth(),
                self.dc_repl()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xfc1, x0" , out (reg) r);
        }
        r
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> MdcmCfg {
        unsafe { MdcmCfg(_read() as u32) }
    }
}
#[doc = "Miscellaneous configuration"]
pub mod mmsc_cfg {
    #[doc = "Misc. Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmscCfg(pub u32);
    impl MmscCfg {
        #[doc = "Global configuration for parity/ECC support"]
        #[must_use]
        #[inline(always)]
        pub const fn ecc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Global configuration for parity/ECC support"]
        #[inline(always)]
        pub const fn set_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TLB parity/ECC support configuration"]
        #[must_use]
        #[inline(always)]
        pub const fn tlb_ecc(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "TLB parity/ECC support configuration"]
        #[inline(always)]
        pub const fn set_tlb_ecc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Indicates whether CodeDense extension is supported or not"]
        #[must_use]
        #[inline(always)]
        pub const fn ecd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates whether CodeDense extension is supported or not"]
        #[inline(always)]
        pub const fn set_ecd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Indicates if the PowerBrake (Performance Throttling) feature exists or not"]
        #[must_use]
        #[inline(always)]
        pub const fn pft(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if the PowerBrake (Performance Throttling) feature exists or not"]
        #[inline(always)]
        pub const fn set_pft(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Indicates if the HW Stack protection and recording mechanism exists or not"]
        #[must_use]
        #[inline(always)]
        pub const fn hsp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if the HW Stack protection and recording mechanism exists or not"]
        #[inline(always)]
        pub const fn set_hsp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Indicates if Andes Custom Extension exists or not"]
        #[must_use]
        #[inline(always)]
        pub const fn ace(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if Andes Custom Extension exists or not"]
        #[inline(always)]
        pub const fn set_ace(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Indicates the number of additional PM counters from counter3 to counter31"]
        #[must_use]
        #[inline(always)]
        pub const fn addpmc(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x1f;
            val as u8
        }
        #[doc = "Indicates the number of additional PM counters from counter3 to counter31"]
        #[inline(always)]
        pub const fn set_addpmc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 7usize)) | (((val as u32) & 0x1f) << 7usize);
        }
        #[doc = "Indicates if vectored PLIC mode is supported or not"]
        #[must_use]
        #[inline(always)]
        pub const fn vplic(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if vectored PLIC mode is supported or not"]
        #[inline(always)]
        pub const fn set_vplic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Indicates whether Andes V5 performance extension is supported or not"]
        #[must_use]
        #[inline(always)]
        pub const fn ev5pe(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates whether Andes V5 performance extension is supported or not"]
        #[inline(always)]
        pub const fn set_ev5pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Indicate if local memory slave port is present or not"]
        #[must_use]
        #[inline(always)]
        pub const fn lmslvp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Indicate if local memory slave port is present or not"]
        #[inline(always)]
        pub const fn set_lmslvp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Indicate if Andes-enhanced performance monitoring feature is present or not"]
        #[must_use]
        #[inline(always)]
        pub const fn pmnds(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Indicate if Andes-enhanced performance monitoring feature is present or not"]
        #[inline(always)]
        pub const fn set_pmnds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Indicate the presence of CSRs for CCTL operations"]
        #[must_use]
        #[inline(always)]
        pub const fn cctlcsr(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Indicate the presence of CSRs for CCTL operations"]
        #[inline(always)]
        pub const fn set_cctlcsr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Indicate the support of FLHW and FSHW instructions"]
        #[must_use]
        #[inline(always)]
        pub const fn efhw(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Indicate the support of FLHW and FSHW instructions"]
        #[inline(always)]
        pub const fn set_efhw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Indicates the version number of CCTL command operation scheme supported by an implementation"]
        #[must_use]
        #[inline(always)]
        pub const fn vcctl(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "Indicates the version number of CCTL command operation scheme supported by an implementation"]
        #[inline(always)]
        pub const fn set_vcctl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "Exception additional save levels for recoverable exception/NMI"]
        #[must_use]
        #[inline(always)]
        pub const fn excslvl(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Exception additional save levels for recoverable exception/NMI"]
        #[inline(always)]
        pub const fn set_excslvl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "Indicates if Performance Monitoring Counters are implemented or not"]
        #[must_use]
        #[inline(always)]
        pub const fn nopmc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if Performance Monitoring Counters are implemented or not"]
        #[inline(always)]
        pub const fn set_nopmc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Indicates if the stack overflow/underflow exception is implemented as a “before” precise exception or an “after” precise exception"]
        #[must_use]
        #[inline(always)]
        pub const fn spe_aft(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if the stack overflow/underflow exception is implemented as a “before” precise exception or an “after” precise exception"]
        #[inline(always)]
        pub const fn set_spe_aft(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Indicates if wfe, sleepvalue, txevt CSRs are implemented or not"]
        #[must_use]
        #[inline(always)]
        pub const fn esleep(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if wfe, sleepvalue, txevt CSRs are implemented or not"]
        #[inline(always)]
        pub const fn set_esleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Indicates if PPI region is supported or not"]
        #[must_use]
        #[inline(always)]
        pub const fn ppi(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if PPI region is supported or not"]
        #[inline(always)]
        pub const fn set_ppi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Indicates if FIO region is supported or not"]
        #[must_use]
        #[inline(always)]
        pub const fn fio(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if FIO region is supported or not"]
        #[inline(always)]
        pub const fn set_fio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Indicates if CLIC (Core Local Interrupt Controller) is supported or not"]
        #[must_use]
        #[inline(always)]
        pub const fn clic(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if CLIC (Core Local Interrupt Controller) is supported or not"]
        #[inline(always)]
        pub const fn set_clic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Indicates if Enhnaced CLIC (Core Local Interrupt Controller) extension is supported or not"]
        #[must_use]
        #[inline(always)]
        pub const fn eclic(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if Enhnaced CLIC (Core Local Interrupt Controller) extension is supported or not"]
        #[inline(always)]
        pub const fn set_eclic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Indicates if DSP extension is supported or not"]
        #[must_use]
        #[inline(always)]
        pub const fn edsp(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if DSP extension is supported or not"]
        #[inline(always)]
        pub const fn set_edsp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Indicates if programmable PMA mechanism with PMA region CSRs is supported or not"]
        #[must_use]
        #[inline(always)]
        pub const fn ppma(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if programmable PMA mechanism with PMA region CSRs is supported or not"]
        #[inline(always)]
        pub const fn set_ppma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Indicates if mmsc_cfg2 CSR is present or not"]
        #[must_use]
        #[inline(always)]
        pub const fn msc_ext(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if mmsc_cfg2 CSR is present or not"]
        #[inline(always)]
        pub const fn set_msc_ext(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MmscCfg {
        #[inline(always)]
        fn default() -> MmscCfg {
            MmscCfg(0)
        }
    }
    impl core::fmt::Debug for MmscCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmscCfg")
                .field("ecc", &self.ecc())
                .field("tlb_ecc", &self.tlb_ecc())
                .field("ecd", &self.ecd())
                .field("pft", &self.pft())
                .field("hsp", &self.hsp())
                .field("ace", &self.ace())
                .field("addpmc", &self.addpmc())
                .field("vplic", &self.vplic())
                .field("ev5pe", &self.ev5pe())
                .field("lmslvp", &self.lmslvp())
                .field("pmnds", &self.pmnds())
                .field("cctlcsr", &self.cctlcsr())
                .field("efhw", &self.efhw())
                .field("vcctl", &self.vcctl())
                .field("excslvl", &self.excslvl())
                .field("nopmc", &self.nopmc())
                .field("spe_aft", &self.spe_aft())
                .field("esleep", &self.esleep())
                .field("ppi", &self.ppi())
                .field("fio", &self.fio())
                .field("clic", &self.clic())
                .field("eclic", &self.eclic())
                .field("edsp", &self.edsp())
                .field("ppma", &self.ppma())
                .field("msc_ext", &self.msc_ext())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmscCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MmscCfg {{ ecc: {=bool:?}, tlb_ecc: {=u8:?}, ecd: {=bool:?}, pft: {=bool:?}, hsp: {=bool:?}, ace: {=bool:?}, addpmc: {=u8:?}, vplic: {=bool:?}, ev5pe: {=bool:?}, lmslvp: {=bool:?}, pmnds: {=bool:?}, cctlcsr: {=bool:?}, efhw: {=bool:?}, vcctl: {=u8:?}, excslvl: {=u8:?}, nopmc: {=bool:?}, spe_aft: {=bool:?}, esleep: {=bool:?}, ppi: {=bool:?}, fio: {=bool:?}, clic: {=bool:?}, eclic: {=bool:?}, edsp: {=bool:?}, ppma: {=bool:?}, msc_ext: {=bool:?} }}",
                self.ecc(),
                self.tlb_ecc(),
                self.ecd(),
                self.pft(),
                self.hsp(),
                self.ace(),
                self.addpmc(),
                self.vplic(),
                self.ev5pe(),
                self.lmslvp(),
                self.pmnds(),
                self.cctlcsr(),
                self.efhw(),
                self.vcctl(),
                self.excslvl(),
                self.nopmc(),
                self.spe_aft(),
                self.esleep(),
                self.ppi(),
                self.fio(),
                self.clic(),
                self.eclic(),
                self.edsp(),
                self.ppma(),
                self.msc_ext()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xfc2, x0" , out (reg) r);
        }
        r
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> MmscCfg {
        unsafe { MmscCfg(_read() as u32) }
    }
}
#[doc = "Miscellaneous configuration (RV32)"]
pub mod mmsc_cfg2 {
    #[doc = "Misc. Configuration 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmscCfg2(pub u32);
    impl MmscCfg2 {
        #[doc = "Indicates if BFLOAT16 conversion extension is supported or not"]
        #[must_use]
        #[inline(always)]
        pub const fn bf16cvt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if BFLOAT16 conversion extension is supported or not"]
        #[inline(always)]
        pub const fn set_bf16cvt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Indicates if FP16 half-precision floating-point extension (Zfh) is supported or not"]
        #[must_use]
        #[inline(always)]
        pub const fn zfh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if FP16 half-precision floating-point extension (Zfh) is supported or not"]
        #[inline(always)]
        pub const fn set_zfh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Indicates if vector INT4 load extension is supported or not"]
        #[must_use]
        #[inline(always)]
        pub const fn vl4(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if vector INT4 load extension is supported or not"]
        #[inline(always)]
        pub const fn set_vl4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Indicates if crash debugging state save feature is supported or not"]
        #[must_use]
        #[inline(always)]
        pub const fn crashsave(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if crash debugging state save feature is supported or not"]
        #[inline(always)]
        pub const fn set_crashsave(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Indicates if mvec_cfg CSR is present or not"]
        #[must_use]
        #[inline(always)]
        pub const fn veccfg(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if mvec_cfg CSR is present or not"]
        #[inline(always)]
        pub const fn set_veccfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Indicates if scalar FPU is implemented using VPU-like pipeline"]
        #[must_use]
        #[inline(always)]
        pub const fn finv(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if scalar FPU is implemented using VPU-like pipeline"]
        #[inline(always)]
        pub const fn set_finv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Indicates if 16-bit push/pop/popret instructions are supported or not"]
        #[must_use]
        #[inline(always)]
        pub const fn pp16(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if 16-bit push/pop/popret instructions are supported or not"]
        #[inline(always)]
        pub const fn set_pp16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Indicates if vector small INT handling extension is supported or not"]
        #[must_use]
        #[inline(always)]
        pub const fn vsih(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if vector small INT handling extension is supported or not"]
        #[inline(always)]
        pub const fn set_vsih(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Version of Code Dense Extension"]
        #[must_use]
        #[inline(always)]
        pub const fn ecdv(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "Version of Code Dense Extension"]
        #[inline(always)]
        pub const fn set_ecdv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "Indicates if vector dot product extension is supported or not"]
        #[must_use]
        #[inline(always)]
        pub const fn vdot(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if vector dot product extension is supported or not"]
        #[inline(always)]
        pub const fn set_vdot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Indicates if vector packed FP16 extension is supported or not. VFPMADT.VF and VFPMADB.VF"]
        #[must_use]
        #[inline(always)]
        pub const fn vpfh(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if vector packed FP16 extension is supported or not. VFPMADT.VF and VFPMADB.VF"]
        #[inline(always)]
        pub const fn set_vpfh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Indicates if CCACHE/MP configuration info is implemented or not. It includes the fields of CCACHE, IO_COHP, CORE_PCLUS, and mccache_ctl_base CSR"]
        #[must_use]
        #[inline(always)]
        pub const fn ccachemp_cfg(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if CCACHE/MP configuration info is implemented or not. It includes the fields of CCACHE, IO_COHP, CORE_PCLUS, and mccache_ctl_base CSR"]
        #[inline(always)]
        pub const fn set_ccachemp_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Indicates if cluster cache is present or not"]
        #[must_use]
        #[inline(always)]
        pub const fn ccache(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if cluster cache is present or not"]
        #[inline(always)]
        pub const fn set_ccache(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Indicates if IO coherence port is present or not"]
        #[must_use]
        #[inline(always)]
        pub const fn io_cohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if IO coherence port is present or not"]
        #[inline(always)]
        pub const fn set_io_cohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Indicates the number of cores in a MP cluster. The number of cores is equal to “CORE_PCLUS+1”"]
        #[must_use]
        #[inline(always)]
        pub const fn core_pclus(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Indicates the number of cores in a MP cluster. The number of cores is equal to “CORE_PCLUS+1”"]
        #[inline(always)]
        pub const fn set_core_pclus(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Indicates if mrvarch_cfg CSR is present or not"]
        #[must_use]
        #[inline(always)]
        pub const fn rvarch(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if mrvarch_cfg CSR is present or not"]
        #[inline(always)]
        pub const fn set_rvarch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for MmscCfg2 {
        #[inline(always)]
        fn default() -> MmscCfg2 {
            MmscCfg2(0)
        }
    }
    impl core::fmt::Debug for MmscCfg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmscCfg2")
                .field("bf16cvt", &self.bf16cvt())
                .field("zfh", &self.zfh())
                .field("vl4", &self.vl4())
                .field("crashsave", &self.crashsave())
                .field("veccfg", &self.veccfg())
                .field("finv", &self.finv())
                .field("pp16", &self.pp16())
                .field("vsih", &self.vsih())
                .field("ecdv", &self.ecdv())
                .field("vdot", &self.vdot())
                .field("vpfh", &self.vpfh())
                .field("ccachemp_cfg", &self.ccachemp_cfg())
                .field("ccache", &self.ccache())
                .field("io_cohp", &self.io_cohp())
                .field("core_pclus", &self.core_pclus())
                .field("rvarch", &self.rvarch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmscCfg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MmscCfg2 {{ bf16cvt: {=bool:?}, zfh: {=bool:?}, vl4: {=bool:?}, crashsave: {=bool:?}, veccfg: {=bool:?}, finv: {=bool:?}, pp16: {=bool:?}, vsih: {=bool:?}, ecdv: {=u8:?}, vdot: {=bool:?}, vpfh: {=bool:?}, ccachemp_cfg: {=bool:?}, ccache: {=bool:?}, io_cohp: {=bool:?}, core_pclus: {=u8:?}, rvarch: {=bool:?} }}",
                self.bf16cvt(),
                self.zfh(),
                self.vl4(),
                self.crashsave(),
                self.veccfg(),
                self.finv(),
                self.pp16(),
                self.vsih(),
                self.ecdv(),
                self.vdot(),
                self.vpfh(),
                self.ccachemp_cfg(),
                self.ccache(),
                self.io_cohp(),
                self.core_pclus(),
                self.rvarch()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xfc3, x0" , out (reg) r);
        }
        r
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> MmscCfg2 {
        unsafe { MmscCfg2(_read() as u32) }
    }
}
#[doc = "Vector processor configuration"]
pub mod mvec_cfg {
    #[doc = "Vector Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MvecCfg(pub u32);
    impl MvecCfg {
        #[must_use]
        #[inline(always)]
        pub const fn minor(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_minor(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn major(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_major(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dw(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_dw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mw(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn misew(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_misew(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn mfsew(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_mfsew(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for MvecCfg {
        #[inline(always)]
        fn default() -> MvecCfg {
            MvecCfg(0)
        }
    }
    impl core::fmt::Debug for MvecCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MvecCfg")
                .field("minor", &self.minor())
                .field("major", &self.major())
                .field("dw", &self.dw())
                .field("mw", &self.mw())
                .field("misew", &self.misew())
                .field("mfsew", &self.mfsew())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MvecCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MvecCfg {{ minor: {=u8:?}, major: {=u8:?}, dw: {=u8:?}, mw: {=u8:?}, misew: {=u8:?}, mfsew: {=u8:?} }}",
                self.minor(),
                self.major(),
                self.dw(),
                self.mw(),
                self.misew(),
                self.mfsew()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xfc7, x0" , out (reg) r);
        }
        r
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> MvecCfg {
        unsafe { MvecCfg(_read() as u32) }
    }
}
#[doc = "Current state save for crash debugging"]
pub mod mcrash_statesave {
    #[doc = "Machine Crash State Save"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McrashStatesave(pub u32);
    impl McrashStatesave {
        #[must_use]
        #[inline(always)]
        pub const fn mie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_mie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn cp(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_cp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ppft_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ppft_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pime(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pime(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn pdme(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_pdme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ptyp(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for McrashStatesave {
        #[inline(always)]
        fn default() -> McrashStatesave {
            McrashStatesave(0)
        }
    }
    impl core::fmt::Debug for McrashStatesave {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("McrashStatesave")
                .field("mie", &self.mie())
                .field("cp", &self.cp())
                .field("ppft_en", &self.ppft_en())
                .field("pime", &self.pime())
                .field("pdme", &self.pdme())
                .field("ptyp", &self.ptyp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for McrashStatesave {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "McrashStatesave {{ mie: {=bool:?}, cp: {=u8:?}, ppft_en: {=bool:?}, pime: {=bool:?}, pdme: {=bool:?}, ptyp: {=u8:?} }}",
                self.mie(),
                self.cp(),
                self.ppft_en(),
                self.pime(),
                self.pdme(),
                self.ptyp()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xfc8, x0" , out (reg) r);
        }
        r
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> McrashStatesave {
        unsafe { McrashStatesave(_read() as u32) }
    }
}
#[doc = "mstatus state save for crash debugging"]
pub mod mstatus_crashsave {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xfc9, x0" , out (reg) r);
        }
        r
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
}
#[doc = "RISC-V Architecture"]
pub mod mrvarch_cfg {
    #[doc = "RISC-V Architecture Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MrvarchCfg(pub u32);
    impl MrvarchCfg {
        #[must_use]
        #[inline(always)]
        pub const fn zba(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_zba(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn zbb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_zbb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn zbc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_zbc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn zbs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_zbs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn smepmp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_smepmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn svinval(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_svinval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn smstateen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_smstateen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sscofmpf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sscofmpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sstc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sstc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn zicbom(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_zicbom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn zicbop(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_zicbop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn zicboz(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_zicboz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn zbk(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_zbk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn zkn(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_zkn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn zks(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_zks(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn zkt(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_zkt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn zkr(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_zkr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sm_verion(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_sm_verion(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ss_version(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ss_version(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn svpbmt(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_svpbmt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn svnapot(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_svnapot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn zihintpause(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_zihintpause(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for MrvarchCfg {
        #[inline(always)]
        fn default() -> MrvarchCfg {
            MrvarchCfg(0)
        }
    }
    impl core::fmt::Debug for MrvarchCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MrvarchCfg")
                .field("zba", &self.zba())
                .field("zbb", &self.zbb())
                .field("zbc", &self.zbc())
                .field("zbs", &self.zbs())
                .field("smepmp", &self.smepmp())
                .field("svinval", &self.svinval())
                .field("smstateen", &self.smstateen())
                .field("sscofmpf", &self.sscofmpf())
                .field("sstc", &self.sstc())
                .field("zicbom", &self.zicbom())
                .field("zicbop", &self.zicbop())
                .field("zicboz", &self.zicboz())
                .field("zbk", &self.zbk())
                .field("zkn", &self.zkn())
                .field("zks", &self.zks())
                .field("zkt", &self.zkt())
                .field("zkr", &self.zkr())
                .field("sm_verion", &self.sm_verion())
                .field("ss_version", &self.ss_version())
                .field("svpbmt", &self.svpbmt())
                .field("svnapot", &self.svnapot())
                .field("zihintpause", &self.zihintpause())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MrvarchCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MrvarchCfg {{ zba: {=bool:?}, zbb: {=bool:?}, zbc: {=bool:?}, zbs: {=bool:?}, smepmp: {=bool:?}, svinval: {=bool:?}, smstateen: {=bool:?}, sscofmpf: {=bool:?}, sstc: {=bool:?}, zicbom: {=bool:?}, zicbop: {=bool:?}, zicboz: {=bool:?}, zbk: {=bool:?}, zkn: {=bool:?}, zks: {=bool:?}, zkt: {=bool:?}, zkr: {=bool:?}, sm_verion: {=u8:?}, ss_version: {=u8:?}, svpbmt: {=bool:?}, svnapot: {=bool:?}, zihintpause: {=bool:?} }}",
                self.zba(),
                self.zbb(),
                self.zbc(),
                self.zbs(),
                self.smepmp(),
                self.svinval(),
                self.smstateen(),
                self.sscofmpf(),
                self.sstc(),
                self.zicbom(),
                self.zicbop(),
                self.zicboz(),
                self.zbk(),
                self.zkn(),
                self.zks(),
                self.zkt(),
                self.zkr(),
                self.sm_verion(),
                self.ss_version(),
                self.svpbmt(),
                self.svnapot(),
                self.zihintpause()
            )
        }
    }
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xfca, x0" , out (reg) r);
        }
        r
    }
    #[doc = r" Read the CSR value."]
    #[inline]
    pub fn read() -> MrvarchCfg {
        unsafe { MrvarchCfg(_read() as u32) }
    }
}
#[doc = "Cluster cache control base address"]
pub mod mccache_ctl_base {
    #[inline(always)]
    unsafe fn _read() -> usize {
        let r: usize;
        unsafe {
            core :: arch :: asm ! ("csrrs {0}, 0xfcf, x0" , out (reg) r);
        }
        r
    }
    #[doc = r" Read the CSR value as raw usize."]
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
}
pub mod vals {
    #[repr(u8)]
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Aces {
        Off = 0x0,
        Initial = 0x01,
        Clean = 0x02,
        Dirty = 0x03,
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
    #[repr(u8)]
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum EntryType {
        #[doc = "This PMA entry is disabled"]
        Off = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "Naturally aligned power-of-2 region. The granularity is 4K bytes."]
        Napot = 0x03,
    }
    impl EntryType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> EntryType {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for EntryType {
        #[inline(always)]
        fn from(val: u8) -> EntryType {
            EntryType::from_bits(val)
        }
    }
    impl From<EntryType> for u8 {
        #[inline(always)]
        fn from(val: EntryType) -> u8 {
            EntryType::to_bits(val)
        }
    }
    #[repr(u8)]
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum MemoryType {
        #[doc = "Device memory, non-bufferable"]
        DevNonBuf = 0x0,
        #[doc = "Device memory, bufferable"]
        DevBuf = 0x01,
        #[doc = "Normal memory, non-cacheable, non-bufferable"]
        MemNonCacheNonBuf = 0x02,
        #[doc = "Normal memory, non-cacheable, bufferable"]
        MemNonCacheBuf = 0x03,
        #[doc = "Normal memory, write-through, no allocate"]
        MemWtNoAlloc = 0x04,
        #[doc = "Normal memory, write-through, read allocate"]
        MemWtReadAlloc = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Normal memory, write-back, no allocate"]
        MemWbNoAlloc = 0x08,
        #[doc = "Normal memory, write-back, read allocate"]
        MemWbReadAlloc = 0x09,
        #[doc = "Normal memory, write-back, write allocate"]
        MemWbWriteAlloc = 0x0a,
        #[doc = "Normal memory, write-back, read and write allocate"]
        MemWbReadWriteAlloc = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        #[doc = "Empty hole"]
        EmptyHole = 0x0f,
    }
    impl MemoryType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> MemoryType {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for MemoryType {
        #[inline(always)]
        fn from(val: u8) -> MemoryType {
            MemoryType::from_bits(val)
        }
    }
    impl From<MemoryType> for u8 {
        #[inline(always)]
        fn from(val: MemoryType) -> u8 {
            MemoryType::to_bits(val)
        }
    }
}
