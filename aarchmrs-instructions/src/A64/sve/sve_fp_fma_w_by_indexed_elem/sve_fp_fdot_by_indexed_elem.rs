/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fdot_z_zzzi_ {
    #[inline]
    pub fn fdot_z_zzzi_(
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100001u32 << 21u32
                | u32::from(i2.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod bfdot_z_zzzi_ {
    #[inline]
    pub fn bfdot_z_zzzi_(
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100011u32 << 21u32
                | u32::from(i2.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b010000u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod fdot_z_zz8z8i_ {
    #[inline]
    pub fn fdot_z_zz8z8i_(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100001u32 << 21u32
                | u32::from(i3h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b0100u32 << 12u32
                | u32::from(i3l.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod fdot_z32_zz8z8i_ {
    #[inline]
    pub fn fdot_z32_zz8z8i_(
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100011u32 << 21u32
                | u32::from(i2.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b010001u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
