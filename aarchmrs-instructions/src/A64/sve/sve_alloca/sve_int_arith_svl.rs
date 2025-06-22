/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod addsvl_r_ri_ {
    pub const OPCODE_MASK: u32 = 0b11111111111000001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100001000000101100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "addsvl_r_ri_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct addsvl_r_ri_ {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl addsvl_r_ri_ {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            imm6: ::aarchmrs_types::BitValue<6>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rn, imm6, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100001u32 << 21u32
                    | self.Rn.into_inner() << 16u32
                    | 0b01011u32 << 11u32
                    | self.imm6.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod addspl_r_ri_ {
    pub const OPCODE_MASK: u32 = 0b11111111111000001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100011000000101100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "addspl_r_ri_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct addspl_r_ri_ {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl addspl_r_ri_ {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            imm6: ::aarchmrs_types::BitValue<6>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rn, imm6, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100011u32 << 21u32
                    | self.Rn.into_inner() << 16u32
                    | 0b01011u32 << 11u32
                    | self.imm6.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
