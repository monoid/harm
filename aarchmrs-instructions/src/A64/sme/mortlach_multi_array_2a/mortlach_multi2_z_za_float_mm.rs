/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fadd_za_zw_2x2 {
    #[inline]
    pub fn fadd_za_zw_2x2(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b1000000u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b111u32 << 10u32
                | u32::from(Zm.into()) << 6u32
                | 0b00u32 << 4u32
                | u32::from(S.into()) << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod fsub_za_zw_2x2 {
    #[inline]
    pub fn fsub_za_zw_2x2(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b1000000u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b111u32 << 10u32
                | u32::from(Zm.into()) << 6u32
                | 0b00u32 << 4u32
                | u32::from(S.into()) << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
