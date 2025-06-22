/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqincp_r_p_r_sx {
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000000000u32;
    pub const OPCODE: u32 = 0b00100101001010001000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqincp_r_p_r_sx";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqincp_r_p_r_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqincp_r_p_r_sx {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pm, Rdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1010001000100u32 << 9u32
                    | self.Pm.into_inner() << 5u32
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
pub mod uqincp_r_p_r_uw {
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000000000u32;
    pub const OPCODE: u32 = 0b00100101001010011000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqincp_r_p_r_uw";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqincp_r_p_r_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqincp_r_p_r_uw {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pm, Rdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1010011000100u32 << 9u32
                    | self.Pm.into_inner() << 5u32
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
pub mod sqdecp_r_p_r_sx {
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000000000u32;
    pub const OPCODE: u32 = 0b00100101001010101000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqdecp_r_p_r_sx";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdecp_r_p_r_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdecp_r_p_r_sx {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pm, Rdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1010101000100u32 << 9u32
                    | self.Pm.into_inner() << 5u32
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
pub mod uqdecp_r_p_r_uw {
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000000000u32;
    pub const OPCODE: u32 = 0b00100101001010111000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqdecp_r_p_r_uw";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdecp_r_p_r_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdecp_r_p_r_uw {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pm, Rdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1010111000100u32 << 9u32
                    | self.Pm.into_inner() << 5u32
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
pub mod sqincp_r_p_r_x {
    pub const OPCODE_MASK: u32 = 0b11111111001111101111111000000000u32;
    pub const OPCODE: u32 = 0b00100101001010001000110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqincp_r_p_r_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqincp_r_p_r_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqincp_r_p_r_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, U, Pm, Rdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10100u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b1000110u32 << 9u32
                    | self.Pm.into_inner() << 5u32
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
pub mod sqdecp_r_p_r_x {
    pub const OPCODE_MASK: u32 = 0b11111111001111101111111000000000u32;
    pub const OPCODE: u32 = 0b00100101001010101000110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqdecp_r_p_r_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdecp_r_p_r_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdecp_r_p_r_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, U, Pm, Rdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10101u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b1000110u32 << 9u32
                    | self.Pm.into_inner() << 5u32
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
pub mod uqincp_r_p_r_x {
    pub const OPCODE_MASK: u32 = 0b11111111001111101111111000000000u32;
    pub const OPCODE: u32 = 0b00100101001010001000110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqincp_r_p_r_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqincp_r_p_r_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqincp_r_p_r_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, U, Pm, Rdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10100u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b1000110u32 << 9u32
                    | self.Pm.into_inner() << 5u32
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
pub mod uqdecp_r_p_r_x {
    pub const OPCODE_MASK: u32 = 0b11111111001111101111111000000000u32;
    pub const OPCODE: u32 = 0b00100101001010101000110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqdecp_r_p_r_x";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdecp_r_p_r_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdecp_r_p_r_x {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, U, Pm, Rdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10101u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b1000110u32 << 9u32
                    | self.Pm.into_inner() << 5u32
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
