/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mad_z_p_zzz_ {
    #[inline]
    pub fn mad_z_p_zzz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Za: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b110u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Za.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod msb_z_p_zzz_ {
    #[inline]
    pub fn msb_z_p_zzz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Za: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b111u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Za.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
