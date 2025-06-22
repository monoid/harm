/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cntb_r_s_ {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001110000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "cntb_r_s_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cntb_r_s_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl cntb_r_s_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111000u32 << 10u32
                    | self.pattern.into_inner() << 5u32
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
pub mod cnth_r_s_ {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001110000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "cnth_r_s_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cnth_r_s_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl cnth_r_s_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111000u32 << 10u32
                    | self.pattern.into_inner() << 5u32
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
pub mod cntw_r_s_ {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001110000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "cntw_r_s_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cntw_r_s_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl cntw_r_s_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111000u32 << 10u32
                    | self.pattern.into_inner() << 5u32
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
pub mod cntd_r_s_ {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001110000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "cntd_r_s_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cntd_r_s_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl cntd_r_s_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111000u32 << 10u32
                    | self.pattern.into_inner() << 5u32
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
