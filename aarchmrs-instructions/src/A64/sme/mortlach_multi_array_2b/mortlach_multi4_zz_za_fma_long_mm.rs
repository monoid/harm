/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod bfmlal_za_zzw_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmlal_za_zzw_4x4 {
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl bfmlal_za_zzw_4x4 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                S: S.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001101u32 << 21u32
                    | u32::from(self.Zm) << 18u32
                    | 0b010u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b010u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b001u32 << 4u32
                    | u32::from(self.S) << 3u32
                    | 0b0u32 << 2u32
                    | u32::from(self.off2) << 0u32,
            )
        }
    }
}
pub mod fmlal_za_zzw_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlal_za_zzw_4x4 {
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl fmlal_za_zzw_4x4 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                S: S.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001101u32 << 21u32
                    | u32::from(self.Zm) << 18u32
                    | 0b010u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b010u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b000u32 << 4u32
                    | u32::from(self.S) << 3u32
                    | 0b0u32 << 2u32
                    | u32::from(self.off2) << 0u32,
            )
        }
    }
}
pub mod bfmlsl_za_zzw_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmlsl_za_zzw_4x4 {
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl bfmlsl_za_zzw_4x4 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                S: S.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001101u32 << 21u32
                    | u32::from(self.Zm) << 18u32
                    | 0b010u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b010u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b001u32 << 4u32
                    | u32::from(self.S) << 3u32
                    | 0b0u32 << 2u32
                    | u32::from(self.off2) << 0u32,
            )
        }
    }
}
pub mod fmlsl_za_zzw_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlsl_za_zzw_4x4 {
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl fmlsl_za_zzw_4x4 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                S: S.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001101u32 << 21u32
                    | u32::from(self.Zm) << 18u32
                    | 0b010u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b010u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b000u32 << 4u32
                    | u32::from(self.S) << 3u32
                    | 0b0u32 << 2u32
                    | u32::from(self.off2) << 0u32,
            )
        }
    }
}
