/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ldr_za_ri_ {
    #[inline]
    pub fn ldr_za_ri_(
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        off4: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11100001000000000u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(off4.into()) << 0u32,
        )
    }
}
pub mod str_za_ri_ {
    #[inline]
    pub fn str_za_ri_(
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        off4: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11100001001000000u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(off4.into()) << 0u32,
        )
    }
}
