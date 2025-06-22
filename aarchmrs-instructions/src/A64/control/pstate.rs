/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MSR_SI_pstate {
    pub const OPCODE_MASK: u32 = 0b11111111111110001111000000011111u32;
    pub const OPCODE: u32 = 0b11010101000000000100000000011111u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MSR_SI_pstate";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MSR_SI_pstate {
        pub op1: ::aarchmrs_types::BitValue<3>,
        pub CRm: ::aarchmrs_types::BitValue<4>,
        pub op2: ::aarchmrs_types::BitValue<3>,
    }
    impl MSR_SI_pstate {
        #[inline]
        pub const fn new(
            op1: ::aarchmrs_types::BitValue<3>,
            CRm: ::aarchmrs_types::BitValue<4>,
            op2: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { op1, CRm, op2 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101010100000u32 << 19u32
                    | self.op1.into_inner() << 16u32
                    | 0b0100u32 << 12u32
                    | self.CRm.into_inner() << 8u32
                    | self.op2.into_inner() << 5u32
                    | 0b11111u32 << 0u32,
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
pub mod CFINV_M_pstate {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    pub const OPCODE: u32 = 0b11010101000000000100000000011111u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    pub const NAME: &str = "CFINV_M_pstate";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CFINV_M_pstate {}
    impl CFINV_M_pstate {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000000100000000011111u32 << 0u32,
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
pub mod XAFLAG_M_pstate {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    pub const OPCODE: u32 = 0b11010101000000000100000000111111u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    pub const NAME: &str = "XAFLAG_M_pstate";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct XAFLAG_M_pstate {}
    impl XAFLAG_M_pstate {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000000100000000111111u32 << 0u32,
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
pub mod AXFLAG_M_pstate {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    pub const OPCODE: u32 = 0b11010101000000000100000001011111u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    pub const NAME: &str = "AXFLAG_M_pstate";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AXFLAG_M_pstate {}
    impl AXFLAG_M_pstate {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000000100000001011111u32 << 0u32,
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
