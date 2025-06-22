/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod str_p_bi_ {
    pub const OPCODE_MASK: u32 = 0b11111111110000001110000000010000u32;
    pub const OPCODE: u32 = 0b11100101100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "str_p_bi_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct str_p_bi_ {
        pub imm9h: ::aarchmrs_types::BitValue<6>,
        pub imm9l: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Pt: ::aarchmrs_types::BitValue<4>,
    }
    impl str_p_bi_ {
        #[inline]
        pub const fn new(
            imm9h: ::aarchmrs_types::BitValue<6>,
            imm9l: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Pt: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm9h,
                imm9l,
                Rn,
                Pt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110010110u32 << 22u32
                    | self.imm9h.into_inner() << 16u32
                    | 0b000u32 << 13u32
                    | self.imm9l.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pt.into_inner() << 0u32,
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
