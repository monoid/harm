/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod EOR3_VVV16_crypto4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001110000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "EOR3_VVV16_crypto4";
    #[inline]
    pub const fn EOR3_VVV16_crypto4(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod BCAX_VVV16_crypto4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001110001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BCAX_VVV16_crypto4";
    #[inline]
    pub const fn BCAX_VVV16_crypto4(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110001u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SM3SS1_VVV4_crypto4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001110010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SM3SS1_VVV4_crypto4";
    #[inline]
    pub const fn SM3SS1_VVV4_crypto4(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110010u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
