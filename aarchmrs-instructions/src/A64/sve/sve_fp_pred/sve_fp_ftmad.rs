/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ftmad_z_zzi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ftmad_z_zzi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl ftmad_z_zzi_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm3,
                Zm,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b010u32 << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b100000u32 << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
