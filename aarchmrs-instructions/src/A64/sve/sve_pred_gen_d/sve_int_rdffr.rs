/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod rdffr_p_p_f_ {
    #[inline]
    pub fn rdffr_p_p_f_(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b0110001111000u32 << 9u32
                | u32::from(Pg.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod rdffrs_p_p_f_ {
    #[inline]
    pub fn rdffrs_p_p_f_(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b0110001111000u32 << 9u32
                | u32::from(Pg.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
