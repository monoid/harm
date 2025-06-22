/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod RMIF_only_rmif {
    pub const OPCODE_MASK: u32 = 0b11111111111000000111110000010000u32;
    pub const OPCODE: u32 = 0b10111010000000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "RMIF_only_rmif";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RMIF_only_rmif {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub mask: ::aarchmrs_types::BitValue<4>,
    }
    impl RMIF_only_rmif {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            mask: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { imm6, Rn, mask }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111010000u32 << 21u32
                    | self.imm6.into_inner() << 15u32
                    | 0b00001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.mask.into_inner() << 0u32,
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
