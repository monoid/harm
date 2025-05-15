/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod brkpa_p_p_pp_ {
    #[inline]
    pub fn brkpa_p_p_pp_(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        B: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b00u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b11u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | u32::from(B.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod brkpas_p_p_pp_ {
    #[inline]
    pub fn brkpas_p_p_pp_(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        B: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b00u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b11u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | u32::from(B.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod brkpb_p_p_pp_ {
    #[inline]
    pub fn brkpb_p_p_pp_(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        B: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b00u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b11u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | u32::from(B.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod brkpbs_p_p_pp_ {
    #[inline]
    pub fn brkpbs_p_p_pp_(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        B: impl Into<::aarchmrs_types::BitValue<1>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b00u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b11u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | u32::from(B.into()) << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
