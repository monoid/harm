/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqcvtn_z_mz2_ {
    #[inline]
    pub fn sqcvtn_z_mz2_(
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101001100010100u32 << 12u32
                | u32::from(U.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b0u32 << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sqcvtun_z_mz2_ {
    #[inline]
    pub fn sqcvtun_z_mz2_(
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100010100110001010100u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b0u32 << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod uqcvtn_z_mz2_ {
    #[inline]
    pub fn uqcvtn_z_mz2_(
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101001100010100u32 << 12u32
                | u32::from(U.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b0u32 << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
