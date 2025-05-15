/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod frintn_mz_z_4 {
    #[inline]
    pub fn frintn_mz_z_4(
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000110111000111000u32 << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b00u32 << 5u32
                | u32::from(Zd.into()) << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod frintp_mz_z_4 {
    #[inline]
    pub fn frintp_mz_z_4(
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000110111001111000u32 << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b00u32 << 5u32
                | u32::from(Zd.into()) << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod frintm_mz_z_4 {
    #[inline]
    pub fn frintm_mz_z_4(
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000110111010111000u32 << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b00u32 << 5u32
                | u32::from(Zd.into()) << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod frinta_mz_z_4 {
    #[inline]
    pub fn frinta_mz_z_4(
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zd: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000110111100111000u32 << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b00u32 << 5u32
                | u32::from(Zd.into()) << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
