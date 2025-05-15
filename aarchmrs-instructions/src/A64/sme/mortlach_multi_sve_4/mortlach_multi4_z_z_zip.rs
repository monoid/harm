/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod zip_mz_z_4 {
    #[inline]
    pub fn zip_mz_z_4(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b110110111000u32 << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b00u32 << 5u32
                | u32::from(Zd.into()) << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod uzp_mz_z_4 {
    #[inline]
    pub fn uzp_mz_z_4(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b110110111000u32 << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b00u32 << 5u32
                | u32::from(Zd.into()) << 2u32
                | 0b10u32 << 0u32,
        )
    }
}
