/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod movaz_mz_za2_1 {
    #[inline]
    pub fn movaz_mz_za2_1(
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
        Zd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000000001100u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b01010u32 << 8u32
                | u32::from(off3.into()) << 5u32
                | u32::from(Zd.into()) << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
