/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MOVN_32_movewide {
    #[inline]
    pub fn MOVN_32_movewide(
        hw: impl Into<::aarchmrs_types::BitValue<1>>,
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001001010u32 << 22u32
                | u32::from(hw.into()) << 21u32
                | u32::from(imm16.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod MOVZ_32_movewide {
    #[inline]
    pub fn MOVZ_32_movewide(
        hw: impl Into<::aarchmrs_types::BitValue<1>>,
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101001010u32 << 22u32
                | u32::from(hw.into()) << 21u32
                | u32::from(imm16.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod MOVK_32_movewide {
    #[inline]
    pub fn MOVK_32_movewide(
        hw: impl Into<::aarchmrs_types::BitValue<1>>,
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0111001010u32 << 22u32
                | u32::from(hw.into()) << 21u32
                | u32::from(imm16.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod MOVN_64_movewide {
    #[inline]
    pub fn MOVN_64_movewide(
        hw: impl Into<::aarchmrs_types::BitValue<2>>,
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b100100101u32 << 23u32
                | u32::from(hw.into()) << 21u32
                | u32::from(imm16.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod MOVZ_64_movewide {
    #[inline]
    pub fn MOVZ_64_movewide(
        hw: impl Into<::aarchmrs_types::BitValue<2>>,
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110100101u32 << 23u32
                | u32::from(hw.into()) << 21u32
                | u32::from(imm16.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod MOVK_64_movewide {
    #[inline]
    pub fn MOVK_64_movewide(
        hw: impl Into<::aarchmrs_types::BitValue<2>>,
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100101u32 << 23u32
                | u32::from(hw.into()) << 21u32
                | u32::from(imm16.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
