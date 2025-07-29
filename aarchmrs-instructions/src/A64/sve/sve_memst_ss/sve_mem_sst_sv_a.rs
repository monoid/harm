/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod st1h_z_p_bz_d_x32_scaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100100101000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st1h_z_p_bz_d_x32_scaled";
    #[inline]
    pub const fn st1h_z_p_bz_d_x32_scaled(
        Zm: ::aarchmrs_types::BitValue<5>,
        xs: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11100100101u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | xs.into_inner() << 14u32
                | 0b0u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod st1w_z_p_bz_d_x32_scaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100101001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st1w_z_p_bz_d_x32_scaled";
    #[inline]
    pub const fn st1w_z_p_bz_d_x32_scaled(
        Zm: ::aarchmrs_types::BitValue<5>,
        xs: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11100101001u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | xs.into_inner() << 14u32
                | 0b0u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod st1d_z_p_bz_d_x32_scaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100101101000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st1d_z_p_bz_d_x32_scaled";
    #[inline]
    pub const fn st1d_z_p_bz_d_x32_scaled(
        Zm: ::aarchmrs_types::BitValue<5>,
        xs: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11100101101u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | xs.into_inner() << 14u32
                | 0b0u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
