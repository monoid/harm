/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADDG_64_addsub_immtags {
    pub const OPCODE_MASK: u32 = 0b11111111110000001100000000000000u32;
    pub const OPCODE: u32 = 0b10010001100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001100000000000000u32;
    pub const NAME: &str = "ADDG_64_addsub_immtags";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADDG_64_addsub_immtags {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADDG_64_addsub_immtags {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm6, imm4, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1001000110u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.imm4.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod SUBG_64_addsub_immtags {
    pub const OPCODE_MASK: u32 = 0b11111111110000001100000000000000u32;
    pub const OPCODE: u32 = 0b11010001100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001100000000000000u32;
    pub const NAME: &str = "SUBG_64_addsub_immtags";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUBG_64_addsub_immtags {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUBG_64_addsub_immtags {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm6, imm4, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101000110u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.imm4.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
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
