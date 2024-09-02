//! Low level access to Andes' AndeStar V5 RISC-V processors

#![no_std]

/// The `register` module provides access to the processor's registers
pub mod register;

#[doc(hidden)]
pub mod common;

pub mod l1c;
