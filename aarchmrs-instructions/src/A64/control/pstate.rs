/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MSR_SI_pstate {
    #[inline]
    pub fn MSR_SI_pstate(
        op1: impl Into<::aarchmrs_types::BitValue<3>>,
        CRm: impl Into<::aarchmrs_types::BitValue<4>>,
        op2: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101010100000u32 << 19u32
                | u32::from(op1.into()) << 16u32
                | 0b0100u32 << 12u32
                | u32::from(CRm.into()) << 8u32
                | u32::from(op2.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod CFINV_M_pstate {
    #[inline]
    pub fn CFINV_M_pstate() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000000100000000011111u32 << 0u32)
    }
}
pub mod XAFLAG_M_pstate {
    #[inline]
    pub fn XAFLAG_M_pstate() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000000100000000111111u32 << 0u32)
    }
}
pub mod AXFLAG_M_pstate {
    #[inline]
    pub fn AXFLAG_M_pstate() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000000100000001011111u32 << 0u32)
    }
}
