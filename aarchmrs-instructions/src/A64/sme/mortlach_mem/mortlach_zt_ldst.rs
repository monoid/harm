/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ldr_zt_br_ {
    #[inline]
    pub fn ldr_zt_br_(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110000100011111100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b00000u32 << 0u32,
        )
    }
}
pub mod str_zt_br_ {
    #[inline]
    pub fn str_zt_br_(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110000100111111100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b00000u32 << 0u32,
        )
    }
}
