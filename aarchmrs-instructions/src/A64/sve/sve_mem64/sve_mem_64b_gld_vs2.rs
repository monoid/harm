/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1sb_z_p_bz_d_64_unscaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1sb_z_p_bz_d_64_unscaled";
    #[inline]
    pub const fn ld1sb_z_p_bz_d_64_unscaled(
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        ff: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000100010u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | U.into_inner() << 14u32
                | ff.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1sh_z_p_bz_d_64_unscaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000100110000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1sh_z_p_bz_d_64_unscaled";
    #[inline]
    pub const fn ld1sh_z_p_bz_d_64_unscaled(
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        ff: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000100110u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | U.into_inner() << 14u32
                | ff.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1sw_z_p_bz_d_64_unscaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000101010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1sw_z_p_bz_d_64_unscaled";
    #[inline]
    pub const fn ld1sw_z_p_bz_d_64_unscaled(
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        ff: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000101010u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | U.into_inner() << 14u32
                | ff.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1d_z_p_bz_d_64_unscaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000101110000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1d_z_p_bz_d_64_unscaled";
    #[inline]
    pub const fn ld1d_z_p_bz_d_64_unscaled(
        Zm: ::aarchmrs_types::BitValue<5>,
        ff: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000101110u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b11u32 << 14u32
                | ff.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1b_z_p_bz_d_64_unscaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1b_z_p_bz_d_64_unscaled";
    #[inline]
    pub const fn ld1b_z_p_bz_d_64_unscaled(
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        ff: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000100010u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | U.into_inner() << 14u32
                | ff.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1h_z_p_bz_d_64_unscaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000100110000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1h_z_p_bz_d_64_unscaled";
    #[inline]
    pub const fn ld1h_z_p_bz_d_64_unscaled(
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        ff: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000100110u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | U.into_inner() << 14u32
                | ff.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld1w_z_p_bz_d_64_unscaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000101010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1w_z_p_bz_d_64_unscaled";
    #[inline]
    pub const fn ld1w_z_p_bz_d_64_unscaled(
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        ff: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000101010u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | U.into_inner() << 14u32
                | ff.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldff1sb_z_p_bz_d_64_unscaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldff1sb_z_p_bz_d_64_unscaled";
    #[inline]
    pub const fn ldff1sb_z_p_bz_d_64_unscaled(
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        ff: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000100010u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | U.into_inner() << 14u32
                | ff.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldff1sh_z_p_bz_d_64_unscaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000100110000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldff1sh_z_p_bz_d_64_unscaled";
    #[inline]
    pub const fn ldff1sh_z_p_bz_d_64_unscaled(
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        ff: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000100110u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | U.into_inner() << 14u32
                | ff.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldff1sw_z_p_bz_d_64_unscaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000101010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldff1sw_z_p_bz_d_64_unscaled";
    #[inline]
    pub const fn ldff1sw_z_p_bz_d_64_unscaled(
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        ff: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000101010u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | U.into_inner() << 14u32
                | ff.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldff1d_z_p_bz_d_64_unscaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000101110000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldff1d_z_p_bz_d_64_unscaled";
    #[inline]
    pub const fn ldff1d_z_p_bz_d_64_unscaled(
        Zm: ::aarchmrs_types::BitValue<5>,
        ff: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000101110u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b11u32 << 14u32
                | ff.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldff1b_z_p_bz_d_64_unscaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldff1b_z_p_bz_d_64_unscaled";
    #[inline]
    pub const fn ldff1b_z_p_bz_d_64_unscaled(
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        ff: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000100010u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | U.into_inner() << 14u32
                | ff.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldff1h_z_p_bz_d_64_unscaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000100110000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldff1h_z_p_bz_d_64_unscaled";
    #[inline]
    pub const fn ldff1h_z_p_bz_d_64_unscaled(
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        ff: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000100110u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | U.into_inner() << 14u32
                | ff.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldff1w_z_p_bz_d_64_unscaled {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000101010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldff1w_z_p_bz_d_64_unscaled";
    #[inline]
    pub const fn ldff1w_z_p_bz_d_64_unscaled(
        Zm: ::aarchmrs_types::BitValue<5>,
        U: ::aarchmrs_types::BitValue<1>,
        ff: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000101010u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | U.into_inner() << 14u32
                | ff.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
