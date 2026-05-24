/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::{
    bits::BitError,
    outcome::{Outcome, Unfallible},
    register::{IntoReg, RegOrSp64, RegOrZero32, RegOrZero64},
    sealed::Sealed,
};

use super::{
    Inc, LdStIncOffset, LdStPcOffset, LdpStpOffset32, LdpStpOffset64, Pc,
    ScaledOffset8, ScaledOffset16, ScaledOffset32, ScaledOffset64, ScaledOffset128, UnscaledOffset,
    shift_extend::Extended,
};

/// Marker trait for the concrete immediate-offset types used by ldst instructions.
/// Visibility-sealed by `pub(super)`: external code cannot implement this trait.
pub(super) trait LdStImmOffset: Copy + Default {}

impl LdStImmOffset for ScaledOffset8 {}
impl LdStImmOffset for ScaledOffset16 {}
impl LdStImmOffset for ScaledOffset32 {}
impl LdStImmOffset for ScaledOffset64 {}
impl LdStImmOffset for ScaledOffset128 {}
impl LdStImmOffset for UnscaledOffset {}

/// Shared argument struct for all single-register load/store instructions.
#[derive(Debug, Copy, Clone)]
pub struct LdStArgs<Rt, Addr> {
    pub rt: Rt,
    pub addr: Addr,
}

impl<Rt, Addr> Sealed for LdStArgs<Rt, Addr> {}

/// Trait for constructing `LdStArgs` from various input argument combinations.
pub trait MakeLdStArgs<RtIn, AddrIn>: Sealed {
    type Outcome: Outcome<Inner = Self>;
    fn new(dst: RtIn, addr: AddrIn) -> Self::Outcome;
}

// ── Scaled immediate offset: bare base (infallible, zero offset) ─────────────
// One concrete impl per (Rt, Off) pair keeps each call site unambiguous.

impl<RtIn, Base> MakeLdStArgs<RtIn, Base>
    for LdStArgs<RegOrZero64, (RegOrSp64, ScaledOffset64)>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, base: Base) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr: (base.into_reg(), Default::default()) })
    }
}

impl<RtIn, Base> MakeLdStArgs<RtIn, Base>
    for LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset32)>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, base: Base) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr: (base.into_reg(), Default::default()) })
    }
}

// ── Scaled immediate offset: 1-tuple base (infallible, zero offset) ──────────

impl<RtIn, Base> MakeLdStArgs<RtIn, (Base,)>
    for LdStArgs<RegOrZero64, (RegOrSp64, ScaledOffset64)>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, (base,): (Base,)) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr: (base.into_reg(), Default::default()) })
    }
}

impl<RtIn, Base> MakeLdStArgs<RtIn, (Base,)>
    for LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset32)>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, (base,): (Base,)) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr: (base.into_reg(), Default::default()) })
    }
}

// ── Scaled immediate offset: u32 fallible ────────────────────────────────────

impl<RtIn, Base> MakeLdStArgs<RtIn, (Base, u32)>
    for LdStArgs<RegOrZero64, (RegOrSp64, ScaledOffset64)>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
{
    type Outcome = Result<Self, BitError>;
    #[inline]
    fn new(dst: RtIn, (base, offset): (Base, u32)) -> Self::Outcome {
        Ok(Self { rt: dst.into_reg(), addr: (base.into_reg(), ScaledOffset64::try_from(offset)?) })
    }
}

impl<RtIn, Base> MakeLdStArgs<RtIn, (Base, u32)>
    for LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset32)>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
{
    type Outcome = Result<Self, BitError>;
    #[inline]
    fn new(dst: RtIn, (base, offset): (Base, u32)) -> Self::Outcome {
        Ok(Self { rt: dst.into_reg(), addr: (base.into_reg(), ScaledOffset32::try_from(offset)?) })
    }
}

// ── Scaled immediate offset: i32 fallible ────────────────────────────────────

impl<RtIn, Base> MakeLdStArgs<RtIn, (Base, i32)>
    for LdStArgs<RegOrZero64, (RegOrSp64, ScaledOffset64)>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
{
    type Outcome = Result<Self, BitError>;
    #[inline]
    fn new(dst: RtIn, (base, offset): (Base, i32)) -> Self::Outcome {
        Ok(Self { rt: dst.into_reg(), addr: (base.into_reg(), ScaledOffset64::try_from(offset)?) })
    }
}

impl<RtIn, Base> MakeLdStArgs<RtIn, (Base, i32)>
    for LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset32)>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
{
    type Outcome = Result<Self, BitError>;
    #[inline]
    fn new(dst: RtIn, (base, offset): (Base, i32)) -> Self::Outcome {
        Ok(Self { rt: dst.into_reg(), addr: (base.into_reg(), ScaledOffset32::try_from(offset)?) })
    }
}

// ── Scaled immediate offset: typed offset (infallible) ───────────────────────

impl<RtIn, Base, Off> MakeLdStArgs<RtIn, (Base, Off)>
    for LdStArgs<RegOrZero64, (RegOrSp64, Off)>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
    Off: LdStImmOffset,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, (base, offset): (Base, Off)) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr: (base.into_reg(), offset) })
    }
}

impl<RtIn, Base, Off> MakeLdStArgs<RtIn, (Base, Off)>
    for LdStArgs<RegOrZero32, (RegOrSp64, Off)>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
    Off: LdStImmOffset,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, (base, offset): (Base, Off)) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr: (base.into_reg(), offset) })
    }
}

// ── Pre-increment: (Inc<LdStIncOffset>, Base) ────────────────────────────────

impl<RtIn, Base> MakeLdStArgs<RtIn, (Inc<LdStIncOffset>, Base)>
    for LdStArgs<RegOrZero64, (Inc<LdStIncOffset>, RegOrSp64)>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, (inc, base): (Inc<LdStIncOffset>, Base)) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr: (inc, base.into_reg()) })
    }
}

impl<RtIn, Base> MakeLdStArgs<RtIn, (Inc<LdStIncOffset>, Base)>
    for LdStArgs<RegOrZero32, (Inc<LdStIncOffset>, RegOrSp64)>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, (inc, base): (Inc<LdStIncOffset>, Base)) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr: (inc, base.into_reg()) })
    }
}

// ── Post-increment: (Base, Inc<LdStIncOffset>) ───────────────────────────────

impl<RtIn, Base> MakeLdStArgs<RtIn, (Base, Inc<LdStIncOffset>)>
    for LdStArgs<RegOrZero64, (RegOrSp64, Inc<LdStIncOffset>)>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, (base, inc): (Base, Inc<LdStIncOffset>)) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr: (base.into_reg(), inc) })
    }
}

impl<RtIn, Base> MakeLdStArgs<RtIn, (Base, Inc<LdStIncOffset>)>
    for LdStArgs<RegOrZero32, (RegOrSp64, Inc<LdStIncOffset>)>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, (base, inc): (Base, Inc<LdStIncOffset>)) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr: (base.into_reg(), inc) })
    }
}

// ── Extended register offset (Dest matches Rt to pin Dest in `ext` type inference) ───────────

impl<RtIn, Base, Ext> MakeLdStArgs<RtIn, (Base, Ext)>
    for LdStArgs<RegOrZero64, (RegOrSp64, Extended<RegOrZero64, RegOrZero64>)>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
    Ext: Into<Extended<RegOrZero64, RegOrZero64>>,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, (base, ext): (Base, Ext)) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr: (base.into_reg(), ext.into()) })
    }
}

impl<RtIn, Base, Ext> MakeLdStArgs<RtIn, (Base, Ext)>
    for LdStArgs<RegOrZero64, (RegOrSp64, Extended<RegOrZero64, RegOrZero32>)>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
    Ext: Into<Extended<RegOrZero64, RegOrZero32>>,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, (base, ext): (Base, Ext)) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr: (base.into_reg(), ext.into()) })
    }
}

impl<RtIn, Base, Ext> MakeLdStArgs<RtIn, (Base, Ext)>
    for LdStArgs<RegOrZero32, (RegOrSp64, Extended<RegOrZero32, RegOrZero64>)>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
    Ext: Into<Extended<RegOrZero32, RegOrZero64>>,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, (base, ext): (Base, Ext)) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr: (base.into_reg(), ext.into()) })
    }
}

impl<RtIn, Base, Ext> MakeLdStArgs<RtIn, (Base, Ext)>
    for LdStArgs<RegOrZero32, (RegOrSp64, Extended<RegOrZero32, RegOrZero32>)>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
    Ext: Into<Extended<RegOrZero32, RegOrZero32>>,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, (base, ext): (Base, Ext)) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr: (base.into_reg(), ext.into()) })
    }
}

// ── Bare 64-bit register offset ──────────────────────────────────────────────

impl<RtIn, Base, OffsetReg> MakeLdStArgs<RtIn, (Base, OffsetReg)>
    for LdStArgs<RegOrZero64, (RegOrSp64, RegOrZero64)>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
    OffsetReg: IntoReg<RegOrZero64>,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, (base, offset): (Base, OffsetReg)) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr: (base.into_reg(), offset.into_reg()) })
    }
}

impl<RtIn, Base, OffsetReg> MakeLdStArgs<RtIn, (Base, OffsetReg)>
    for LdStArgs<RegOrZero32, (RegOrSp64, RegOrZero64)>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
    OffsetReg: IntoReg<RegOrZero64>,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, (base, offset): (Base, OffsetReg)) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr: (base.into_reg(), offset.into_reg()) })
    }
}

// ── PC-relative: typed offset (infallible) ───────────────────────────────────

impl<RtIn> MakeLdStArgs<RtIn, (Pc, LdStPcOffset)>
    for LdStArgs<RegOrZero64, (Pc, LdStPcOffset)>
where
    RtIn: IntoReg<RegOrZero64>,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, addr: (Pc, LdStPcOffset)) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr })
    }
}

impl<RtIn> MakeLdStArgs<RtIn, (Pc, LdStPcOffset)>
    for LdStArgs<RegOrZero32, (Pc, LdStPcOffset)>
where
    RtIn: IntoReg<RegOrZero32>,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, addr: (Pc, LdStPcOffset)) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr })
    }
}

// ── PC-relative: i32 offset (fallible) ───────────────────────────────────────

impl<RtIn> MakeLdStArgs<RtIn, (Pc, i32)>
    for LdStArgs<RegOrZero64, (Pc, LdStPcOffset)>
where
    RtIn: IntoReg<RegOrZero64>,
{
    type Outcome = Result<Self, BitError>;
    #[inline]
    fn new(dst: RtIn, (pc, offset): (Pc, i32)) -> Self::Outcome {
        LdStPcOffset::try_from(offset)
            .map(|offset| Self { rt: dst.into_reg(), addr: (pc, offset) })
    }
}

impl<RtIn> MakeLdStArgs<RtIn, (Pc, i32)>
    for LdStArgs<RegOrZero32, (Pc, LdStPcOffset)>
where
    RtIn: IntoReg<RegOrZero32>,
{
    type Outcome = Result<Self, BitError>;
    #[inline]
    fn new(dst: RtIn, (pc, offset): (Pc, i32)) -> Self::Outcome {
        LdStPcOffset::try_from(offset)
            .map(|offset| Self { rt: dst.into_reg(), addr: (pc, offset) })
    }
}

// ── Label reference ───────────────────────────────────────────────────────────

impl<RtIn> MakeLdStArgs<RtIn, crate::reloc::LabelRef>
    for LdStArgs<RegOrZero64, crate::reloc::LabelRef>
where
    RtIn: IntoReg<RegOrZero64>,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, addr: crate::reloc::LabelRef) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr })
    }
}

impl<RtIn> MakeLdStArgs<RtIn, crate::reloc::LabelRef>
    for LdStArgs<RegOrZero32, crate::reloc::LabelRef>
where
    RtIn: IntoReg<RegOrZero32>,
{
    type Outcome = Unfallible<Self>;
    #[inline]
    fn new(dst: RtIn, addr: crate::reloc::LabelRef) -> Self::Outcome {
        Unfallible(Self { rt: dst.into_reg(), addr })
    }
}

// ── Fallible wrapper: (Base, Result<Ext, Err>) ───────────────────────────────
// Rt is kept generic here: the inner impl's concrete Rt uniquely determines it.

impl<Rt, RtIn, BaseIn, Ext, Err> MakeLdStArgs<RtIn, (BaseIn, Result<Ext, Err>)>
    for LdStArgs<Rt, (RegOrSp64, Ext)>
where
    LdStArgs<Rt, (RegOrSp64, Ext)>:
        MakeLdStArgs<RtIn, (BaseIn, Ext), Outcome = Unfallible<LdStArgs<Rt, (RegOrSp64, Ext)>>>,
    BaseIn: IntoReg<RegOrSp64>,
{
    type Outcome = Result<LdStArgs<Rt, (RegOrSp64, Ext)>, Err>;
    #[inline]
    fn new(dst: RtIn, (base, ext_res): (BaseIn, Result<Ext, Err>)) -> Self::Outcome {
        ext_res.map(|ext| {
            <LdStArgs<Rt, (RegOrSp64, Ext)> as MakeLdStArgs<RtIn, (BaseIn, Ext)>>::new(
                dst, (base, ext),
            )
            .0
        })
    }
}

// ── Fallible wrapper: (Result<Ext, Err>, Base) ───────────────────────────────

impl<Rt, RtIn, BaseIn, Ext, Err> MakeLdStArgs<RtIn, (Result<Ext, Err>, BaseIn)>
    for LdStArgs<Rt, (Ext, RegOrSp64)>
where
    LdStArgs<Rt, (Ext, RegOrSp64)>:
        MakeLdStArgs<RtIn, (Ext, BaseIn), Outcome = Unfallible<LdStArgs<Rt, (Ext, RegOrSp64)>>>,
    BaseIn: IntoReg<RegOrSp64>,
{
    type Outcome = Result<LdStArgs<Rt, (Ext, RegOrSp64)>, Err>;
    #[inline]
    fn new(dst: RtIn, (ext_res, base): (Result<Ext, Err>, BaseIn)) -> Self::Outcome {
        ext_res.map(|ext| {
            <LdStArgs<Rt, (Ext, RegOrSp64)> as MakeLdStArgs<RtIn, (Ext, BaseIn)>>::new(
                dst, (ext, base),
            )
            .0
        })
    }
}

// ── Fallible wrapper: Result<Addr, Err> ──────────────────────────────────────

impl<Rt, RtIn, Addr, Err> MakeLdStArgs<RtIn, Result<Addr, Err>>
    for LdStArgs<Rt, Addr>
where
    LdStArgs<Rt, Addr>: MakeLdStArgs<RtIn, Addr, Outcome = Unfallible<LdStArgs<Rt, Addr>>>,
{
    type Outcome = Result<LdStArgs<Rt, Addr>, Err>;
    #[inline]
    fn new(dst: RtIn, addr_res: Result<Addr, Err>) -> Self::Outcome {
        addr_res.map(|addr| {
            <LdStArgs<Rt, Addr> as MakeLdStArgs<RtIn, Addr>>::new(dst, addr).0
        })
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Unscaled-offset construction traits
// ═════════════════════════════════════════════════════════════════════════════
//
// Naming mirrors the instruction suffix: Urb = unscaled-register-byte, etc.
//   MakeUrbArgs — ldurb, sturb                    (RegOrZero32 only)
//   MakeUrhArgs — ldurh, sturh                    (RegOrZero32 only)
//   MakeUrArgs  — ldur, stur, ldursb, ldursh,
//                 ldursw                          (RegOrZero32 and RegOrZero64)

macro_rules! impl_unscaled_args {
    ($trait:ident, $rt:ty) => {
        impl<RtIn, Base> $trait<RtIn, (Base,)>
            for LdStArgs<$rt, (RegOrSp64, UnscaledOffset)>
        where
            RtIn: IntoReg<$rt>,
            Base: IntoReg<RegOrSp64>,
        {
            type Outcome = Unfallible<Self>;
            #[inline]
            fn new(rt: RtIn, (base,): (Base,)) -> Self::Outcome {
                Unfallible(Self { rt: rt.into_reg(), addr: (base.into_reg(), Default::default()) })
            }
        }

        impl<RtIn, Base> $trait<RtIn, (Base, UnscaledOffset)>
            for LdStArgs<$rt, (RegOrSp64, UnscaledOffset)>
        where
            RtIn: IntoReg<$rt>,
            Base: IntoReg<RegOrSp64>,
        {
            type Outcome = Unfallible<Self>;
            #[inline]
            fn new(rt: RtIn, (base, offset): (Base, UnscaledOffset)) -> Self::Outcome {
                Unfallible(Self { rt: rt.into_reg(), addr: (base.into_reg(), offset) })
            }
        }

        impl<RtIn, Base> $trait<RtIn, (Base, i32)>
            for LdStArgs<$rt, (RegOrSp64, UnscaledOffset)>
        where
            RtIn: IntoReg<$rt>,
            Base: IntoReg<RegOrSp64>,
        {
            type Outcome = Result<Self, BitError>;
            #[inline]
            fn new(rt: RtIn, (base, offset): (Base, i32)) -> Self::Outcome {
                UnscaledOffset::try_from(offset)
                    .map(|offset| Self { rt: rt.into_reg(), addr: (base.into_reg(), offset) })
            }
        }
    };
}

/// Construction trait for byte-level unscaled-offset instructions (`ldurb`, `sturb`).
pub trait MakeUrbArgs<RtIn, AddrIn>: Sealed {
    type Outcome: Outcome<Inner = Self>;
    fn new(rt: RtIn, addr: AddrIn) -> Self::Outcome;
}

impl_unscaled_args!(MakeUrbArgs, RegOrZero32);

/// Construction trait for halfword-level unscaled-offset instructions (`ldurh`, `sturh`).
pub trait MakeUrhArgs<RtIn, AddrIn>: Sealed {
    type Outcome: Outcome<Inner = Self>;
    fn new(rt: RtIn, addr: AddrIn) -> Self::Outcome;
}

impl_unscaled_args!(MakeUrhArgs, RegOrZero32);

/// Construction trait for word/doubleword unscaled-offset instructions
/// (`ldur`, `stur`, `ldursb`, `ldursh`, `ldursw`).
pub trait MakeUrArgs<RtIn, AddrIn>: Sealed {
    type Outcome: Outcome<Inner = Self>;
    fn new(rt: RtIn, addr: AddrIn) -> Self::Outcome;
}

impl_unscaled_args!(MakeUrArgs, RegOrZero32);
impl_unscaled_args!(MakeUrArgs, RegOrZero64);

/// Construction trait for signed-byte unscaled-offset load instructions (`ldursb`).
pub trait MakeUrsbArgs<RtIn, AddrIn>: Sealed {
    type Outcome: Outcome<Inner = Self>;
    fn new(rt: RtIn, addr: AddrIn) -> Self::Outcome;
}

impl_unscaled_args!(MakeUrsbArgs, RegOrZero32);
impl_unscaled_args!(MakeUrsbArgs, RegOrZero64);

/// Construction trait for signed-halfword unscaled-offset load instructions (`ldursh`).
pub trait MakeUrshArgs<RtIn, AddrIn>: Sealed {
    type Outcome: Outcome<Inner = Self>;
    fn new(rt: RtIn, addr: AddrIn) -> Self::Outcome;
}

impl_unscaled_args!(MakeUrshArgs, RegOrZero32);
impl_unscaled_args!(MakeUrshArgs, RegOrZero64);

pub trait MakeUrswArgs<RtIn, AddrIn>: Sealed {
    type Outcome: Outcome<Inner = Self>;
    fn new(rt: RtIn, addr: AddrIn) -> Self::Outcome;
}
impl_unscaled_args!(MakeUrswArgs, RegOrZero64);

// ── Pair load/store arguments ─────────────────────────────────────────────────

/// Shared argument struct for two-register load/store pair instructions.
#[derive(Debug, Copy, Clone)]
pub struct LdStPairArgs<Rt, Addr> {
    pub rt: (Rt, Rt),
    pub addr: Addr,
}

impl<Rt, Addr> Sealed for LdStPairArgs<Rt, Addr> {}

/// Construction trait shared by `ldp` and `stp`.
pub trait MakeLdpStpArgs<D1, D2, AddrIn>: Sealed {
    type Outcome: Outcome<Inner = Self>;
    fn new(d1: D1, d2: D2, addr: AddrIn) -> Self::Outcome;
}

/// Construction trait for `ldpsw`.
pub trait MakeLdpswArgs<D1, D2, AddrIn>: Sealed {
    type Outcome: Outcome<Inner = Self>;
    fn new(d1: D1, d2: D2, addr: AddrIn) -> Self::Outcome;
}

macro_rules! impl_pair_args {
    ($trait:ident, $rt:ty, $off:ty) => {
        impl<D1, D2, Base> $trait<D1, D2, (Base, $off)>
            for LdStPairArgs<$rt, (RegOrSp64, $off)>
        where
            D1: IntoReg<$rt>,
            D2: IntoReg<$rt>,
            Base: IntoReg<RegOrSp64>,
        {
            type Outcome = Unfallible<Self>;
            #[inline]
            fn new(d1: D1, d2: D2, (base, offset): (Base, $off)) -> Self::Outcome {
                Unfallible(Self { rt: (d1.into_reg(), d2.into_reg()), addr: (base.into_reg(), offset) })
            }
        }

        impl<D1, D2, Base> $trait<D1, D2, Base>
            for LdStPairArgs<$rt, (RegOrSp64, $off)>
        where
            D1: IntoReg<$rt>,
            D2: IntoReg<$rt>,
            Base: IntoReg<RegOrSp64>,
        {
            type Outcome = Unfallible<Self>;
            #[inline]
            fn new(d1: D1, d2: D2, base: Base) -> Self::Outcome {
                Unfallible(Self { rt: (d1.into_reg(), d2.into_reg()), addr: (base.into_reg(), Default::default()) })
            }
        }

        impl<D1, D2, Base> $trait<D1, D2, (Base,)>
            for LdStPairArgs<$rt, (RegOrSp64, $off)>
        where
            D1: IntoReg<$rt>,
            D2: IntoReg<$rt>,
            Base: IntoReg<RegOrSp64>,
        {
            type Outcome = Unfallible<Self>;
            #[inline]
            fn new(d1: D1, d2: D2, (base,): (Base,)) -> Self::Outcome {
                Unfallible(Self { rt: (d1.into_reg(), d2.into_reg()), addr: (base.into_reg(), Default::default()) })
            }
        }

        impl<D1, D2, Base> $trait<D1, D2, (Base, i32)>
            for LdStPairArgs<$rt, (RegOrSp64, $off)>
        where
            D1: IntoReg<$rt>,
            D2: IntoReg<$rt>,
            Base: IntoReg<RegOrSp64>,
        {
            type Outcome = Result<Self, BitError>;
            #[inline]
            fn new(d1: D1, d2: D2, (base, offset): (Base, i32)) -> Self::Outcome {
                <$off>::try_from(offset)
                    .map(|offset| Self { rt: (d1.into_reg(), d2.into_reg()), addr: (base.into_reg(), offset) })
            }
        }

        impl<D1, D2, Base> $trait<D1, D2, (Inc<$off>, Base)>
            for LdStPairArgs<$rt, (Inc<$off>, RegOrSp64)>
        where
            D1: IntoReg<$rt>,
            D2: IntoReg<$rt>,
            Base: IntoReg<RegOrSp64>,
        {
            type Outcome = Unfallible<Self>;
            #[inline]
            fn new(d1: D1, d2: D2, (inc, base): (Inc<$off>, Base)) -> Self::Outcome {
                Unfallible(Self { rt: (d1.into_reg(), d2.into_reg()), addr: (inc, base.into_reg()) })
            }
        }

        impl<D1, D2, Base> $trait<D1, D2, (Base, Inc<$off>)>
            for LdStPairArgs<$rt, (RegOrSp64, Inc<$off>)>
        where
            D1: IntoReg<$rt>,
            D2: IntoReg<$rt>,
            Base: IntoReg<RegOrSp64>,
        {
            type Outcome = Unfallible<Self>;
            #[inline]
            fn new(d1: D1, d2: D2, (base, inc): (Base, Inc<$off>)) -> Self::Outcome {
                Unfallible(Self { rt: (d1.into_reg(), d2.into_reg()), addr: (base.into_reg(), inc) })
            }
        }

        impl<D1, D2, Base, Err> $trait<D1, D2, (Result<Inc<$off>, Err>, Base)>
            for LdStPairArgs<$rt, (Inc<$off>, RegOrSp64)>
        where
            D1: IntoReg<$rt>,
            D2: IntoReg<$rt>,
            Base: IntoReg<RegOrSp64>,
        {
            type Outcome = Result<Self, Err>;
            #[inline]
            fn new(d1: D1, d2: D2, (inc, base): (Result<Inc<$off>, Err>, Base)) -> Self::Outcome {
                inc.map(|inc| Self { rt: (d1.into_reg(), d2.into_reg()), addr: (inc, base.into_reg()) })
            }
        }

        impl<D1, D2, Base, Err> $trait<D1, D2, (Base, Result<Inc<$off>, Err>)>
            for LdStPairArgs<$rt, (RegOrSp64, Inc<$off>)>
        where
            D1: IntoReg<$rt>,
            D2: IntoReg<$rt>,
            Base: IntoReg<RegOrSp64>,
        {
            type Outcome = Result<Self, Err>;
            #[inline]
            fn new(d1: D1, d2: D2, (base, inc): (Base, Result<Inc<$off>, Err>)) -> Self::Outcome {
                inc.map(|inc| Self { rt: (d1.into_reg(), d2.into_reg()), addr: (base.into_reg(), inc) })
            }
        }
    };
}

impl_pair_args!(MakeLdpStpArgs, RegOrZero32, LdpStpOffset32);
impl_pair_args!(MakeLdpStpArgs, RegOrZero64, LdpStpOffset64);
impl_pair_args!(MakeLdpswArgs, RegOrZero64, LdpStpOffset32);
