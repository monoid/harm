/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CBZ_32_compbranch {
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    pub const OPCODE: u32 = 0b00110100000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CBZ_32_compbranch";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBZ_32_compbranch {
        pub imm19: ::aarchmrs_types::BitValue<19>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBZ_32_compbranch {
        #[inline]
        pub const fn new(
            imm19: ::aarchmrs_types::BitValue<19>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm19, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00110100u32 << 24u32
                    | self.imm19.into_inner() << 5u32
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
pub mod CBNZ_32_compbranch {
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    pub const OPCODE: u32 = 0b00110101000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CBNZ_32_compbranch";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBNZ_32_compbranch {
        pub imm19: ::aarchmrs_types::BitValue<19>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBNZ_32_compbranch {
        #[inline]
        pub const fn new(
            imm19: ::aarchmrs_types::BitValue<19>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm19, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00110101u32 << 24u32
                    | self.imm19.into_inner() << 5u32
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
pub mod CBZ_64_compbranch {
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    pub const OPCODE: u32 = 0b10110100000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CBZ_64_compbranch";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBZ_64_compbranch {
        pub imm19: ::aarchmrs_types::BitValue<19>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBZ_64_compbranch {
        #[inline]
        pub const fn new(
            imm19: ::aarchmrs_types::BitValue<19>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm19, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10110100u32 << 24u32
                    | self.imm19.into_inner() << 5u32
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
pub mod CBNZ_64_compbranch {
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    pub const OPCODE: u32 = 0b10110101000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CBNZ_64_compbranch";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBNZ_64_compbranch {
        pub imm19: ::aarchmrs_types::BitValue<19>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBNZ_64_compbranch {
        #[inline]
        pub const fn new(
            imm19: ::aarchmrs_types::BitValue<19>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm19, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10110101u32 << 24u32
                    | self.imm19.into_inner() << 5u32
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
