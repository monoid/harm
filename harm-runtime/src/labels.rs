/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use harm::reloc::Offset;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LabelInfo {
    Forward,
    // TODO segment
    Offset(Offset),
}
