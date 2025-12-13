/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

// TODO These type definitions should probably be moved to `harm-types` crate.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LabelId(pub usize);

// Every offset in an instruction does fit in i32.
// But "[relocation] is sign-extended to 64 bits".
pub type Offset = i64;

pub type Addr = u64;

// b_cond(Cond, LabelRef)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LabelRef {
    pub id: LabelId,
    pub addend: Offset,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rel64 {
    None,
    // Static data relocations
    // ...
    Abs64(LabelRef),
    Abs32(LabelRef),
    Abs16(LabelRef),
    PRel64(LabelRef),
    PRel32(LabelRef),
    PRel16(LabelRef),
    Plt32(LabelRef),

    // Static AArch64 relocations
    LdPrelLo19(LabelRef),
    AdrPrelLo21(LabelRef),
    AdrPrelPgHi21(LabelRef),
    AdrPrelPgHi21Nc(LabelRef),
    AddAbsLo12Nc(LabelRef),
    // Static control flow relocations
    TstBr14(LabelRef),
    CondBr19(LabelRef),
    Jump26(LabelRef),
    Call26(LabelRef),  // same as Jump26 actually?

    // TODO `MOVW` and some `add`/`ldst`-related relocations
}
