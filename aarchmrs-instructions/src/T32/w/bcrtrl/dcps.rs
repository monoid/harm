/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod DCPS1_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110111100011111000000000000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "DCPS1_T1";
    #[inline]
    pub const fn DCPS1_T1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110111100011111000000000000001u32 << 0u32)
    }
}
pub mod DCPS2_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110111100011111000000000000010u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "DCPS2_T1";
    #[inline]
    pub const fn DCPS2_T1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110111100011111000000000000010u32 << 0u32)
    }
}
pub mod DCPS3_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110111100011111000000000000011u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "DCPS3_T1";
    #[inline]
    pub const fn DCPS3_T1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110111100011111000000000000011u32 << 0u32)
    }
}
