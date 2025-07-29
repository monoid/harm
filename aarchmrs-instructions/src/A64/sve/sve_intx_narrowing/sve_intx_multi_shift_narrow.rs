/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqrshrn_z_mz2_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110110000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101101100000010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqrshrn_z_mz2_";
    #[inline]
    pub const fn sqrshrn_z_mz2_(
        imm4: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001011011u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b001u32 << 13u32
                | U.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqrshrun_z_mz2_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101101100000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqrshrun_z_mz2_";
    #[inline]
    pub const fn sqrshrun_z_mz2_(
        imm4: ::aarchmrs_types::BitValue<4>,
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001011011u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b000010u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uqrshrn_z_mz2_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110110000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101101100000010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqrshrn_z_mz2_";
    #[inline]
    pub const fn uqrshrn_z_mz2_(
        imm4: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001011011u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b001u32 << 13u32
                | U.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
