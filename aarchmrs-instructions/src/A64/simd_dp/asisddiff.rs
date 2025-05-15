/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SQDMLAL_asisddiff_only {
    #[inline]
    pub fn SQDMLAL_asisddiff_only(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(o1.into()) << 13u32
                | 0b100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQDMLSL_asisddiff_only {
    #[inline]
    pub fn SQDMLSL_asisddiff_only(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(o1.into()) << 13u32
                | 0b100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQDMULL_asisddiff_only {
    #[inline]
    pub fn SQDMULL_asisddiff_only(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b110100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
