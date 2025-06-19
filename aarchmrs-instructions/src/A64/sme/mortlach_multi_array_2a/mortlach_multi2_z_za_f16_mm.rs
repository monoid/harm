/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fadd_za_zw_2x2_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fadd_za_zw_2x2_16 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fadd_za_zw_2x2_16 {
        #[inline]
        pub fn new(
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Rv: Rv.into(),
                Zm: Zm.into(),
                S: S.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001101001000u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b111u32 << 10u32
                    | u32::from(self.Zm) << 6u32
                    | 0b00u32 << 4u32
                    | u32::from(self.S) << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod bfadd_za_zw_2x2_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfadd_za_zw_2x2_16 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl bfadd_za_zw_2x2_16 {
        #[inline]
        pub fn new(
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Rv: Rv.into(),
                Zm: Zm.into(),
                S: S.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001111001000u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b111u32 << 10u32
                    | u32::from(self.Zm) << 6u32
                    | 0b00u32 << 4u32
                    | u32::from(self.S) << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod fsub_za_zw_2x2_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fsub_za_zw_2x2_16 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fsub_za_zw_2x2_16 {
        #[inline]
        pub fn new(
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Rv: Rv.into(),
                Zm: Zm.into(),
                S: S.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001101001000u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b111u32 << 10u32
                    | u32::from(self.Zm) << 6u32
                    | 0b00u32 << 4u32
                    | u32::from(self.S) << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod bfsub_za_zw_2x2_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfsub_za_zw_2x2_16 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl bfsub_za_zw_2x2_16 {
        #[inline]
        pub fn new(
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Rv: Rv.into(),
                Zm: Zm.into(),
                S: S.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001111001000u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b111u32 << 10u32
                    | u32::from(self.Zm) << 6u32
                    | 0b00u32 << 4u32
                    | u32::from(self.S) << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
