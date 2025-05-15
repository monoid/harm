/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cpy_z_o_i_ {
    #[inline]
    pub fn cpy_z_o_i_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        sh: impl Into<::aarchmrs_types::BitValue<1>>,
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b01u32 << 20u32
                | u32::from(Pg.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(sh.into()) << 13u32
                | u32::from(imm8.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod cpy_z_p_i_ {
    #[inline]
    pub fn cpy_z_p_i_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        sh: impl Into<::aarchmrs_types::BitValue<1>>,
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b01u32 << 20u32
                | u32::from(Pg.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(sh.into()) << 13u32
                | u32::from(imm8.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
