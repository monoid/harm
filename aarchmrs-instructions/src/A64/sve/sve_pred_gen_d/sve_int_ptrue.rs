/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ptrue_p_s_ {
    pub const OPCODE_MASK: u32 = 0b11111111001111101111110000010000u32;
    pub const OPCODE: u32 = 0b00100101000110001110000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ptrue_p_s_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ptrue_p_s_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl ptrue_p_s_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            S: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                size,
                S,
                pattern,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b01100u32 << 17u32
                    | self.S.into_inner() << 16u32
                    | 0b111000u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
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
pub mod ptrues_p_s_ {
    pub const OPCODE_MASK: u32 = 0b11111111001111101111110000010000u32;
    pub const OPCODE: u32 = 0b00100101000110001110000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ptrues_p_s_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ptrues_p_s_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl ptrues_p_s_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            S: ::aarchmrs_types::BitValue<1>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                size,
                S,
                pattern,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b01100u32 << 17u32
                    | self.S.into_inner() << 16u32
                    | 0b111000u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
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
