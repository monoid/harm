/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fadd_za_zw_2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fadd_za_zw_2x2 {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fadd_za_zw_2x2 {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Rv: Rv.into(),
                Zm: Zm.into(),
                S: S.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b1000000u32 << 15u32
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
pub mod fsub_za_zw_2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fsub_za_zw_2x2 {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fsub_za_zw_2x2 {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Rv: Rv.into(),
                Zm: Zm.into(),
                S: S.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b1000000u32 << 15u32
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
