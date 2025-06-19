/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod luti4_mz4_ztmz2_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct luti4_mz4_ztmz2_4 {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub D: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<2>,
    }
    impl luti4_mz4_ztmz2_4 {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<4>,
            D: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { size, Zn, D, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000001001101100u32 << 14u32
                    | self.size.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0u32 << 5u32
                    | self.D.into_inner() << 4u32
                    | 0b00u32 << 2u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
