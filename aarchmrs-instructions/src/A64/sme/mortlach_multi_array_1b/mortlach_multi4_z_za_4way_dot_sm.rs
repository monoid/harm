/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sdot_za_zzv_4x1 {
    #[inline]
    pub fn sdot_za_zzv_4x1(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b101u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(U.into()) << 4u32
                | 0b0u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod udot_za_zzv_4x1 {
    #[inline]
    pub fn udot_za_zzv_4x1(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b101u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(U.into()) << 4u32
                | 0b0u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
