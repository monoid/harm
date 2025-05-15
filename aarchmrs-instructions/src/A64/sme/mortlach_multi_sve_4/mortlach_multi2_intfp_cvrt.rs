/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod scvtf_mz_z_2 {
    #[inline]
    pub fn scvtf_mz_z_2(
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100100010111000u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | u32::from(U.into()) << 5u32
                | u32::from(Zd.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod ucvtf_mz_z_2 {
    #[inline]
    pub fn ucvtf_mz_z_2(
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100100010111000u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | u32::from(U.into()) << 5u32
                | u32::from(Zd.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
