/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod extq_z_zi_des {
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    pub const OPCODE: u32 = 0b00000101011000000010010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "extq_z_zi_des";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct extq_z_zi_des {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl extq_z_zi_des {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm4, Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000001010110u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b001001u32 << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
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
