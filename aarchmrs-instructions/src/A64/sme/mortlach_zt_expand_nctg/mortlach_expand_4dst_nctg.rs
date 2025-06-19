/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod luti2_mz4_ztz_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct luti2_mz4_ztz_4 {
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub D: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<2>,
    }
    impl luti2_mz4_ztz_4 {
        #[inline]
        pub fn new(
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            D: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                i2: i2.into(),
                size: size.into(),
                Zn: Zn.into(),
                D: D.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000100111u32 << 18u32
                    | u32::from(self.i2) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.size) << 12u32
                    | 0b00u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.D) << 4u32
                    | 0b00u32 << 2u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod luti4_mz4_ztz_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct luti4_mz4_ztz_4 {
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub D: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<2>,
    }
    impl luti4_mz4_ztz_4 {
        #[inline]
        pub fn new(
            i1: impl Into<::aarchmrs_types::BitValue<1>>,
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            D: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                i1: i1.into(),
                size: size.into(),
                Zn: Zn.into(),
                D: D.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000001001101u32 << 17u32
                    | u32::from(self.i1) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.size) << 12u32
                    | 0b00u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.D) << 4u32
                    | 0b00u32 << 2u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
