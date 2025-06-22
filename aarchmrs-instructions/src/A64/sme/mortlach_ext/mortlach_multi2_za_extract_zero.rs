/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod movaz_mz_za2_1 {
    pub const OPCODE_MASK: u32 = 0b11111111111111111001111100000001u32;
    pub const OPCODE: u32 = 0b11000000000001100000101000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "movaz_mz_za2_1";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct movaz_mz_za2_1 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub off3: ::aarchmrs_types::BitValue<3>,
        pub Zd: ::aarchmrs_types::BitValue<4>,
    }
    impl movaz_mz_za2_1 {
        #[inline]
        pub const fn new(
            Rv: ::aarchmrs_types::BitValue<2>,
            off3: ::aarchmrs_types::BitValue<3>,
            Zd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Rv, off3, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000000001100u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b01010u32 << 8u32
                    | self.off3.into_inner() << 5u32
                    | self.Zd.into_inner() << 1u32
                    | 0b0u32 << 0u32,
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
