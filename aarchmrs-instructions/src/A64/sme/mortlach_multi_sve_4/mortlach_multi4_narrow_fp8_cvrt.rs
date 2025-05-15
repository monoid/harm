/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fcvt_z8_mz4_ {
    #[inline]
    pub fn fcvt_z8_mz4_(
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100110100111000u32 << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b0u32 << 6u32
                | u32::from(N.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod fcvtn_z8_mz4_ {
    #[inline]
    pub fn fcvtn_z8_mz4_(
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100110100111000u32 << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b0u32 << 6u32
                | u32::from(N.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
