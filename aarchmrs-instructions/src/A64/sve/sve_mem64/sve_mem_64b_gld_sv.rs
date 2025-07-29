/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1sh_z_p_bz_d_x32_scaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000100101000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1sh_z_p_bz_d_x32_scaled";
    #[inline]
    pub const fn ld1sh_z_p_bz_d_x32_scaled(
        xs: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110001001u32 << 23u32
                | xs.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b000u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1sw_z_p_bz_d_x32_scaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000101001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1sw_z_p_bz_d_x32_scaled";
    #[inline]
    pub const fn ld1sw_z_p_bz_d_x32_scaled(
        xs: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110001010u32 << 23u32
                | xs.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b000u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1d_z_p_bz_d_x32_scaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000101101000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1d_z_p_bz_d_x32_scaled";
    #[inline]
    pub const fn ld1d_z_p_bz_d_x32_scaled(
        xs: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110001011u32 << 23u32
                | xs.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b010u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1h_z_p_bz_d_x32_scaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000100101000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1h_z_p_bz_d_x32_scaled";
    #[inline]
    pub const fn ld1h_z_p_bz_d_x32_scaled(
        xs: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110001001u32 << 23u32
                | xs.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b010u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1w_z_p_bz_d_x32_scaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000101001000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1w_z_p_bz_d_x32_scaled";
    #[inline]
    pub const fn ld1w_z_p_bz_d_x32_scaled(
        xs: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110001010u32 << 23u32
                | xs.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b010u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldff1sh_z_p_bz_d_x32_scaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000100101000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldff1sh_z_p_bz_d_x32_scaled";
    #[inline]
    pub const fn ldff1sh_z_p_bz_d_x32_scaled(
        xs: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110001001u32 << 23u32
                | xs.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b001u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldff1sw_z_p_bz_d_x32_scaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000101001000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldff1sw_z_p_bz_d_x32_scaled";
    #[inline]
    pub const fn ldff1sw_z_p_bz_d_x32_scaled(
        xs: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110001010u32 << 23u32
                | xs.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b001u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldff1d_z_p_bz_d_x32_scaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000101101000000110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldff1d_z_p_bz_d_x32_scaled";
    #[inline]
    pub const fn ldff1d_z_p_bz_d_x32_scaled(
        xs: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110001011u32 << 23u32
                | xs.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b011u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldff1h_z_p_bz_d_x32_scaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000100101000000110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldff1h_z_p_bz_d_x32_scaled";
    #[inline]
    pub const fn ldff1h_z_p_bz_d_x32_scaled(
        xs: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110001001u32 << 23u32
                | xs.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b011u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldff1w_z_p_bz_d_x32_scaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000101001000000110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldff1w_z_p_bz_d_x32_scaled";
    #[inline]
    pub const fn ldff1w_z_p_bz_d_x32_scaled(
        xs: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110001010u32 << 23u32
                | xs.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b011u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
