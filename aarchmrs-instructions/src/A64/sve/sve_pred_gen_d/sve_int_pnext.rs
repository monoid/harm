/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod pnext_p_p_p_ {
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000010000u32;
    pub const OPCODE: u32 = 0b00100101000110011100010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "pnext_p_p_p_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct pnext_p_p_p_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pv: ::aarchmrs_types::BitValue<4>,
        pub Pdn: ::aarchmrs_types::BitValue<4>,
    }
    impl pnext_p_p_p_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pv: ::aarchmrs_types::BitValue<4>,
            Pdn: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { size, Pv, Pdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b0110011100010u32 << 9u32
                    | self.Pv.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.Pdn.into_inner() << 0u32,
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
