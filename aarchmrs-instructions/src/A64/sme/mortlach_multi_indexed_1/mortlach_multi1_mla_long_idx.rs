/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlal_za_zzi_1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smlal_za_zzi_1 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub i3h: ::aarchmrs_types::BitValue<1>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i3l: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl smlal_za_zzi_1 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            i3h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i3l: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                i3h: i3h.into(),
                Rv: Rv.into(),
                i3l: i3l.into(),
                Zn: Zn.into(),
                U: U.into(),
                S: S.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011100u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | u32::from(self.i3h) << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b1u32 << 12u32
                    | u32::from(self.i3l) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.U) << 4u32
                    | u32::from(self.S) << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod smlsl_za_zzi_1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smlsl_za_zzi_1 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub i3h: ::aarchmrs_types::BitValue<1>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i3l: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl smlsl_za_zzi_1 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            i3h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i3l: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                i3h: i3h.into(),
                Rv: Rv.into(),
                i3l: i3l.into(),
                Zn: Zn.into(),
                U: U.into(),
                S: S.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011100u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | u32::from(self.i3h) << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b1u32 << 12u32
                    | u32::from(self.i3l) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.U) << 4u32
                    | u32::from(self.S) << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod umlal_za_zzi_1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umlal_za_zzi_1 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub i3h: ::aarchmrs_types::BitValue<1>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i3l: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl umlal_za_zzi_1 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            i3h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i3l: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                i3h: i3h.into(),
                Rv: Rv.into(),
                i3l: i3l.into(),
                Zn: Zn.into(),
                U: U.into(),
                S: S.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011100u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | u32::from(self.i3h) << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b1u32 << 12u32
                    | u32::from(self.i3l) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.U) << 4u32
                    | u32::from(self.S) << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod umlsl_za_zzi_1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umlsl_za_zzi_1 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub i3h: ::aarchmrs_types::BitValue<1>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i3l: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl umlsl_za_zzi_1 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            i3h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i3l: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                i3h: i3h.into(),
                Rv: Rv.into(),
                i3l: i3l.into(),
                Zn: Zn.into(),
                U: U.into(),
                S: S.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011100u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | u32::from(self.i3h) << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b1u32 << 12u32
                    | u32::from(self.i3l) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.U) << 4u32
                    | u32::from(self.S) << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
