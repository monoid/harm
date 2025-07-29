/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SSAT_T1_ASR {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000100000000000000000000100000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SSAT_T1_ASR";
    #[inline]
    pub const fn SSAT_T1_ASR(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        sat_imm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | 0b0u32 << 5u32
                | sat_imm.into_inner() << 0u32,
        )
    }
}
pub mod SSAT_T1_LSL {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000100000000000000000000100000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SSAT_T1_LSL";
    #[inline]
    pub const fn SSAT_T1_LSL(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        sat_imm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | 0b0u32 << 5u32
                | sat_imm.into_inner() << 0u32,
        )
    }
}
pub mod SSAT16_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000100000000000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SSAT16_T1";
    #[inline]
    pub const fn SSAT16_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        sat_imm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0000u32 << 4u32
                | sat_imm.into_inner() << 0u32,
        )
    }
}
pub mod SBFX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000100000000000000000000100000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SBFX_T1";
    #[inline]
    pub const fn SBFX_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        widthm1: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | 0b0u32 << 5u32
                | widthm1.into_inner() << 0u32,
        )
    }
}
pub mod BFI_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000100000000000000000000100000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BFI_T1";
    #[inline]
    pub const fn BFI_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        msb: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110110u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | 0b0u32 << 5u32
                | msb.into_inner() << 0u32,
        )
    }
}
pub mod BFC_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111000000000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011011011110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000100000000000000000000100000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BFC_T1";
    #[inline]
    pub const fn BFC_T1(
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        msb: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110011011011110u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | 0b0u32 << 5u32
                | msb.into_inner() << 0u32,
        )
    }
}
pub mod USAT_T1_ASR {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000100000000000000000000100000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "USAT_T1_ASR";
    #[inline]
    pub const fn USAT_T1_ASR(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        sat_imm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | 0b0u32 << 5u32
                | sat_imm.into_inner() << 0u32,
        )
    }
}
pub mod USAT_T1_LSL {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000100000000000000000000100000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "USAT_T1_LSL";
    #[inline]
    pub const fn USAT_T1_LSL(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        sat_imm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | 0b0u32 << 5u32
                | sat_imm.into_inner() << 0u32,
        )
    }
}
pub mod USAT16_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000100000000000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "USAT16_T1";
    #[inline]
    pub const fn USAT16_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        sat_imm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0000u32 << 4u32
                | sat_imm.into_inner() << 0u32,
        )
    }
}
pub mod UBFX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000100000000000000000000100000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UBFX_T1";
    #[inline]
    pub const fn UBFX_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        widthm1: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | 0b0u32 << 5u32
                | widthm1.into_inner() << 0u32,
        )
    }
}
