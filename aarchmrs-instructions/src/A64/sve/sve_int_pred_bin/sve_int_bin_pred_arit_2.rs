/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mul_z_p_zz_ {
    #[inline]
    pub fn mul_z_p_zz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b010000000u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zm.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod smulh_z_p_zz_ {
    #[inline]
    pub fn smulh_z_p_zz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b01001u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zm.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod umulh_z_p_zz_ {
    #[inline]
    pub fn umulh_z_p_zz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b01001u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zm.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
