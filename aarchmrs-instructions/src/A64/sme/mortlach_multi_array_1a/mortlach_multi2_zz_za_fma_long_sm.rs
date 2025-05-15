/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod bfmlal_za_zzv_2x1 {
    #[inline]
    pub fn bfmlal_za_zzv_2x1(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b010u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(S.into()) << 3u32
                | 0b0u32 << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
pub mod fmlal_za_zzv_2x1 {
    #[inline]
    pub fn fmlal_za_zzv_2x1(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b010u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(S.into()) << 3u32
                | 0b0u32 << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
pub mod fmlal_za_z8z8v_2x1 {
    #[inline]
    pub fn fmlal_za_z8z8v_2x1(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b010u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b001u32 << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
pub mod bfmlsl_za_zzv_2x1 {
    #[inline]
    pub fn bfmlsl_za_zzv_2x1(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b010u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(S.into()) << 3u32
                | 0b0u32 << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
pub mod fmlsl_za_zzv_2x1 {
    #[inline]
    pub fn fmlsl_za_zzv_2x1(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b010u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(S.into()) << 3u32
                | 0b0u32 << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
