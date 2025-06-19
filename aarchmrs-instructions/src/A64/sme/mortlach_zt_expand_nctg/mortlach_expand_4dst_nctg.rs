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
        pub const fn new(
            i2: ::aarchmrs_types::BitValue<2>,
            size: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            D: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                i2,
                size,
                Zn,
                D,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000100111u32 << 18u32
                    | self.i2.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.size.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.D.into_inner() << 4u32
                    | 0b00u32 << 2u32
                    | self.Zd.into_inner() << 0u32,
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
        pub const fn new(
            i1: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            D: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                i1,
                size,
                Zn,
                D,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000001001101u32 << 17u32
                    | self.i1.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.size.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.D.into_inner() << 4u32
                    | 0b00u32 << 2u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
