/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod dupm_z_i_ {
    pub const OPCODE_MASK: u32 = 0b11111111111111000000000000000000u32;
    pub const OPCODE: u32 = 0b00000101110000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "dupm_z_i_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct dupm_z_i_ {
        pub imm13: ::aarchmrs_types::BitValue<13>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl dupm_z_i_ {
        #[inline]
        pub const fn new(
            imm13: ::aarchmrs_types::BitValue<13>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm13, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101110000u32 << 18u32
                    | self.imm13.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
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
