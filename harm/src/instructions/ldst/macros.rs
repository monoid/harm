/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

macro_rules! define_reg_offset_rules {
    ($name:ident, $trait_name:ident, $mnem:ident, $rt:ty, $bitness:expr) => {
        define_reg_offset_rules!($name, $trait_name, $mnem, $rt, $bitness, $rt);
    };
    ($name:ident, $trait_name:ident, $mnem:ident, $rt:ty, $bitness:expr, $shift:ty) => {
        /// `LDR` with 64-bit destination, base register with extended 64-bit register offset with scale.
        impl<Rt, Base, Ext> $trait_name<Rt, (Base, Ext)>
            for $name<$rt, (RegOrSp64, Extended<$shift, RegOrZero64>)>
        where
            Rt: Into<$rt>,
            Base: Into<RegOrSp64>,
            Ext: Into<Extended<$shift, RegOrZero64>>,
        {
            type Output = Self;

            #[inline]
            fn new(rt: Rt, (base, offset): (Base, Ext)) -> Self {
                Self {
                    rt: rt.into(),
                    addr: (base.into(), offset.into()),
                }
            }
        }

        /// `LDR` with 64-bit destination, base register with extended 32-bit register offset with scale.
        impl<Rt, Base, Ext> $trait_name<Rt, (Base, Ext)>
            for $name<$rt, (RegOrSp64, Extended<$shift, RegOrZero32>)>
        where
            Rt: Into<$rt>,
            Base: Into<RegOrSp64>,
            Ext: Into<Extended<$shift, RegOrZero32>>,
        {
            type Output = Self;

            #[inline]
            fn new(rt: Rt, (base, offset): (Base, Ext)) -> Self {
                Self {
                    rt: rt.into(),
                    addr: (base.into(), offset.into()),
                }
            }
        }

        /// `LDR` with 64-bit destination, bare base register.
        impl<Rt, Base> $trait_name<Rt, Base> for $name<$rt, (RegOrSp64, RegOrZero64)>
        where
            Rt: Into<$rt>,
            Base: Into<RegOrSp64>,
        {
            type Output = Self;

            #[inline]
            fn new(rt: Rt, base: Base) -> Self {
                Self {
                    rt: rt.into(),
                    // TODO does the spec says something specific?
                    addr: (base.into(), RegOrZero64::XZR),
                }
            }
        }

        /// `LDR` with 64-bit destination, bare base register as a tuple.
        impl<Rt, Base> $trait_name<Rt, (Base,)> for $name<$rt, (RegOrSp64, RegOrZero64)>
        where
            Rt: Into<$rt>,
            Base: Into<RegOrSp64>,
        {
            type Output = Self;

            #[inline]
            fn new(rt: Rt, (base,): (Base,)) -> Self {
                Self {
                    rt: rt.into(),
                    addr: (base.into(), RegOrZero64::XZR),
                }
            }
        }

        /// `LDR` with 64-bit destination, base register with 64-bit offset without scaling.
        impl<Rt, Base, OffsetReg> $trait_name<Rt, (Base, OffsetReg)>
            for $name<$rt, (RegOrSp64, RegOrZero64)>
        where
            Rt: Into<$rt>,
            Base: Into<RegOrSp64>,
            OffsetReg: Into<RegOrZero64>,
        {
            type Output = Self;

            #[inline]
            fn new(rt: Rt, (base, offset): (Base, OffsetReg)) -> Self {
                Self {
                    rt: rt.into(),
                    addr: (base.into(), offset.into()),
                }
            }
        }

        ::paste::paste! {
            impl RawInstruction for $name<$rt, (RegOrSp64, Extended<$shift, RegOrZero64>)> {
                #[inline]
                fn to_code(&self) -> crate::InstructionCode {
                    let (base, offset) = self.addr;
                    [<$mnem:upper _ $bitness _ldst_regoff>](
                        offset.offset.code(),
                        (offset.extend as u8).into(),
                        offset.shifted.into(),
                        base.code(),
                        self.rt.code(),
                    )
                }
            }

            impl RawInstruction for $name<$rt, (RegOrSp64, Extended<$shift, RegOrZero32>)> {
                #[inline]
                fn to_code(&self) -> crate::InstructionCode {
                    let (base, offset) = self.addr;
                    [<$mnem:upper _ $bitness _ldst_regoff>](
                        offset.offset.code(),
                        (offset.extend as u8).into(),
                        offset.shifted.into(),
                        base.code(),
                        self.rt.code(),
                    )
                }
            }

            impl RawInstruction for $name<$rt, (RegOrSp64, RegOrZero64)> {
                #[inline]
                fn to_code(&self) -> crate::InstructionCode {
                    let (base, offset) = self.addr;
                    [<$mnem:upper _ $bitness _ldst_regoff>](
                        offset.code(),
                        (LdStExtendOption64::default() as u8).into(),
                        0b0.into(),
                        base.code(),
                        self.rt.code(),
                    )
                }
            }
        }
    };
}

macro_rules! define_imm_offset_rules {
    ($name:ident, $trait_name:ident, $mnem:ident, $rt:ty, $bitness:expr, $offset_type:ty) => {
        /// `LDR` with 64-bit destination, base register with aligned immediate offset.
        impl<Rt, B> $trait_name<Rt, (B, $offset_type)>
            for $name<$rt, (RegOrSp64, $offset_type)>
        where
            Rt: Into<$rt>,
            B: Into<RegOrSp64>,
        {
            type Output = Self;

            #[inline]
            fn new(rt: Rt, (base, offset): (B, $offset_type)) -> Self {
                Self {
                    rt: rt.into(),
                    addr: (base.into(), offset),
                }
            }
        }

        /// `LDR` with 64-bit destination, base register with immediate offset. It is fallible, as the offset has to have
        /// specific range and alignment.
        impl<Rt, B> $trait_name<Rt, (B, u32)> for $name<$rt, (RegOrSp64, $offset_type)>
        where
            Rt: Into<$rt>,
            B: Into<RegOrSp64>,
        {
            type Output = Result<Self, BitError>;

            #[inline]
            fn new(rt: Rt, (base, offset): (B, u32)) -> Self::Output {
                let offset = <$offset_type>::try_from(offset)?;
                Ok(Self {
                    rt: rt.into(),
                    addr: (base.into(), offset),
                })
            }
        }

        /// `LDR` with 64-bit destination, base register with immediate offset. It is fallible, as the offset has to have
        /// specific range and alignment.
        impl<Rt, B> $trait_name<Rt, (B, i32)> for $name<$rt, (RegOrSp64, $offset_type)>
        where
            Rt: Into<$rt>,
            B: Into<RegOrSp64>,
        {
            type Output = Result<Self, BitError>;

            #[inline]
            fn new(rt: Rt, (base, offset): (B, i32)) -> Self::Output {
                let offset = <$offset_type>::try_from(offset)?;
                Ok(Self {
                    rt: rt.into(),
                    addr: (base.into(), offset),
                })
            }
        }

        impl<Rt: Into<$rt>, Base: Into<RegOrSp64>> $trait_name<Rt, (Inc<LdStIncOffset>, Base)>
            for $name<$rt, (Inc<LdStIncOffset>, RegOrSp64)>
        {
            type Output = Self;

            fn new(rt: Rt, (inc, base): (Inc<LdStIncOffset>, Base)) -> Self {
                Self {
                    rt: rt.into(),
                    addr: (inc, base.into()),
                }
            }
        }

        impl<Rt: Into<$rt>, Base: Into<RegOrSp64>> $trait_name<Rt, (Base, Inc<LdStIncOffset>)>
            for $name<$rt, (RegOrSp64, Inc<LdStIncOffset>)>
        {
            type Output = Self;

            fn new(rt: Rt, (base, inc): (Base, Inc<LdStIncOffset>)) -> Self {
                Self {
                    rt: rt.into(),
                    addr: (base.into(), inc),
                }
            }
        }

        ::paste::paste! {
            impl $crate::instructions::RawInstruction for $name<$rt, (RegOrSp64, $offset_type)> {
                #[inline]
                fn to_code(&self) -> crate::InstructionCode {
                    let (base, offset) = self.addr;
                    let code = [<$mnem:upper _ $bitness _ldst_pos>](offset.into(), base.code(), self.rt.code());
                    code
                }
            }

            impl $crate::instructions::RawInstruction for $name<$rt, (Inc<LdStIncOffset>, RegOrSp64)> {
                #[inline]
                fn to_code(&self) -> crate::InstructionCode {
                    let (inc, base) = self.addr;
                    let code = [<$mnem:upper _ $bitness _ldst_immpre>](inc.offset.into(), base.code(), self.rt.code());
                    code
                }
            }

            impl $crate::instructions::RawInstruction for $name<$rt, (RegOrSp64, Inc<LdStIncOffset>)> {
                #[inline]
                fn to_code(&self) -> crate::InstructionCode {
                    let (base, inc) = self.addr;

                    let code = [<$mnem:upper _ $bitness _ldst_immpost>](inc.offset.into(), base.code(), self.rt.code());
                    code
                }
            }
        }
    };
}

macro_rules! define_unscaled_imm_offset_rules {
    ($name:ident, $make_name:ident, $mnem:ident, $rt:ty, $bitness:expr) => {
        impl<Rt, Base> $make_name<Rt, (Base, UnscaledOffset)>
            for $name<$rt, (RegOrSp64, UnscaledOffset)>
        where
            Rt: Into<$rt>,
            Base: Into<RegOrSp64>,
        {
            type Output = Self;

            fn new(rt: Rt, (base, offset): (Base, UnscaledOffset)) -> Self::Output {
                Self {
                    rt: rt.into(),
                    addr: (base.into(), offset),
                }
            }
        }

        impl<Rt, Base> $make_name<Rt, (Base, i32)>
            for $name<$rt, (RegOrSp64, UnscaledOffset)>
        where
            Rt: Into<$rt>,
            Base: Into<RegOrSp64>,
        {
            type Output = Result<Self, BitError>;

            fn new(rt: Rt, (base, offset): (Base, i32)) -> Self::Output {
                UnscaledOffset::try_from(offset).map(|offset| Self {
                    rt: rt.into(),
                    addr: (base.into(), offset),
                })
            }
        }

        ::paste::paste! {
            impl RawInstruction for $name<$rt, (RegOrSp64, UnscaledOffset)> {
                #[inline]
                fn to_code(&self) -> crate::InstructionCode {
                    let (base, offset) = self.addr;
                    [<$mnem:upper _ $bitness _ldst_unscaled>](offset.into(), base.code(), self.rt.code())
                }
            }
        }
    };
}

macro_rules! define_pc_offset_rules {
    ($name:ident, $trait_name:ident, $mnem:ident, $rt:ty, $bitness:expr) => {
        #[doc = "`"]
        #[doc = stringify!($mnem)]
        #[doc = "` with an immediate PC-related offset."]
        impl<Rt> $trait_name<Rt, ($crate::instructions::ldst::Pc, $crate::instructions::ldst::LdStPcOffset)>
            for $name<$rt, ($crate::instructions::ldst::Pc, $crate::instructions::ldst::LdStPcOffset)>
        where
            Rt: ::core::convert::Into<$rt>,
        {
            type Output = Self;

            #[inline]
            fn new(rt: Rt, addr: ($crate::instructions::ldst::Pc, $crate::instructions::ldst::LdStPcOffset)) -> Self {
                Self {
                    rt: rt.into(),
                    addr,
                }
            }
        }

        #[doc = "`"]
        #[doc = stringify!($mnem)]
        #[doc = "` with an immediate PC-related offset."]
        impl<Rt> $trait_name<Rt, ($crate::instructions::ldst::Pc, i32)>
            for $name<$rt, ($crate::instructions::ldst::Pc, $crate::instructions::ldst::LdStPcOffset)>
        where
            Rt: ::core::convert::Into<$rt>,
        {
            type Output = ::core::result::Result<Self, $crate::bits::BitError>;

            #[inline]
            fn new(rt: Rt, (pc, offset): ($crate::instructions::ldst::Pc, i32)) -> Self::Output {
                $crate::instructions::ldst::LdStPcOffset::try_from(offset).map(|offset| Self {
                    rt: rt.into(),
                    addr: (pc, offset)
                })
            }
        }

        ::paste::paste! {
            impl $crate::instructions::RawInstruction for $name<$rt, ($crate::instructions::ldst::Pc, $crate::instructions::ldst::LdStPcOffset)> {
                #[inline]
                fn to_code(&self) -> crate::InstructionCode {
                    let (_pc, offset) = self.addr;
                    let code = [<$mnem:upper _ $bitness _ loadlit>](offset.into(), self.rt.code());
                    code
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_fallible_rules {
    ($mnem: ident, $name:ident, $trait_name:ident) => {
        #[doc = "`"]
        #[doc = stringify!($mnem)]
        #[doc = "` with fallible offset that delegates to non-faillible variants."]
        impl<RtInp, RtOut, BaseInp, Ext, Err> $trait_name<RtInp, (BaseInp, Result<Ext, Err>)>
            for $name<RtOut, ($crate::register::RegOrSp64, Ext)>
        where
            $name<RtOut, ($crate::register::RegOrSp64, Ext)>: $trait_name<RtInp, (BaseInp, Ext)>,
            $crate::register::RegOrSp64: From<BaseInp>,
        {
            type Output =
                ::core::result::Result<<Self as $trait_name<RtInp, (BaseInp, Ext)>>::Output, Err>;

            #[inline]
            fn new(
                rt: RtInp,
                (base, offset): (BaseInp, ::core::result::Result<Ext, Err>),
            ) -> Self::Output {
                offset.map(|offset| $name::new(rt, (base, offset)))
            }
        }

        #[doc = "`"]
        #[doc = stringify!($mnem)]
        #[doc = "` with fallible offset that delegates to non-faillible variants."]
        impl<RtInp, RtOut, BaseInp, Ext, Err>
            $trait_name<RtInp, (::core::result::Result<Ext, Err>, BaseInp)>
            for $name<RtOut, (Ext, $crate::register::RegOrSp64)>
        where
            $name<RtOut, (Ext, $crate::register::RegOrSp64)>: $trait_name<RtInp, (Ext, BaseInp)>,
            $crate::register::RegOrSp64: From<BaseInp>,
        {
            type Output =
                ::core::result::Result<<Self as $trait_name<RtInp, (Ext, BaseInp)>>::Output, Err>;

            #[inline]
            fn new(
                rt: RtInp,
                (ext, base): (::core::result::Result<Ext, Err>, BaseInp),
            ) -> Self::Output {
                ext.map(|ext| $name::new(rt, (ext, base)))
            }
        }

        #[doc = "`"]
        #[doc = stringify!($mnem)]
        #[doc = "` with fallible address that delegates to non-faillible variants."]
        impl<RtInp, RtOut, Addr, Err> $trait_name<RtInp, ::core::result::Result<Addr, Err>>
            for $name<RtOut, Addr>
        where
            $name<RtOut, Addr>: $trait_name<RtInp, Addr>,
        {
            type Output = ::core::result::Result<<Self as $trait_name<RtInp, Addr>>::Output, Err>;

            #[inline]
            fn new(rt: RtInp, addr: ::core::result::Result<Addr, Err>) -> Self::Output {
                addr.map(|addr| $name::new(rt, addr))
            }
        }
    };
}

#[macro_export]
macro_rules! define_pair_imm_offset_rules {
    ($name:ident, $trait_name:ident, $mnem:ident, $rt:ty, $bitness:expr, $offset_type:ty) => {
        #[doc = r" `LDP` with 64-bit destination, base register with aligned immediate offset."]
        impl<Rt1, Rt2, B> $trait_name<Rt1, Rt2, (B, $offset_type)>
            for $name<$rt, (RegOrSp64, $offset_type)>
        where
            Rt1: Into<$rt>,
            Rt2: Into<$rt>,
            B: Into<RegOrSp64>,
        {
            type Output = Self;
            #[inline]
            fn new(rt: (Rt1, Rt2), (base, offset): (B, $offset_type)) -> Self {
                Self {
                    rt: (rt.0.into(), rt.1.into()),
                    addr: (base.into(), offset),
                }
            }
        }

        #[doc = r" `LDP` with 64-bit destination, base register with immediate offset. It is fallible, as the offset has to have"]
        #[doc = r" specific range and alignment."]
        impl<Rt1, Rt2, B> $trait_name<Rt1, Rt2, (B, i32)> for $name<$rt, (RegOrSp64, $offset_type)>
        where
            Rt1: Into<$rt>,
            Rt2: Into<$rt>,
            B: Into<RegOrSp64>,
        {
            type Output = Result<Self, BitError>;
            #[inline]
            fn new(rt: (Rt1, Rt2), (base, offset): (B, i32)) -> Self::Output {
                let offset = <$offset_type>::try_from(offset)?;
                Ok(Self {
                    rt: (rt.0.into(), rt.1.into()),
                    addr: (base.into(), offset),
                })
            }
        }
        impl<Rt1: Into<$rt>, Rt2: Into<$rt>, Base: Into<RegOrSp64>>
            $trait_name<Rt1, Rt2, (Inc<$offset_type>, Base)>
            for $name<$rt, (Inc<$offset_type>, RegOrSp64)>
        {
            type Output = Self;
            fn new(rt: (Rt1, Rt2), (inc, base): (Inc<$offset_type>, Base)) -> Self {
                Self {
                    rt: (rt.0.into(), rt.1.into()),
                    addr: (inc, base.into()),
                }
            }
        }
        impl<Rt1: Into<$rt>, Rt2: Into<$rt>, Base: Into<RegOrSp64>>
            $trait_name<Rt1, Rt2, (Base, Inc<$offset_type>)>
            for $name<$rt, (RegOrSp64, Inc<$offset_type>)>
        {
            type Output = Self;
            fn new(rt: (Rt1, Rt2), (base, inc): (Base, Inc<$offset_type>)) -> Self {
                Self {
                    rt: (rt.0.into(), rt.1.into()),
                    addr: (base.into(), inc),
                }
            }
        }

        ::paste::paste! {
            impl crate::instructions::RawInstruction for $name<$rt, (RegOrSp64, $offset_type)> {
                #[inline]
                fn to_code(&self) -> crate::InstructionCode {
                    let (base, offset) = self.addr;
                    let code = [<$mnem _ $bitness _ldstpair_off>](
                        offset.into(),
                        self.rt.1.code(),
                        base.code(),
                        self.rt.0.code(),
                    );
                    code
                }
            }
            impl crate::instructions::RawInstruction for $name<$rt, (Inc<$offset_type>, RegOrSp64)> {
                #[inline]
                fn to_code(&self) -> crate::InstructionCode {
                    let (inc, base) = self.addr;
                    let code = [<$mnem _ $bitness _ldstpair_pre>](
                        inc.offset.into(),
                        self.rt.1.code(),
                        base.code(),
                        self.rt.0.code(),
                    );
                    code
                }
            }
            impl crate::instructions::RawInstruction for $name<$rt, (RegOrSp64, Inc<$offset_type>)> {
                #[inline]
                fn to_code(&self) -> crate::InstructionCode {
                    let (base, inc) = self.addr;
                    let code = [<$mnem _ $bitness _ldstpair_post>](
                        inc.offset.into(),
                        self.rt.1.code(),
                        base.code(),
                        self.rt.0.code(),
                    );
                    code
                }
            }
        }
    }
}

#[macro_export]
macro_rules! define_pair_fallible_rules {
    ($mnem: ident, $name:ident, $trait_name:ident) => {
        #[doc = "`"]
        #[doc = stringify!($mnem)]
        #[doc = "` with fallible offset that delegates to non-faillible variants."]
        impl<RtInp1, RtInp2, RtOut, BaseInp, Ext, Err>
            $trait_name<RtInp1, RtInp2, (BaseInp, Result<Ext, Err>)>
            for $name<RtOut, ($crate::register::RegOrSp64, Ext)>
        where
            $name<RtOut, ($crate::register::RegOrSp64, Ext)>:
                $trait_name<RtInp1, RtInp2, (BaseInp, Ext)>,
            $crate::register::RegOrSp64: From<BaseInp>,
        {
            type Output = ::core::result::Result<
                <Self as $trait_name<RtInp1, RtInp2, (BaseInp, Ext)>>::Output,
                Err,
            >;

            #[inline]
            fn new(
                rt: (RtInp1, RtInp2),
                (base, offset): (BaseInp, ::core::result::Result<Ext, Err>),
            ) -> Self::Output {
                offset.map(|offset| $name::new(rt, (base, offset)))
            }
        }

        #[doc = "`"]
        #[doc = stringify!($mnem)]
        #[doc = "` with fallible offset that delegates to non-faillible variants."]
        impl<RtInp1, RtInp2, RtOut, BaseInp, Ext, Err>
            $trait_name<RtInp1, RtInp2, (::core::result::Result<Ext, Err>, BaseInp)>
            for $name<RtOut, (Ext, $crate::register::RegOrSp64)>
        where
            $name<RtOut, (Ext, $crate::register::RegOrSp64)>:
                $trait_name<RtInp1, RtInp2, (Ext, BaseInp)>,
            $crate::register::RegOrSp64: From<BaseInp>,
        {
            type Output = ::core::result::Result<
                <Self as $trait_name<RtInp1, RtInp2, (Ext, BaseInp)>>::Output,
                Err,
            >;

            #[inline]
            fn new(
                rt: (RtInp1, RtInp2),
                (ext, base): (::core::result::Result<Ext, Err>, BaseInp),
            ) -> Self::Output {
                ext.map(|ext| $name::new(rt, (ext, base)))
            }
        }

        #[doc = "`"]
        #[doc = stringify!($mnem)]
        #[doc = "` with fallible address that delegates to non-faillible variants."]
        impl<RtInp1, RtInp2, RtOut, Addr, Err>
            $trait_name<RtInp1, RtInp2, ::core::result::Result<Addr, Err>> for $name<RtOut, Addr>
        where
            $name<RtOut, Addr>: $trait_name<RtInp1, RtInp2, Addr>,
        {
            type Output =
                ::core::result::Result<<Self as $trait_name<RtInp1, RtInp2, Addr>>::Output, Err>;

            #[inline]
            fn new(rt: (RtInp1, RtInp2), addr: ::core::result::Result<Addr, Err>) -> Self::Output {
                addr.map(|addr| $name::new(rt, addr))
            }
        }
    };
}
