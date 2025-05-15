/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ptest__p_p_ {
    #[inline]
    pub fn ptest__p_p_(
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010101000011u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b00000u32 << 0u32,
        )
    }
}
