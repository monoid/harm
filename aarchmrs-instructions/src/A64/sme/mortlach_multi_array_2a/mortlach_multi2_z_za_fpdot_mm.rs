/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fdot_za_zzw_2x2 {
    #[inline]
    pub fn fdot_za_zzw_2x2(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001101u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b00u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b100u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b000u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod bfdot_za_zzw_2x2 {
    #[inline]
    pub fn bfdot_za_zzw_2x2(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001101u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b00u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b100u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b010u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod fdot_za_z8z8w_2x2 {
    #[inline]
    pub fn fdot_za_z8z8w_2x2(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001101u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b00u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b100u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b100u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod fdot_za32_z8z8w_2x2 {
    #[inline]
    pub fn fdot_za32_z8z8w_2x2(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001101u32 << 21u32
                | u32::from(Zm.into()) << 17u32
                | 0b00u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b100u32 << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b110u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
