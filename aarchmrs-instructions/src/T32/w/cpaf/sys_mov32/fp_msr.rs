/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod VMSR_T1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101110111000000000101000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000011101111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMSR_T1_AS";
    #[inline]
    pub const fn VMSR_T1_AS(
        reg: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111011101110u32 << 20u32
                | reg.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b101000010000u32 << 0u32,
        )
    }
}
pub mod VMRS_T1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101110111100000000101000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000011101111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMRS_T1_AS";
    #[inline]
    pub const fn VMRS_T1_AS(
        reg: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111011101111u32 << 20u32
                | reg.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b101000010000u32 << 0u32,
        )
    }
}
