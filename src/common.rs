#![allow(private_bounds)]
use core::marker::PhantomData;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RW;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct R;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct W;
trait SealedAccess {}
impl SealedAccess for R {}
impl SealedAccess for W {}
impl SealedAccess for RW {}

pub trait Access: SealedAccess + Copy {}
impl Access for R {}
impl Access for W {}
impl Access for RW {}
pub trait Read: Access {}
impl Read for RW {}
impl Read for R {}
pub trait Write: Access {}
impl Write for RW {}
impl Write for W {}
pub(crate) trait SealedCSR {
    unsafe fn read_csr() -> usize;
    unsafe fn write_csr(value: usize);
}
pub trait CSR: SealedCSR {}
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Reg<T: Copy, C: CSR, A: Access> {
    phantom: PhantomData<*mut (T, C, A)>,
}
unsafe impl<T: Copy, C: CSR, A: Access> Send for Reg<T, C, A> {}
unsafe impl<T: Copy, C: CSR, A: Access> Sync for Reg<T, C, A> {}
impl<T: Copy, C: CSR, A: Access> Reg<T, C, A> {
    #[allow(clippy::missing_safety_doc)]
    #[inline(always)]
    pub(crate) const unsafe fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
impl<T: Copy, C: CSR, A: Read> Reg<T, C, A> {
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe {
            let mut val: T = core::mem::zeroed();
            let out = C::read_csr() as u32;
            (&mut val as *mut T as *mut u32).write_volatile(out);
            val
        }
    }
}
impl<T: Copy, C: CSR, A: Write> Reg<T, C, A> {
    #[inline(always)]
    pub unsafe fn write_value(&self, val: T) {
        let mut new_val: u32 = 0;
        unsafe {
            (&mut new_val as *mut u32 as *mut T).write_volatile(val);
            C::write_csr(new_val as usize)
        }
    }
}
impl<T: Default + Copy, C: CSR, A: Write> Reg<T, C, A> {
    #[inline(always)]
    pub unsafe fn write<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut val = Default::default();
        let res = f(&mut val);
        self.write_value(val);
        res
    }
}
impl<T: Copy, C: CSR, A: Read + Write> Reg<T, C, A> {
    #[inline(always)]
    pub unsafe fn modify<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut val = self.read();
        let res = f(&mut val);
        self.write_value(val);
        res
    }
}
