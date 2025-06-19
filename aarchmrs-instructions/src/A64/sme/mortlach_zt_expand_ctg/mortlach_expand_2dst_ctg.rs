/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod luti2_mz2_ztz_1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct luti2_mz2_ztz_1 {
        pub i3: ::aarchmrs_types::BitValue<3>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<4>,
    }
    impl luti2_mz2_ztz_1 {
        #[inline]
        pub fn new(
            i3: impl Into<::aarchmrs_types::BitValue<3>>,
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                i3: i3.into(),
                size: size.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000100011u32 << 18u32
                    | u32::from(self.i3) << 15u32
                    | 0b1u32 << 14u32
                    | u32::from(self.size) << 12u32
                    | 0b00u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
pub mod luti4_mz2_ztz_1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct luti4_mz2_ztz_1 {
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<4>,
    }
    impl luti4_mz2_ztz_1 {
        #[inline]
        pub fn new(
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                i2: i2.into(),
                size: size.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000001000101u32 << 17u32
                    | u32::from(self.i2) << 15u32
                    | 0b1u32 << 14u32
                    | u32::from(self.size) << 12u32
                    | 0b00u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
