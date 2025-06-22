/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SVC_EX_exception {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    pub const OPCODE: u32 = 0b11010100000000000000000000000001u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SVC_EX_exception";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SVC_EX_exception {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl SVC_EX_exception {
        #[inline]
        pub const fn new(imm16: ::aarchmrs_types::BitValue<16>) -> Self {
            Self { imm16 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010100000u32 << 21u32 | self.imm16.into_inner() << 5u32 | 0b00001u32 << 0u32,
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
pub mod HVC_EX_exception {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    pub const OPCODE: u32 = 0b11010100000000000000000000000010u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "HVC_EX_exception";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct HVC_EX_exception {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl HVC_EX_exception {
        #[inline]
        pub const fn new(imm16: ::aarchmrs_types::BitValue<16>) -> Self {
            Self { imm16 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010100000u32 << 21u32 | self.imm16.into_inner() << 5u32 | 0b00010u32 << 0u32,
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
pub mod SMC_EX_exception {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    pub const OPCODE: u32 = 0b11010100000000000000000000000011u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SMC_EX_exception";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SMC_EX_exception {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl SMC_EX_exception {
        #[inline]
        pub const fn new(imm16: ::aarchmrs_types::BitValue<16>) -> Self {
            Self { imm16 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010100000u32 << 21u32 | self.imm16.into_inner() << 5u32 | 0b00011u32 << 0u32,
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
pub mod BRK_EX_exception {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    pub const OPCODE: u32 = 0b11010100001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BRK_EX_exception";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BRK_EX_exception {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl BRK_EX_exception {
        #[inline]
        pub const fn new(imm16: ::aarchmrs_types::BitValue<16>) -> Self {
            Self { imm16 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010100001u32 << 21u32 | self.imm16.into_inner() << 5u32 | 0b00000u32 << 0u32,
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
pub mod HLT_EX_exception {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    pub const OPCODE: u32 = 0b11010100010000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "HLT_EX_exception";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct HLT_EX_exception {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl HLT_EX_exception {
        #[inline]
        pub const fn new(imm16: ::aarchmrs_types::BitValue<16>) -> Self {
            Self { imm16 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010100010u32 << 21u32 | self.imm16.into_inner() << 5u32 | 0b00000u32 << 0u32,
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
pub mod TCANCEL_EX_exception {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    pub const OPCODE: u32 = 0b11010100011000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "TCANCEL_EX_exception";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TCANCEL_EX_exception {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl TCANCEL_EX_exception {
        #[inline]
        pub const fn new(imm16: ::aarchmrs_types::BitValue<16>) -> Self {
            Self { imm16 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010100011u32 << 21u32 | self.imm16.into_inner() << 5u32 | 0b00000u32 << 0u32,
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
pub mod DCPS1_DC_exception {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    pub const OPCODE: u32 = 0b11010100101000000000000000000001u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "DCPS1_DC_exception";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct DCPS1_DC_exception {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl DCPS1_DC_exception {
        #[inline]
        pub const fn new(imm16: ::aarchmrs_types::BitValue<16>) -> Self {
            Self { imm16 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010100101u32 << 21u32 | self.imm16.into_inner() << 5u32 | 0b00001u32 << 0u32,
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
pub mod DCPS2_DC_exception {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    pub const OPCODE: u32 = 0b11010100101000000000000000000010u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "DCPS2_DC_exception";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct DCPS2_DC_exception {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl DCPS2_DC_exception {
        #[inline]
        pub const fn new(imm16: ::aarchmrs_types::BitValue<16>) -> Self {
            Self { imm16 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010100101u32 << 21u32 | self.imm16.into_inner() << 5u32 | 0b00010u32 << 0u32,
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
pub mod DCPS3_DC_exception {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    pub const OPCODE: u32 = 0b11010100101000000000000000000011u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "DCPS3_DC_exception";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct DCPS3_DC_exception {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl DCPS3_DC_exception {
        #[inline]
        pub const fn new(imm16: ::aarchmrs_types::BitValue<16>) -> Self {
            Self { imm16 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010100101u32 << 21u32 | self.imm16.into_inner() << 5u32 | 0b00011u32 << 0u32,
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
