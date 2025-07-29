/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqrshr_z_mz2_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001111000001101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqrshr_z_mz2_";
    #[inline]
    pub const fn sqrshr_z_mz2_(
        imm4: ::aarchmrs_types::BitValue<4>,
        Zn: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b110101u32 << 10u32
                | Zn.into_inner() << 6u32
                | U.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqrshru_z_mz2_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001111100001101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqrshru_z_mz2_";
    #[inline]
    pub const fn sqrshru_z_mz2_(
        imm4: ::aarchmrs_types::BitValue<4>,
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011111u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b110101u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uqrshr_z_mz2_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001111000001101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqrshr_z_mz2_";
    #[inline]
    pub const fn uqrshr_z_mz2_(
        imm4: ::aarchmrs_types::BitValue<4>,
        Zn: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b110101u32 << 10u32
                | Zn.into_inner() << 6u32
                | U.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
