/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod NOP_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001011111100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "NOP_T1";
    #[inline]
    pub const fn NOP_T1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b1011111100000000u32 << 0u32)
    }
}
pub mod YIELD_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001011111100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "YIELD_T1";
    #[inline]
    pub const fn YIELD_T1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b1011111100010000u32 << 0u32)
    }
}
pub mod WFE_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001011111100100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "WFE_T1";
    #[inline]
    pub const fn WFE_T1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b1011111100100000u32 << 0u32)
    }
}
pub mod WFI_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001011111100110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "WFI_T1";
    #[inline]
    pub const fn WFI_T1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b1011111100110000u32 << 0u32)
    }
}
pub mod SEV_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001011111101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SEV_T1";
    #[inline]
    pub const fn SEV_T1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b1011111101000000u32 << 0u32)
    }
}
pub mod SEVL_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001011111101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SEVL_T1";
    #[inline]
    pub const fn SEVL_T1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b1011111101010000u32 << 0u32)
    }
}
