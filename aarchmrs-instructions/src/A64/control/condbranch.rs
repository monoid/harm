/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod B_only_condbranch {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct B_only_condbranch {
        pub imm19: ::aarchmrs_types::BitValue<19>,
        pub cond: ::aarchmrs_types::BitValue<4>,
    }
    impl B_only_condbranch {
        #[inline]
        pub const fn new(
            imm19: ::aarchmrs_types::BitValue<19>,
            cond: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { imm19, cond }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01010100u32 << 24u32
                    | self.imm19.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.cond.into_inner() << 0u32,
            )
        }
    }
}
pub mod BC_only_condbranch {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BC_only_condbranch {
        pub imm19: ::aarchmrs_types::BitValue<19>,
        pub cond: ::aarchmrs_types::BitValue<4>,
    }
    impl BC_only_condbranch {
        #[inline]
        pub const fn new(
            imm19: ::aarchmrs_types::BitValue<19>,
            cond: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { imm19, cond }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01010100u32 << 24u32
                    | self.imm19.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.cond.into_inner() << 0u32,
            )
        }
    }
}
