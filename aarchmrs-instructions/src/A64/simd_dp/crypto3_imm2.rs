/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SM3TT1A_VVV4_crypto3_imm2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001110010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SM3TT1A_VVV4_crypto3_imm2";
    #[inline]
    pub const fn SM3TT1A_VVV4_crypto3_imm2(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm2: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110010u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b10u32 << 14u32
                | imm2.into_inner() << 12u32
                | 0b00u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SM3TT1B_VVV4_crypto3_imm2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001110010000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SM3TT1B_VVV4_crypto3_imm2";
    #[inline]
    pub const fn SM3TT1B_VVV4_crypto3_imm2(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm2: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110010u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b10u32 << 14u32
                | imm2.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SM3TT2A_VVV4_crypto3_imm2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001110010000001000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SM3TT2A_VVV4_crypto3_imm2";
    #[inline]
    pub const fn SM3TT2A_VVV4_crypto3_imm2(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm2: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110010u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b10u32 << 14u32
                | imm2.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SM3TT2B_VVV_crypto3_imm2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001110010000001000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SM3TT2B_VVV_crypto3_imm2";
    #[inline]
    pub const fn SM3TT2B_VVV_crypto3_imm2(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm2: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110010u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b10u32 << 14u32
                | imm2.into_inner() << 12u32
                | 0b11u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
