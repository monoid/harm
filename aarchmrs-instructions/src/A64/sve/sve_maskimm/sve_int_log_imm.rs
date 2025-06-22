/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod orr_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111111111000000000000000000u32;
    pub const OPCODE: u32 = 0b00000101000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "orr_z_zi_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct orr_z_zi_ {
        pub imm13: ::aarchmrs_types::BitValue<13>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl orr_z_zi_ {
        #[inline]
        pub const fn new(
            imm13: ::aarchmrs_types::BitValue<13>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm13, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101000000u32 << 18u32
                    | self.imm13.into_inner() << 5u32
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
pub mod eor_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111111111000000000000000000u32;
    pub const OPCODE: u32 = 0b00000101010000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "eor_z_zi_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct eor_z_zi_ {
        pub imm13: ::aarchmrs_types::BitValue<13>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl eor_z_zi_ {
        #[inline]
        pub const fn new(
            imm13: ::aarchmrs_types::BitValue<13>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm13, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101010000u32 << 18u32
                    | self.imm13.into_inner() << 5u32
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
pub mod and_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111111111000000000000000000u32;
    pub const OPCODE: u32 = 0b00000101100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "and_z_zi_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct and_z_zi_ {
        pub imm13: ::aarchmrs_types::BitValue<13>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl and_z_zi_ {
        #[inline]
        pub const fn new(
            imm13: ::aarchmrs_types::BitValue<13>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm13, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101100000u32 << 18u32
                    | self.imm13.into_inner() << 5u32
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
