/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CBNZ_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111110100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001011100100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBNZ_T1";
    #[inline]
    pub const fn CBNZ_T1(
        i: ::aarchmrs_types::BitValue<1>,
        imm5: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101110u32 << 10u32
                | i.into_inner() << 9u32
                | 0b1u32 << 8u32
                | imm5.into_inner() << 3u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod CBZ_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111110100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001011000100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBZ_T1";
    #[inline]
    pub const fn CBZ_T1(
        i: ::aarchmrs_types::BitValue<1>,
        imm5: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101100u32 << 10u32
                | i.into_inner() << 9u32
                | 0b1u32 << 8u32
                | imm5.into_inner() << 3u32
                | Rn.into_inner() << 0u32,
        )
    }
}
