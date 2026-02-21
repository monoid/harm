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
pub struct Rel64 {
    pub rel: Rel64Tag,
    pub label: LabelRef,
}

impl Rel64 {
    #[inline]
    pub const fn new(rel: Rel64Tag, label: LabelRef) -> Self {
        Self { rel, label }
    }

    #[inline]
    pub const fn ld_prel_lo19(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::LdPrelLo19,
            label,
        }
    }

    #[inline]
    pub const fn cond_br19(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::CondBr19,
            label,
        }
    }

    #[inline]
    pub const fn tst_br14(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::TstBr14,
            label,
        }
    }

    #[inline]
    pub const fn jump26(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::Jump26,
            label,
        }
    }

    #[inline]
    pub const fn call26(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::Call26,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g0(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG0,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g0nc(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG0Nc,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g0s(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG0S,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g1(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG1,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g1nc(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG1Nc,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g1s(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG1S,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g2(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG2,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g2nc(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG2Nc,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g2s(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG2S,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g3(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG3,
            label,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rel64Tag {
    None,
    // Static data relocations
    // ...
    Abs64,
    Abs32,
    Abs16,
    PRel64,
    PRel32,
    PRel16,
    Plt32,

    // Static AArch64 relocations
    LdPrelLo19,
    AdrPrelLo21,
    AdrPrelPgHi21,
    AdrPrelPgHi21Nc,
    AddAbsLo12Nc,
    // Static control flow relocations
    TstBr14,
    CondBr19,
    Jump26,
    Call26, // same as Jump26 actually?

    // TODO `MOVW` and some `add`/`ldst`-related relocations
    MovWAbsG0,
    MovWAbsG0Nc,
    MovWAbsG0S,
    MovWAbsG1,
    MovWAbsG1Nc,
    MovWAbsG1S,
    MovWAbsG2,
    MovWAbsG2Nc,
    MovWAbsG2S,
    MovWAbsG3,
}
