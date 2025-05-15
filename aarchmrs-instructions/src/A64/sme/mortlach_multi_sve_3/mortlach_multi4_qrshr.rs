/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqrshr_z_mz4_ {
    #[inline]
    pub fn sqrshr_z_mz4_(
        tsize: impl Into<::aarchmrs_types::BitValue<2>>,
        imm5: impl Into<::aarchmrs_types::BitValue<5>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(tsize.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(imm5.into()) << 16u32
                | 0b11011u32 << 11u32
                | u32::from(N.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b0u32 << 6u32
                | u32::from(U.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sqrshru_z_mz4_ {
    #[inline]
    pub fn sqrshru_z_mz4_(
        tsize: impl Into<::aarchmrs_types::BitValue<2>>,
        imm5: impl Into<::aarchmrs_types::BitValue<5>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(tsize.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(imm5.into()) << 16u32
                | 0b11011u32 << 11u32
                | u32::from(N.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b10u32 << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sqrshrn_z_mz4_ {
    #[inline]
    pub fn sqrshrn_z_mz4_(
        tsize: impl Into<::aarchmrs_types::BitValue<2>>,
        imm5: impl Into<::aarchmrs_types::BitValue<5>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(tsize.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(imm5.into()) << 16u32
                | 0b11011u32 << 11u32
                | u32::from(N.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b0u32 << 6u32
                | u32::from(U.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sqrshrun_z_mz4_ {
    #[inline]
    pub fn sqrshrun_z_mz4_(
        tsize: impl Into<::aarchmrs_types::BitValue<2>>,
        imm5: impl Into<::aarchmrs_types::BitValue<5>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(tsize.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(imm5.into()) << 16u32
                | 0b11011u32 << 11u32
                | u32::from(N.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b10u32 << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod uqrshr_z_mz4_ {
    #[inline]
    pub fn uqrshr_z_mz4_(
        tsize: impl Into<::aarchmrs_types::BitValue<2>>,
        imm5: impl Into<::aarchmrs_types::BitValue<5>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(tsize.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(imm5.into()) << 16u32
                | 0b11011u32 << 11u32
                | u32::from(N.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b0u32 << 6u32
                | u32::from(U.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod uqrshrn_z_mz4_ {
    #[inline]
    pub fn uqrshrn_z_mz4_(
        tsize: impl Into<::aarchmrs_types::BitValue<2>>,
        imm5: impl Into<::aarchmrs_types::BitValue<5>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | u32::from(tsize.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(imm5.into()) << 16u32
                | 0b11011u32 << 11u32
                | u32::from(N.into()) << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b0u32 << 6u32
                | u32::from(U.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
