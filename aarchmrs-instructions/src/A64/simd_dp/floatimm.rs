/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FMOV_S_floatimm {
    pub const OPCODE_MASK: u32 = 0b11111111111000000001111111100000u32;
    pub const OPCODE: u32 = 0b00011110001000000001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMOV_S_floatimm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMOV_S_floatimm {
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMOV_S_floatimm {
        #[inline]
        pub const fn new(
            imm8: ::aarchmrs_types::BitValue<8>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm8, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110001u32 << 21u32
                    | self.imm8.into_inner() << 13u32
                    | 0b10000000u32 << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod FMOV_D_floatimm {
    pub const OPCODE_MASK: u32 = 0b11111111111000000001111111100000u32;
    pub const OPCODE: u32 = 0b00011110011000000001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMOV_D_floatimm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMOV_D_floatimm {
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMOV_D_floatimm {
        #[inline]
        pub const fn new(
            imm8: ::aarchmrs_types::BitValue<8>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm8, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110011u32 << 21u32
                    | self.imm8.into_inner() << 13u32
                    | 0b10000000u32 << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod FMOV_H_floatimm {
    pub const OPCODE_MASK: u32 = 0b11111111111000000001111111100000u32;
    pub const OPCODE: u32 = 0b00011110111000000001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMOV_H_floatimm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMOV_H_floatimm {
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMOV_H_floatimm {
        #[inline]
        pub const fn new(
            imm8: ::aarchmrs_types::BitValue<8>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm8, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110111u32 << 21u32
                    | self.imm8.into_inner() << 13u32
                    | 0b10000000u32 << 5u32
                    | self.Rd.into_inner() << 0u32,
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
