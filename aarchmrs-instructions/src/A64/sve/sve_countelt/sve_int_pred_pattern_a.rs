/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod incb_r_rs_ {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001110000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "incb_r_rs_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct incb_r_rs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl incb_r_rs_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b11u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111000u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
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
pub mod decb_r_rs_ {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "decb_r_rs_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct decb_r_rs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl decb_r_rs_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b11u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
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
pub mod inch_r_rs_ {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001110000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "inch_r_rs_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct inch_r_rs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl inch_r_rs_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b11u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111000u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
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
pub mod dech_r_rs_ {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "dech_r_rs_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct dech_r_rs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl dech_r_rs_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b11u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
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
pub mod incw_r_rs_ {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001110000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "incw_r_rs_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct incw_r_rs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl incw_r_rs_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b11u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111000u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
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
pub mod decw_r_rs_ {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "decw_r_rs_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct decw_r_rs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl decw_r_rs_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b11u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
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
pub mod incd_r_rs_ {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001110000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "incd_r_rs_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct incd_r_rs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl incd_r_rs_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b11u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111000u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
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
pub mod decd_r_rs_ {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "decd_r_rs_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct decd_r_rs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl decd_r_rs_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b11u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
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
