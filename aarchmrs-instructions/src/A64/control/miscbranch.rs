/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod RETAASPPC_only_miscbranch {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01010101000000000000000000011111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RETAASPPC_only_miscbranch";
    #[inline]
    pub const fn RETAASPPC_only_miscbranch(
        imm16: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01010101000u32 << 21u32 | imm16.into_inner() << 5u32 | 0b11111u32 << 0u32,
        )
    }
}
pub mod RETABSPPC_only_miscbranch {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01010101001000000000000000011111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RETABSPPC_only_miscbranch";
    #[inline]
    pub const fn RETABSPPC_only_miscbranch(
        imm16: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01010101001u32 << 21u32 | imm16.into_inner() << 5u32 | 0b11111u32 << 0u32,
        )
    }
}
