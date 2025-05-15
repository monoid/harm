/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod st1q_z_p_ar_d_64_unscaled {
    #[inline]
    pub fn st1q_z_p_ar_d_64_unscaled(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11100100001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
