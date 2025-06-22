/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod B_only_condbranch {
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000010000u32;
    pub const OPCODE: u32 = 0b01010100000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "B_only_condbranch";
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
pub mod BC_only_condbranch {
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000010000u32;
    pub const OPCODE: u32 = 0b01010100000000000000000000010000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BC_only_condbranch";
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
