/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MOVN_32_movewide {
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    pub const OPCODE: u32 = 0b00010010100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MOVN_32_movewide";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MOVN_32_movewide {
        pub hw: ::aarchmrs_types::BitValue<1>,
        pub imm16: ::aarchmrs_types::BitValue<16>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MOVN_32_movewide {
        #[inline]
        pub const fn new(
            hw: ::aarchmrs_types::BitValue<1>,
            imm16: ::aarchmrs_types::BitValue<16>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { hw, imm16, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0001001010u32 << 22u32
                    | self.hw.into_inner() << 21u32
                    | self.imm16.into_inner() << 5u32
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
pub mod MOVZ_32_movewide {
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    pub const OPCODE: u32 = 0b01010010100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MOVZ_32_movewide";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MOVZ_32_movewide {
        pub hw: ::aarchmrs_types::BitValue<1>,
        pub imm16: ::aarchmrs_types::BitValue<16>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MOVZ_32_movewide {
        #[inline]
        pub const fn new(
            hw: ::aarchmrs_types::BitValue<1>,
            imm16: ::aarchmrs_types::BitValue<16>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { hw, imm16, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101001010u32 << 22u32
                    | self.hw.into_inner() << 21u32
                    | self.imm16.into_inner() << 5u32
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
pub mod MOVK_32_movewide {
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    pub const OPCODE: u32 = 0b01110010100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MOVK_32_movewide";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MOVK_32_movewide {
        pub hw: ::aarchmrs_types::BitValue<1>,
        pub imm16: ::aarchmrs_types::BitValue<16>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MOVK_32_movewide {
        #[inline]
        pub const fn new(
            hw: ::aarchmrs_types::BitValue<1>,
            imm16: ::aarchmrs_types::BitValue<16>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { hw, imm16, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0111001010u32 << 22u32
                    | self.hw.into_inner() << 21u32
                    | self.imm16.into_inner() << 5u32
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
pub mod MOVN_64_movewide {
    pub const OPCODE_MASK: u32 = 0b11111111100000000000000000000000u32;
    pub const OPCODE: u32 = 0b10010010100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MOVN_64_movewide";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MOVN_64_movewide {
        pub hw: ::aarchmrs_types::BitValue<2>,
        pub imm16: ::aarchmrs_types::BitValue<16>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MOVN_64_movewide {
        #[inline]
        pub const fn new(
            hw: ::aarchmrs_types::BitValue<2>,
            imm16: ::aarchmrs_types::BitValue<16>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { hw, imm16, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b100100101u32 << 23u32
                    | self.hw.into_inner() << 21u32
                    | self.imm16.into_inner() << 5u32
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
pub mod MOVZ_64_movewide {
    pub const OPCODE_MASK: u32 = 0b11111111100000000000000000000000u32;
    pub const OPCODE: u32 = 0b11010010100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MOVZ_64_movewide";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MOVZ_64_movewide {
        pub hw: ::aarchmrs_types::BitValue<2>,
        pub imm16: ::aarchmrs_types::BitValue<16>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MOVZ_64_movewide {
        #[inline]
        pub const fn new(
            hw: ::aarchmrs_types::BitValue<2>,
            imm16: ::aarchmrs_types::BitValue<16>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { hw, imm16, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110100101u32 << 23u32
                    | self.hw.into_inner() << 21u32
                    | self.imm16.into_inner() << 5u32
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
pub mod MOVK_64_movewide {
    pub const OPCODE_MASK: u32 = 0b11111111100000000000000000000000u32;
    pub const OPCODE: u32 = 0b11110010100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MOVK_64_movewide";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MOVK_64_movewide {
        pub hw: ::aarchmrs_types::BitValue<2>,
        pub imm16: ::aarchmrs_types::BitValue<16>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MOVK_64_movewide {
        #[inline]
        pub const fn new(
            hw: ::aarchmrs_types::BitValue<2>,
            imm16: ::aarchmrs_types::BitValue<16>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { hw, imm16, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b111100101u32 << 23u32
                    | self.hw.into_inner() << 21u32
                    | self.imm16.into_inner() << 5u32
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
