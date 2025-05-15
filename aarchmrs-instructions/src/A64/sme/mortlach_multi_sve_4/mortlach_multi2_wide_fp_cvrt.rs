/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fcvt_mz2_z_ {
    #[inline]
    pub fn fcvt_mz2_z_(
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<4>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000110100000111000u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 1u32
                | u32::from(L.into()) << 0u32,
        )
    }
}
pub mod fcvtl_mz2_z_ {
    #[inline]
    pub fn fcvtl_mz2_z_(
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<4>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000110100000111000u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 1u32
                | u32::from(L.into()) << 0u32,
        )
    }
}
