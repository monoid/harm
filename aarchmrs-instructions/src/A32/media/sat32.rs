/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SSAT_A1_ASR {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111000000000000001110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110101000000000000001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SSAT_A1_ASR";
    #[inline]
    pub const fn SSAT_A1_ASR(
        cond: ::aarchmrs_types::BitValue<4>,
        sat_imm: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0110101u32 << 21u32
                | sat_imm.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | 0b101u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SSAT_A1_LSL {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111000000000000001110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110101000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SSAT_A1_LSL";
    #[inline]
    pub const fn SSAT_A1_LSL(
        cond: ::aarchmrs_types::BitValue<4>,
        sat_imm: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0110101u32 << 21u32
                | sat_imm.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | 0b001u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod USAT_A1_ASR {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111000000000000001110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110111000000000000001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "USAT_A1_ASR";
    #[inline]
    pub const fn USAT_A1_ASR(
        cond: ::aarchmrs_types::BitValue<4>,
        sat_imm: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0110111u32 << 21u32
                | sat_imm.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | 0b101u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod USAT_A1_LSL {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111000000000000001110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000110111000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "USAT_A1_LSL";
    #[inline]
    pub const fn USAT_A1_LSL(
        cond: ::aarchmrs_types::BitValue<4>,
        sat_imm: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm5: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b0110111u32 << 21u32
                | sat_imm.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm5.into_inner() << 7u32
                | 0b001u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
