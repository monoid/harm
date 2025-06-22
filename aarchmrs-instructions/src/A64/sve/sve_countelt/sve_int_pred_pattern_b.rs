/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqincb_r_rs_sx {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqincb_r_rs_sx";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqincb_r_rs_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqincb_r_rs_sx {
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
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111100u32 << 10u32
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
pub mod uqincb_r_rs_uw {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqincb_r_rs_uw";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqincb_r_rs_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqincb_r_rs_uw {
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
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111101u32 << 10u32
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
pub mod sqdecb_r_rs_sx {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001111100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqdecb_r_rs_sx";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdecb_r_rs_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdecb_r_rs_sx {
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
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111110u32 << 10u32
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
pub mod uqdecb_r_rs_uw {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001111110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqdecb_r_rs_uw";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdecb_r_rs_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdecb_r_rs_uw {
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
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111111u32 << 10u32
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
pub mod sqincb_r_rs_x {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqincb_r_rs_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqincb_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqincb_r_rs_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                U,
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
                    | 0b11110u32 << 11u32
                    | self.U.into_inner() << 10u32
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
pub mod sqdecb_r_rs_x {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001111100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqdecb_r_rs_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdecb_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdecb_r_rs_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                U,
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
                    | 0b11111u32 << 11u32
                    | self.U.into_inner() << 10u32
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
pub mod sqinch_r_rs_sx {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqinch_r_rs_sx";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqinch_r_rs_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqinch_r_rs_sx {
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
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111100u32 << 10u32
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
pub mod uqinch_r_rs_uw {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqinch_r_rs_uw";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqinch_r_rs_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqinch_r_rs_uw {
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
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111101u32 << 10u32
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
pub mod sqdech_r_rs_sx {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001111100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqdech_r_rs_sx";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdech_r_rs_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdech_r_rs_sx {
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
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111110u32 << 10u32
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
pub mod uqdech_r_rs_uw {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001111110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqdech_r_rs_uw";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdech_r_rs_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdech_r_rs_uw {
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
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111111u32 << 10u32
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
pub mod sqinch_r_rs_x {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqinch_r_rs_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqinch_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqinch_r_rs_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                U,
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
                    | 0b11110u32 << 11u32
                    | self.U.into_inner() << 10u32
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
pub mod sqdech_r_rs_x {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001111100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqdech_r_rs_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdech_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdech_r_rs_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                U,
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
                    | 0b11111u32 << 11u32
                    | self.U.into_inner() << 10u32
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
pub mod sqincw_r_rs_sx {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqincw_r_rs_sx";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqincw_r_rs_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqincw_r_rs_sx {
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
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111100u32 << 10u32
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
pub mod uqincw_r_rs_uw {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqincw_r_rs_uw";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqincw_r_rs_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqincw_r_rs_uw {
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
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111101u32 << 10u32
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
pub mod sqdecw_r_rs_sx {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001111100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqdecw_r_rs_sx";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdecw_r_rs_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdecw_r_rs_sx {
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
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111110u32 << 10u32
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
pub mod uqdecw_r_rs_uw {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001111110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqdecw_r_rs_uw";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdecw_r_rs_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdecw_r_rs_uw {
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
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111111u32 << 10u32
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
pub mod sqincw_r_rs_x {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqincw_r_rs_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqincw_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqincw_r_rs_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                U,
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
                    | 0b11110u32 << 11u32
                    | self.U.into_inner() << 10u32
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
pub mod sqdecw_r_rs_x {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001111100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqdecw_r_rs_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdecw_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdecw_r_rs_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                U,
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
                    | 0b11111u32 << 11u32
                    | self.U.into_inner() << 10u32
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
pub mod sqincd_r_rs_sx {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqincd_r_rs_sx";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqincd_r_rs_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqincd_r_rs_sx {
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
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111100u32 << 10u32
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
pub mod uqincd_r_rs_uw {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqincd_r_rs_uw";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqincd_r_rs_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqincd_r_rs_uw {
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
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111101u32 << 10u32
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
pub mod sqdecd_r_rs_sx {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001111100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqdecd_r_rs_sx";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdecd_r_rs_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdecd_r_rs_sx {
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
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111110u32 << 10u32
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
pub mod uqdecd_r_rs_uw {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001111110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqdecd_r_rs_uw";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdecd_r_rs_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdecd_r_rs_uw {
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
                    | 0b10u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111111u32 << 10u32
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
pub mod sqincd_r_rs_x {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqincd_r_rs_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqincd_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqincd_r_rs_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                U,
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
                    | 0b11110u32 << 11u32
                    | self.U.into_inner() << 10u32
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
pub mod sqdecd_r_rs_x {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001111100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqdecd_r_rs_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdecd_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdecd_r_rs_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                U,
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
                    | 0b11111u32 << 11u32
                    | self.U.into_inner() << 10u32
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
pub mod uqincb_r_rs_x {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqincb_r_rs_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqincb_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqincb_r_rs_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                U,
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
                    | 0b11110u32 << 11u32
                    | self.U.into_inner() << 10u32
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
pub mod uqdecb_r_rs_x {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001111100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqdecb_r_rs_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdecb_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdecb_r_rs_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                U,
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
                    | 0b11111u32 << 11u32
                    | self.U.into_inner() << 10u32
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
pub mod uqinch_r_rs_x {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqinch_r_rs_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqinch_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqinch_r_rs_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                U,
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
                    | 0b11110u32 << 11u32
                    | self.U.into_inner() << 10u32
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
pub mod uqdech_r_rs_x {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001111100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqdech_r_rs_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdech_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdech_r_rs_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                U,
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
                    | 0b11111u32 << 11u32
                    | self.U.into_inner() << 10u32
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
pub mod uqincw_r_rs_x {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqincw_r_rs_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqincw_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqincw_r_rs_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                U,
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
                    | 0b11110u32 << 11u32
                    | self.U.into_inner() << 10u32
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
pub mod uqdecw_r_rs_x {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001111100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqdecw_r_rs_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdecw_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdecw_r_rs_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                U,
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
                    | 0b11111u32 << 11u32
                    | self.U.into_inner() << 10u32
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
pub mod uqincd_r_rs_x {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqincd_r_rs_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqincd_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqincd_r_rs_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                U,
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
                    | 0b11110u32 << 11u32
                    | self.U.into_inner() << 10u32
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
pub mod uqdecd_r_rs_x {
    pub const OPCODE_MASK: u32 = 0b11111111001100001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100001100001111100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqdecd_r_rs_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdecd_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdecd_r_rs_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                U,
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
                    | 0b11111u32 << 11u32
                    | self.U.into_inner() << 10u32
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
