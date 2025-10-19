/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::outcome::Outcome;

#[derive(Debug, Copy, Clone)]
pub struct LogicalArgs<RegD, RegN, Mask> {
    pub rd: RegD,
    pub rn: RegN,
    pub mask: Mask,
}

pub trait MakeSpLogicalArgs<Rd, Rn, Mask> {
    type Outcome: Outcome;

    fn new(rd: Rd, rn: Rn, mask: Mask) -> Self::Outcome;
}

pub trait MakeZeroLogicalArgs<Rd, Rn, Mask> {
    type Outcome: Outcome;

    fn new(rd: Rd, rn: Rn, mask: Mask) -> Self::Outcome;
}
