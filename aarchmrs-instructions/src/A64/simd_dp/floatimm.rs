/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FMOV_S_floatimm {
    #[inline]
    pub fn FMOV_S_floatimm(
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | u32::from(imm8.into()) << 13u32
                | 0b10000000u32 << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMOV_D_floatimm {
    #[inline]
    pub fn FMOV_D_floatimm(
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | u32::from(imm8.into()) << 13u32
                | 0b10000000u32 << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMOV_H_floatimm {
    #[inline]
    pub fn FMOV_H_floatimm(
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | u32::from(imm8.into()) << 13u32
                | 0b10000000u32 << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
