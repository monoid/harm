/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod movt_r_zt_ {
    pub const OPCODE_MASK: u32 = 0b11111111111111111000111111100000u32;
    pub const OPCODE: u32 = 0b11000000010011000000001111100000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "movt_r_zt_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct movt_r_zt_ {
        pub off3: ::aarchmrs_types::BitValue<3>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl movt_r_zt_ {
        #[inline]
        pub const fn new(
            off3: ::aarchmrs_types::BitValue<3>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { off3, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000010011000u32 << 15u32
                    | self.off3.into_inner() << 12u32
                    | 0b0011111u32 << 5u32
                    | self.Rt.into_inner() << 0u32,
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
