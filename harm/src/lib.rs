/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */
#![no_std]
#[cfg(test)]
extern crate alloc;
pub mod bits;
pub mod instructions;
pub mod register;
pub mod reloc;

pub use aarchmrs_types::InstructionCode;
