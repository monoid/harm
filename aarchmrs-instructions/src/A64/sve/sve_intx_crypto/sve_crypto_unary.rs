/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod aesmc_z_z_ {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111100000u32;
    pub const OPCODE: u32 = 0b01000101001000001110000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "aesmc_z_z_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct aesmc_z_z_ {
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl aesmc_z_z_ {
        #[inline]
        pub const fn new(Zdn: ::aarchmrs_types::BitValue<5>) -> Self {
            Self { Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010010000011100000000u32 << 5u32 | self.Zdn.into_inner() << 0u32,
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
pub mod aesimc_z_z_ {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111100000u32;
    pub const OPCODE: u32 = 0b01000101001000001110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "aesimc_z_z_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct aesimc_z_z_ {
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl aesimc_z_z_ {
        #[inline]
        pub const fn new(Zdn: ::aarchmrs_types::BitValue<5>) -> Self {
            Self { Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010010000011100100000u32 << 5u32 | self.Zdn.into_inner() << 0u32,
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
