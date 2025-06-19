/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod dupq_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct dupq_z_zi_ {
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub tsz: ::aarchmrs_types::BitValue<4>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl dupq_z_zi_ {
        #[inline]
        pub const fn new(
            i1: ::aarchmrs_types::BitValue<1>,
            tsz: ::aarchmrs_types::BitValue<4>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { i1, tsz, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101001u32 << 21u32
                    | self.i1.into_inner() << 20u32
                    | self.tsz.into_inner() << 16u32
                    | 0b001001u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
