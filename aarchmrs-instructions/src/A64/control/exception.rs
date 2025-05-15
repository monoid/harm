/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SVC_EX_exception {
    #[inline]
    pub fn SVC_EX_exception(
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010100000u32 << 21u32 | u32::from(imm16.into()) << 5u32 | 0b00001u32 << 0u32,
        )
    }
}
pub mod HVC_EX_exception {
    #[inline]
    pub fn HVC_EX_exception(
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010100000u32 << 21u32 | u32::from(imm16.into()) << 5u32 | 0b00010u32 << 0u32,
        )
    }
}
pub mod SMC_EX_exception {
    #[inline]
    pub fn SMC_EX_exception(
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010100000u32 << 21u32 | u32::from(imm16.into()) << 5u32 | 0b00011u32 << 0u32,
        )
    }
}
pub mod BRK_EX_exception {
    #[inline]
    pub fn BRK_EX_exception(
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010100001u32 << 21u32 | u32::from(imm16.into()) << 5u32 | 0b00000u32 << 0u32,
        )
    }
}
pub mod HLT_EX_exception {
    #[inline]
    pub fn HLT_EX_exception(
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010100010u32 << 21u32 | u32::from(imm16.into()) << 5u32 | 0b00000u32 << 0u32,
        )
    }
}
pub mod TCANCEL_EX_exception {
    #[inline]
    pub fn TCANCEL_EX_exception(
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010100011u32 << 21u32 | u32::from(imm16.into()) << 5u32 | 0b00000u32 << 0u32,
        )
    }
}
pub mod DCPS1_DC_exception {
    #[inline]
    pub fn DCPS1_DC_exception(
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010100101u32 << 21u32 | u32::from(imm16.into()) << 5u32 | 0b00001u32 << 0u32,
        )
    }
}
pub mod DCPS2_DC_exception {
    #[inline]
    pub fn DCPS2_DC_exception(
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010100101u32 << 21u32 | u32::from(imm16.into()) << 5u32 | 0b00010u32 << 0u32,
        )
    }
}
pub mod DCPS3_DC_exception {
    #[inline]
    pub fn DCPS3_DC_exception(
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010100101u32 << 21u32 | u32::from(imm16.into()) << 5u32 | 0b00011u32 << 0u32,
        )
    }
}
