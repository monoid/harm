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

pub(crate) mod bit_value;
pub(crate) mod instruction_code;

pub use bit_value::*;
pub use instruction_code::*;
