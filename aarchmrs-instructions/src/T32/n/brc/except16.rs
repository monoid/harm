/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod UDF_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001101111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UDF_T1";
    #[inline]
    pub const fn UDF_T1(imm8: ::aarchmrs_types::BitValue<8>) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011110u32 << 8u32 | imm8.into_inner() << 0u32,
        )
    }
}
pub mod SVC_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001101111100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SVC_T1";
    #[inline]
    pub const fn SVC_T1(imm8: ::aarchmrs_types::BitValue<8>) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011111u32 << 8u32 | imm8.into_inner() << 0u32,
        )
    }
}
