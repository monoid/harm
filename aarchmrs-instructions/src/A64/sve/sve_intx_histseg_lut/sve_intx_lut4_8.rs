/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod luti4_z_zz_8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct luti4_z_zz_8 {
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl luti4_z_zz_8 {
        #[inline]
        pub const fn new(
            i1: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { i1, Zm, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101u32 << 24u32
                    | self.i1.into_inner() << 23u32
                    | 0b11u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b101001u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
