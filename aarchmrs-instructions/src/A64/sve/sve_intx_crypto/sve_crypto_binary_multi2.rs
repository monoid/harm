/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod aese_mz_zzi_2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111001111111110000000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000101110100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "aese_mz_zzi_2x1";
    #[inline]
    pub const fn aese_mz_zzi_2x1(
        i2: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101001u32 << 21u32
                | i2.into_inner() << 19u32
                | 0b010111010u32 << 10u32
                | Zm.into_inner() << 5u32
                | Zdn.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod aesd_mz_zzi_2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111001111111110000000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000101110110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "aesd_mz_zzi_2x1";
    #[inline]
    pub const fn aesd_mz_zzi_2x1(
        i2: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101001u32 << 21u32
                | i2.into_inner() << 19u32
                | 0b010111011u32 << 10u32
                | Zm.into_inner() << 5u32
                | Zdn.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod aesemc_mz_zzi_2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111001111111110000000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000111110100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "aesemc_mz_zzi_2x1";
    #[inline]
    pub const fn aesemc_mz_zzi_2x1(
        i2: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101001u32 << 21u32
                | i2.into_inner() << 19u32
                | 0b011111010u32 << 10u32
                | Zm.into_inner() << 5u32
                | Zdn.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod aesdimc_mz_zzi_2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111001111111110000000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "aesdimc_mz_zzi_2x1";
    #[inline]
    pub const fn aesdimc_mz_zzi_2x1(
        i2: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101001u32 << 21u32
                | i2.into_inner() << 19u32
                | 0b011111011u32 << 10u32
                | Zm.into_inner() << 5u32
                | Zdn.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
