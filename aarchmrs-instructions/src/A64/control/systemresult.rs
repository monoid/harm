/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod TSTART_BR_systemresult {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111100000u32;
    pub const OPCODE: u32 = 0b11010101001000110011000001100000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "TSTART_BR_systemresult";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TSTART_BR_systemresult {
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl TSTART_BR_systemresult {
        #[inline]
        pub const fn new(Rt: ::aarchmrs_types::BitValue<5>) -> Self {
            Self { Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110101010010001100110000011u32 << 5u32 | self.Rt.into_inner() << 0u32,
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
pub mod TTEST_BR_systemresult {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111100000u32;
    pub const OPCODE: u32 = 0b11010101001000110011000101100000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "TTEST_BR_systemresult";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TTEST_BR_systemresult {
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl TTEST_BR_systemresult {
        #[inline]
        pub const fn new(Rt: ::aarchmrs_types::BitValue<5>) -> Self {
            Self { Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110101010010001100110001011u32 << 5u32 | self.Rt.into_inner() << 0u32,
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
