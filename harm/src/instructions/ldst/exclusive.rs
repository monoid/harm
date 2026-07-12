/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

//! Load-Exclusive/Store-Exclusive (`LDXR`, `STXR`, `LDXRB`, etc.) instructions.

pub mod exclusive_args;
mod ldxp;
mod ldxr;
mod ldxrb;
mod ldxrh;
mod stxp;
mod stxr;
mod stxrb;
mod stxrh;

pub use self::ldxp::*;
pub use self::ldxr::*;
pub use self::ldxrb::*;
pub use self::ldxrh::*;
pub use self::stxp::*;
pub use self::stxr::*;
pub use self::stxrb::*;
pub use self::stxrh::*;
