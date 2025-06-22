/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod incp_r_p_r_ {
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000000000u32;
    pub const OPCODE: u32 = 0b00100101001011001000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "incp_r_p_r_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct incp_r_p_r_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl incp_r_p_r_ {
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
                    | 0b1011001000100u32 << 9u32
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
pub mod decp_r_p_r_ {
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000000000u32;
    pub const OPCODE: u32 = 0b00100101001011011000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "decp_r_p_r_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct decp_r_p_r_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl decp_r_p_r_ {
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
                    | 0b1011011000100u32 << 9u32
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
