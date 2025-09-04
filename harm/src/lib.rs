/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */
#![no_std]
#![forbid(unsafe_code)]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

pub mod bits;
pub mod instructions;
pub use harm_types::A64::register;
pub mod reloc;

pub use aarchmrs_types::InstructionCode;
