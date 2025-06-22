/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod AUTIASPPC_only_dp_1src_imm {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    pub const OPCODE: u32 = 0b11110011100000000000000000011111u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "AUTIASPPC_only_dp_1src_imm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIASPPC_only_dp_1src_imm {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl AUTIASPPC_only_dp_1src_imm {
        #[inline]
        pub const fn new(imm16: ::aarchmrs_types::BitValue<16>) -> Self {
            Self { imm16 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11110011100u32 << 21u32 | self.imm16.into_inner() << 5u32 | 0b11111u32 << 0u32,
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
pub mod AUTIBSPPC_only_dp_1src_imm {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    pub const OPCODE: u32 = 0b11110011101000000000000000011111u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "AUTIBSPPC_only_dp_1src_imm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIBSPPC_only_dp_1src_imm {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl AUTIBSPPC_only_dp_1src_imm {
        #[inline]
        pub const fn new(imm16: ::aarchmrs_types::BitValue<16>) -> Self {
            Self { imm16 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11110011101u32 << 21u32 | self.imm16.into_inner() << 5u32 | 0b11111u32 << 0u32,
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
