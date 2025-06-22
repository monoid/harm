/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod B_only_branch_imm {
    pub const OPCODE_MASK: u32 = 0b11111100000000000000000000000000u32;
    pub const OPCODE: u32 = 0b00010100000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "B_only_branch_imm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct B_only_branch_imm {
        pub imm26: ::aarchmrs_types::BitValue<26>,
    }
    impl B_only_branch_imm {
        #[inline]
        pub const fn new(imm26: ::aarchmrs_types::BitValue<26>) -> Self {
            Self { imm26 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000101u32 << 26u32 | self.imm26.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod BL_only_branch_imm {
    pub const OPCODE_MASK: u32 = 0b11111100000000000000000000000000u32;
    pub const OPCODE: u32 = 0b10010100000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BL_only_branch_imm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BL_only_branch_imm {
        pub imm26: ::aarchmrs_types::BitValue<26>,
    }
    impl BL_only_branch_imm {
        #[inline]
        pub const fn new(imm26: ::aarchmrs_types::BitValue<26>) -> Self {
            Self { imm26 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b100101u32 << 26u32 | self.imm26.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
