//! Low level access to Andes' AndeStar V5 RISC-V processors

#![no_std]

/// The `register` module provides access to the processor's registers
pub mod register;

pub mod l1c;

pub use riscv;

pub mod plic;

pub mod dsp;
