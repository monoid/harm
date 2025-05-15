/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqrshrn_z_mz2_ {
    #[inline]
    pub fn sqrshrn_z_mz2_(
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001011011u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b001u32 << 13u32
                | u32::from(U.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b0u32 << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sqrshrun_z_mz2_ {
    #[inline]
    pub fn sqrshrun_z_mz2_(
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001011011u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b000010u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b0u32 << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod uqrshrn_z_mz2_ {
    #[inline]
    pub fn uqrshrn_z_mz2_(
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001011011u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b001u32 << 13u32
                | u32::from(U.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b0u32 << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
