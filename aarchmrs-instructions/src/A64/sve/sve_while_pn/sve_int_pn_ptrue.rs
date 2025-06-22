/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ptrue_pn_i_ {
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111111111000u32;
    pub const OPCODE: u32 = 0b00100101001000000111100000010000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ptrue_pn_i_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ptrue_pn_i_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub PNd: ::aarchmrs_types::BitValue<3>,
    }
    impl ptrue_pn_i_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            PNd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { size, PNd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1000000111100000010u32 << 3u32
                    | self.PNd.into_inner() << 0u32,
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
