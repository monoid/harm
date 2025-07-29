/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod aesmc_z_z_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "aesmc_z_z_";
    #[inline]
    pub const fn aesmc_z_z_(
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010010000011100000000u32 << 5u32 | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod aesimc_z_z_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "aesimc_z_z_";
    #[inline]
    pub const fn aesimc_z_z_(
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010010000011100100000u32 << 5u32 | Zdn.into_inner() << 0u32,
        )
    }
}
