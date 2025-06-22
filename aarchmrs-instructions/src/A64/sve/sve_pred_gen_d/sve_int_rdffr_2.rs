/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod rdffr_p_f_ {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111110000u32;
    pub const OPCODE: u32 = 0b00100101000110011111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "rdffr_p_f_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct rdffr_p_f_ {
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl rdffr_p_f_ {
        #[inline]
        pub const fn new(Pd: ::aarchmrs_types::BitValue<4>) -> Self {
            Self { Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0010010100011001111100000000u32 << 4u32 | self.Pd.into_inner() << 0u32,
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
