/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod index_z_ii_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct index_z_ii_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm5b: ::aarchmrs_types::BitValue<5>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl index_z_ii_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm5b: ::aarchmrs_types::BitValue<5>,
            imm5: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm5b,
                imm5,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.imm5b.into_inner() << 16u32
                    | 0b010000u32 << 10u32
                    | self.imm5.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
