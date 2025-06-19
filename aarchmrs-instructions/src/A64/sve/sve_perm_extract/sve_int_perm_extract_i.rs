/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ext_z_zi_des {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ext_z_zi_des {
        pub imm8h: ::aarchmrs_types::BitValue<5>,
        pub imm8l: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl ext_z_zi_des {
        #[inline]
        pub const fn new(
            imm8h: ::aarchmrs_types::BitValue<5>,
            imm8l: ::aarchmrs_types::BitValue<3>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                imm8h,
                imm8l,
                Zm,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101001u32 << 21u32
                    | self.imm8h.into_inner() << 16u32
                    | 0b000u32 << 13u32
                    | self.imm8l.into_inner() << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
