/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mova_za_mz2_1 {
    #[inline]
    pub fn mova_za_mz2_1(
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000000001000u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b010u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b000u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
