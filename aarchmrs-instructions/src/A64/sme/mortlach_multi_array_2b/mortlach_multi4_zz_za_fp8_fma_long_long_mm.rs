/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlall_za32_z8z8w_4x4 {
    #[inline]
    pub fn fmlall_za32_z8z8w_4x4(
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001101u32 << 21u32
                | u32::from(Zm.into()) << 18u32
                | 0b010u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b000u32 << 10u32
                | u32::from(Zn.into()) << 7u32
                | 0b010000u32 << 1u32
                | u32::from(o1.into()) << 0u32,
        )
    }
}
