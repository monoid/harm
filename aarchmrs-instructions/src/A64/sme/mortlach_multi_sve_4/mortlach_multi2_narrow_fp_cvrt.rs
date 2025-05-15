/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod bfcvt_z_mz2_ {
    #[inline]
    pub fn bfcvt_z_mz2_(
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000101100000111000u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | u32::from(N.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod fcvt_z_mz2_ {
    #[inline]
    pub fn fcvt_z_mz2_(
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100100000111000u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | u32::from(N.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod bfcvtn_z_mz2_ {
    #[inline]
    pub fn bfcvtn_z_mz2_(
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000101100000111000u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | u32::from(N.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod fcvtn_z_mz2_ {
    #[inline]
    pub fn fcvtn_z_mz2_(
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100100000111000u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | u32::from(N.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
