/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqcvt_z_mz4_ {
    #[inline]
    pub fn sqcvt_z_mz4_(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(sz.into()) << 23u32
                | 0b0110011111000u32 << 10u32
                | u32::from(Zn.into()) << 7u32
                | u32::from(N.into()) << 6u32
                | u32::from(U.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sqcvtu_z_mz4_ {
    #[inline]
    pub fn sqcvtu_z_mz4_(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(sz.into()) << 23u32
                | 0b1110011111000u32 << 10u32
                | u32::from(Zn.into()) << 7u32
                | u32::from(N.into()) << 6u32
                | 0b0u32 << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sqcvtn_z_mz4_ {
    #[inline]
    pub fn sqcvtn_z_mz4_(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(sz.into()) << 23u32
                | 0b0110011111000u32 << 10u32
                | u32::from(Zn.into()) << 7u32
                | u32::from(N.into()) << 6u32
                | u32::from(U.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sqcvtun_z_mz4_ {
    #[inline]
    pub fn sqcvtun_z_mz4_(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(sz.into()) << 23u32
                | 0b1110011111000u32 << 10u32
                | u32::from(Zn.into()) << 7u32
                | u32::from(N.into()) << 6u32
                | 0b0u32 << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod uqcvt_z_mz4_ {
    #[inline]
    pub fn uqcvt_z_mz4_(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(sz.into()) << 23u32
                | 0b0110011111000u32 << 10u32
                | u32::from(Zn.into()) << 7u32
                | u32::from(N.into()) << 6u32
                | u32::from(U.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod uqcvtn_z_mz4_ {
    #[inline]
    pub fn uqcvtn_z_mz4_(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(sz.into()) << 23u32
                | 0b0110011111000u32 << 10u32
                | u32::from(Zn.into()) << 7u32
                | u32::from(N.into()) << 6u32
                | u32::from(U.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
