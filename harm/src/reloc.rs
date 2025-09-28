/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

// TODO These type definitions should probably be moved to `harm-types` crate.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LabelId(pub usize);

// Every offset in an instruction does fit in i32.
// But what if we want to support virtual instructions with larger offsets?
pub type Offset = i64;

pub type Addr = u64;

// b_cond(Cond, LabelRef)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LabelRef {
    pub id: LabelId,
    pub offset: Offset,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reloc {
    Label(LabelRef),
    // TODO lower bits, middle bits, signed and unsigned highest bits
    Delta(LabelRef, LabelRef), // label1 - label2
}
