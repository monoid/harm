/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod scvtf_mz_z_4 {
    #[inline]
    pub fn scvtf_mz_z_4(
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100110010111000u32 << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b0u32 << 6u32
                | u32::from(U.into()) << 5u32
                | u32::from(Zd.into()) << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod ucvtf_mz_z_4 {
    #[inline]
    pub fn ucvtf_mz_z_4(
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100110010111000u32 << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b0u32 << 6u32
                | u32::from(U.into()) << 5u32
                | u32::from(Zd.into()) << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
