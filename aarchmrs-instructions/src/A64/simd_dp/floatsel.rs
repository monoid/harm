/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FCSEL_S_floatsel {
    #[inline]
    pub fn FCSEL_S_floatsel(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b11u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCSEL_D_floatsel {
    #[inline]
    pub fn FCSEL_D_floatsel(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b11u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCSEL_H_floatsel {
    #[inline]
    pub fn FCSEL_H_floatsel(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b11u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
