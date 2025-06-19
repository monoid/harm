/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod bfmlal_za_zzv_1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmlal_za_zzv_1 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl bfmlal_za_zzv_1 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                S: S.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010010u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b011u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.S) << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod fmlal_za_zzv_1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlal_za_zzv_1 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fmlal_za_zzv_1 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                S: S.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010010u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b011u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.S) << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod bfmlsl_za_zzv_1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmlsl_za_zzv_1 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl bfmlsl_za_zzv_1 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                S: S.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010010u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b011u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.S) << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod fmlsl_za_zzv_1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlsl_za_zzv_1 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fmlsl_za_zzv_1 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                Zn: Zn.into(),
                S: S.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010010u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b011u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.S) << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
