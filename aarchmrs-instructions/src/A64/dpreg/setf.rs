/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SETF8_only_setf {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000011111u32;
    pub const OPCODE: u32 = 0b00111010000000000000100000001101u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETF8_only_setf";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETF8_only_setf {
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl SETF8_only_setf {
        #[inline]
        pub const fn new(Rn: ::aarchmrs_types::BitValue<5>) -> Self {
            Self { Rn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0011101000000000000010u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b01101u32 << 0u32,
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
pub mod SETF16_only_setf {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000011111u32;
    pub const OPCODE: u32 = 0b00111010000000000100100000001101u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETF16_only_setf";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETF16_only_setf {
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl SETF16_only_setf {
        #[inline]
        pub const fn new(Rn: ::aarchmrs_types::BitValue<5>) -> Self {
            Self { Rn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0011101000000000010010u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b01101u32 << 0u32,
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
