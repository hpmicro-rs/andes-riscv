#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PLIC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plic {
    ptr: *mut u8,
}
unsafe impl Send for Plic {}
unsafe impl Sync for Plic {}
impl Plic {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Feature enable register."]
    #[inline(always)]
    pub const fn feature(self) -> self::common::Reg<regs::Feature, self::common::RW> {
        unsafe { self::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn priority(self, n: usize) -> self::common::Reg<regs::Priority, self::common::RW> {
        assert!(n < 127usize);
        unsafe { self::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn pending(self, n: usize) -> self::common::Reg<regs::Pending, self::common::RW> {
        assert!(n < 4usize);
        unsafe { self::common::Reg::from_ptr(self.ptr.add(0x1000usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn trigger(self, n: usize) -> self::common::Reg<regs::Trigger, self::common::RW> {
        assert!(n < 4usize);
        unsafe { self::common::Reg::from_ptr(self.ptr.add(0x1080usize + n * 4usize) as _) }
    }
    #[doc = "Number of supported interrupt sources and targets."]
    #[inline(always)]
    pub const fn number(self) -> self::common::Reg<regs::Number, self::common::RW> {
        unsafe { self::common::Reg::from_ptr(self.ptr.add(0x1100usize) as _) }
    }
    #[doc = "Version and the maximum priority."]
    #[inline(always)]
    pub const fn info(self) -> self::common::Reg<regs::Info, self::common::RW> {
        unsafe { self::common::Reg::from_ptr(self.ptr.add(0x1104usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn targetint(self, n: usize) -> Targetint {
        assert!(n < 2usize);
        unsafe { Targetint::from_ptr(self.ptr.add(0x2000usize + n * 128usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn targetconfig(self, n: usize) -> Targetconfig {
        assert!(n < 2usize);
        unsafe { Targetconfig::from_ptr(self.ptr.add(0x0020_0000usize + n * 4096usize) as _) }
    }
}

#[doc = "PLICSW, Platform Level Software Interrupt Controller. Compatible with PLIC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plicsw {
    ptr: *mut u8,
}
unsafe impl Send for Plicsw {}
unsafe impl Sync for Plicsw {}
impl Plicsw {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Pending status."]
    #[inline(always)]
    pub const fn pending(self) -> self::common::Reg<regs::Pending, self::common::RW> {
        unsafe { self::common::Reg::from_ptr(self.ptr.add(0x1000usize) as _) }
    }
    #[doc = "Interrupt enable."]
    #[inline(always)]
    pub const fn inten(self) -> self::common::Reg<regs::Inten, self::common::RW> {
        unsafe { self::common::Reg::from_ptr(self.ptr.add(0x2000usize) as _) }
    }
    #[doc = "Claim and complete."]
    #[inline(always)]
    pub const fn claim(self) -> self::common::Reg<regs::Claim, self::common::RW> {
        unsafe { self::common::Reg::from_ptr(self.ptr.add(0x0020_0004usize) as _) }
    }
}

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Targetconfig {
    ptr: *mut u8,
}
unsafe impl Send for Targetconfig {}
unsafe impl Sync for Targetconfig {}
impl Targetconfig {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Target0 priority threshold."]
    #[inline(always)]
    pub const fn threshold(self) -> self::common::Reg<regs::Threshold, self::common::RW> {
        unsafe { self::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Target claim and complete."]
    #[inline(always)]
    pub const fn claim(self) -> self::common::Reg<regs::Claim, self::common::RW> {
        unsafe { self::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Preempted priority stack."]
    #[inline(always)]
    pub const fn pps(self) -> self::common::Reg<regs::Pps, self::common::RW> {
        unsafe { self::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Targetint {
    ptr: *mut u8,
}
unsafe impl Send for Targetint {}
unsafe impl Sync for Targetint {}
impl Targetint {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn inten(self, n: usize) -> self::common::Reg<regs::Inten, self::common::RW> {
        assert!(n < 4usize);
        unsafe { self::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
}
pub mod common {
    use core::marker::PhantomData;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct RW;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct R;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct W;
    mod sealed {
        use super::*;
        pub trait Access {}
        impl Access for R {}
        impl Access for W {}
        impl Access for RW {}
    }
    pub trait Access: sealed::Access + Copy {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
    pub trait Read: Access {}
    impl Read for RW {}
    impl Read for R {}
    pub trait Write: Access {}
    impl Write for RW {}
    impl Write for W {}
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct Reg<T: Copy, A: Access> {
        ptr: *mut u8,
        phantom: PhantomData<*mut (T, A)>,
    }
    unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
    unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}
    impl<T: Copy, A: Access> Reg<T, A> {
        #[allow(clippy::missing_safety_doc)]
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
            Self {
                ptr: ptr as _,
                phantom: PhantomData,
            }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut T {
            self.ptr as _
        }
    }
    impl<T: Copy, A: Read> Reg<T, A> {
        #[inline(always)]
        pub fn read(&self) -> T {
            unsafe { (self.ptr as *mut T).read_volatile() }
        }
    }
    impl<T: Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write_value(&self, val: T) {
            unsafe { (self.ptr as *mut T).write_volatile(val) }
        }
    }
    impl<T: Default + Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = Default::default();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
    impl<T: Copy, A: Read + Write> Reg<T, A> {
        #[inline(always)]
        pub fn modify<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = self.read();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
}
pub mod regs {
    #[doc = "Target claim and complete."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Claim(pub u32);
    impl Claim {
        #[doc = "On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed)."]
        #[inline(always)]
        pub const fn interrupt_id(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed)."]
        #[inline(always)]
        pub fn set_interrupt_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Claim {
        #[inline(always)]
        fn default() -> Claim {
            Claim(0)
        }
    }
    #[doc = "Feature enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Feature(pub u32);
    impl Feature {
        #[doc = "Preemptive priority interrupt enable 0: Disabled 1: Enabled."]
        #[inline(always)]
        pub const fn preempt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Preemptive priority interrupt enable 0: Disabled 1: Enabled."]
        #[inline(always)]
        pub fn set_preempt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Vector mode enable 0: Disabled 1: Enabled."]
        #[inline(always)]
        pub const fn vectored(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Vector mode enable 0: Disabled 1: Enabled."]
        #[inline(always)]
        pub fn set_vectored(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Feature {
        #[inline(always)]
        fn default() -> Feature {
            Feature(0)
        }
    }
    #[doc = "Version and the maximum priority."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Info(pub u32);
    impl Info {
        #[doc = "The version of the PLIC design."]
        #[inline(always)]
        pub const fn version(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The version of the PLIC design."]
        #[inline(always)]
        pub fn set_version(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The maximum priority supported."]
        #[inline(always)]
        pub const fn max_priority(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The maximum priority supported."]
        #[inline(always)]
        pub fn set_max_priority(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Info {
        #[inline(always)]
        fn default() -> Info {
            Info(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Inten(pub u32);
    impl Inten {
        #[doc = "The interrupt enable bit for interrupt. Every interrupt source occupies 1 bit."]
        #[inline(always)]
        pub const fn interrupt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The interrupt enable bit for interrupt. Every interrupt source occupies 1 bit."]
        #[inline(always)]
        pub fn set_interrupt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Inten {
        #[inline(always)]
        fn default() -> Inten {
            Inten(0)
        }
    }
    #[doc = "Number of supported interrupt sources and targets."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Number(pub u32);
    impl Number {
        #[doc = "The number of supported interrupt sources."]
        #[inline(always)]
        pub const fn num_interrupt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The number of supported interrupt sources."]
        #[inline(always)]
        pub fn set_num_interrupt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The number of supported targets."]
        #[inline(always)]
        pub const fn num_target(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The number of supported targets."]
        #[inline(always)]
        pub fn set_num_target(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Number {
        #[inline(always)]
        fn default() -> Number {
            Number(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pending(pub u32);
    impl Pending {
        #[doc = "The interrupt pending status of inpterrupt sources. Every interrupt source occupies 1 bit."]
        #[inline(always)]
        pub const fn interrupt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The interrupt pending status of inpterrupt sources. Every interrupt source occupies 1 bit."]
        #[inline(always)]
        pub fn set_interrupt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pending {
        #[inline(always)]
        fn default() -> Pending {
            Pending(0)
        }
    }
    #[doc = "Preempted priority stack."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pps(pub u32);
    impl Pps {
        #[doc = "Each bit indicates if the corresponding priority level has been preempted by a higher-priority interrupt."]
        #[inline(always)]
        pub const fn priority_preempted(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Each bit indicates if the corresponding priority level has been preempted by a higher-priority interrupt."]
        #[inline(always)]
        pub fn set_priority_preempted(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pps {
        #[inline(always)]
        fn default() -> Pps {
            Pps(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priority(pub u32);
    impl Priority {
        #[doc = "Interrupt source priority. The valid range of this field is 0-7. 0: Never interrupt 1-7: Interrupt source priority. The larger the value, the higher the priority."]
        #[inline(always)]
        pub const fn priority(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interrupt source priority. The valid range of this field is 0-7. 0: Never interrupt 1-7: Interrupt source priority. The larger the value, the higher the priority."]
        #[inline(always)]
        pub fn set_priority(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Priority {
        #[inline(always)]
        fn default() -> Priority {
            Priority(0)
        }
    }
    #[doc = "Target0 priority threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Threshold(pub u32);
    impl Threshold {
        #[doc = "Interrupt priority threshold."]
        #[inline(always)]
        pub const fn threshold(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interrupt priority threshold."]
        #[inline(always)]
        pub fn set_threshold(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Threshold {
        #[inline(always)]
        fn default() -> Threshold {
            Threshold(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trigger(pub u32);
    impl Trigger {
        #[doc = "The interrupt trigger type of interrupt sources. Every interrupt source occupies 1 bit. 0: Level-triggered interrupt 1: Edge-triggered interrupt."]
        #[inline(always)]
        pub const fn interrupt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The interrupt trigger type of interrupt sources. Every interrupt source occupies 1 bit. 0: Level-triggered interrupt 1: Edge-triggered interrupt."]
        #[inline(always)]
        pub fn set_interrupt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Trigger {
        #[inline(always)]
        fn default() -> Trigger {
            Trigger(0)
        }
    }
}

// Helper extension trait for the PLIC
pub trait PlicExt {
    unsafe fn enable_vectored_mode(&self);

    unsafe fn enable_preemptive_mode(&self);

    fn threshold(&self) -> u32;

    fn set_threshold(&self, threshold: u32);

    fn claim(&self) -> u16;

    fn complete(&self, id: u16);
}

impl PlicExt for Plic {
    #[inline]
    unsafe fn enable_vectored_mode(&self) {
        self.feature().modify(|w| w.set_vectored(true));
    }

    #[inline]
    unsafe fn enable_preemptive_mode(&self) {
        self.feature().modify(|w| w.set_preempt(true));
    }

    #[inline]
    fn threshold(&self) -> u32 {
        self.targetconfig(0).threshold().read().threshold()
    }

    #[inline]
    fn set_threshold(&self, threshold: u32) {
        self.targetconfig(0)
            .threshold()
            .write(|w| w.set_threshold(threshold));
    }

    #[inline]
    fn claim(&self) -> u16 {
        self.targetconfig(0).claim().read().interrupt_id()
    }

    #[inline]
    fn complete(&self, id: u16) {
        self.targetconfig(0)
            .claim()
            .modify(|w| w.set_interrupt_id(id));
    }
}
