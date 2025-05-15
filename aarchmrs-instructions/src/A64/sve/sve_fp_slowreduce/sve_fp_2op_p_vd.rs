/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fadda_v_p_z_ {
    #[inline]
    pub fn fadda_v_p_z_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Vdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b011000001u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zm.into()) << 5u32
                | u32::from(Vdn.into()) << 0u32,
        )
    }
}
