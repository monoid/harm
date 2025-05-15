/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod dupq_z_zi_ {
    #[inline]
    pub fn dupq_z_zi_(
        i1: impl Into<::aarchmrs_types::BitValue<1>>,
        tsz: impl Into<::aarchmrs_types::BitValue<4>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101001u32 << 21u32
                | u32::from(i1.into()) << 20u32
                | u32::from(tsz.into()) << 16u32
                | 0b001001u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
