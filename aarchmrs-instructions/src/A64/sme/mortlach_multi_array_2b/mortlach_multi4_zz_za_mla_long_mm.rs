/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlal_za_zzw_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smlal_za_zzw_4x4 {
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl smlal_za_zzw_4x4 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                U: U.into(),
                S: S.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001111u32 << 21u32
                    | u32::from(self.Zm) << 18u32
                    | 0b010u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b010u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b00u32 << 5u32
                    | u32::from(self.U) << 4u32
                    | u32::from(self.S) << 3u32
                    | 0b0u32 << 2u32
                    | u32::from(self.off2) << 0u32,
            )
        }
    }
}
pub mod smlsl_za_zzw_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smlsl_za_zzw_4x4 {
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl smlsl_za_zzw_4x4 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                U: U.into(),
                S: S.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001111u32 << 21u32
                    | u32::from(self.Zm) << 18u32
                    | 0b010u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b010u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b00u32 << 5u32
                    | u32::from(self.U) << 4u32
                    | u32::from(self.S) << 3u32
                    | 0b0u32 << 2u32
                    | u32::from(self.off2) << 0u32,
            )
        }
    }
}
pub mod umlal_za_zzw_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umlal_za_zzw_4x4 {
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl umlal_za_zzw_4x4 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                U: U.into(),
                S: S.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001111u32 << 21u32
                    | u32::from(self.Zm) << 18u32
                    | 0b010u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b010u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b00u32 << 5u32
                    | u32::from(self.U) << 4u32
                    | u32::from(self.S) << 3u32
                    | 0b0u32 << 2u32
                    | u32::from(self.off2) << 0u32,
            )
        }
    }
}
pub mod umlsl_za_zzw_4x4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umlsl_za_zzw_4x4 {
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl umlsl_za_zzw_4x4 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                U: U.into(),
                S: S.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001111u32 << 21u32
                    | u32::from(self.Zm) << 18u32
                    | 0b010u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b010u32 << 10u32
                    | u32::from(self.Zn) << 7u32
                    | 0b00u32 << 5u32
                    | u32::from(self.U) << 4u32
                    | u32::from(self.S) << 3u32
                    | 0b0u32 << 2u32
                    | u32::from(self.off2) << 0u32,
            )
        }
    }
}
