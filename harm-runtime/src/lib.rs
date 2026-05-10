/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

pub mod builder;
pub mod labels;
#[macro_use]
mod macros;
pub mod memory;
pub mod runtime;

pub use self::runtime::{Assembler, AssemblerError};
