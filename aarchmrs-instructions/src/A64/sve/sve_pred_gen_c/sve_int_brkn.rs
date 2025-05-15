/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod brkn_p_p_pp_ {
    #[inline]
    pub fn brkn_p_p_pp_(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pdm: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b01100001u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pdm.into()) << 0u32,
        )
    }
}
pub mod brkns_p_p_pp_ {
    #[inline]
    pub fn brkns_p_p_pp_(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pdm: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b01100001u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pdm.into()) << 0u32,
        )
    }
}
