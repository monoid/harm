/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fadd_z_p_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110001111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000110001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fadd_z_p_zs_";
    #[inline]
    pub const fn fadd_z_p_zs_(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        i1: ::aarchmrs_types::BitValue<1>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b011000100u32 << 13u32
                | Pg.into_inner() << 10u32
                | 0b0000u32 << 6u32
                | i1.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod fsub_z_p_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110001111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000110011000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fsub_z_p_zs_";
    #[inline]
    pub const fn fsub_z_p_zs_(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        i1: ::aarchmrs_types::BitValue<1>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b011001100u32 << 13u32
                | Pg.into_inner() << 10u32
                | 0b0000u32 << 6u32
                | i1.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod fmul_z_p_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110001111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000110101000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmul_z_p_zs_";
    #[inline]
    pub const fn fmul_z_p_zs_(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        i1: ::aarchmrs_types::BitValue<1>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b011010100u32 << 13u32
                | Pg.into_inner() << 10u32
                | 0b0000u32 << 6u32
                | i1.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod fsubr_z_p_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110001111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000110111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fsubr_z_p_zs_";
    #[inline]
    pub const fn fsubr_z_p_zs_(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        i1: ::aarchmrs_types::BitValue<1>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b011011100u32 << 13u32
                | Pg.into_inner() << 10u32
                | 0b0000u32 << 6u32
                | i1.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod fmaxnm_z_p_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110001111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000111001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmaxnm_z_p_zs_";
    #[inline]
    pub const fn fmaxnm_z_p_zs_(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        i1: ::aarchmrs_types::BitValue<1>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b011100100u32 << 13u32
                | Pg.into_inner() << 10u32
                | 0b0000u32 << 6u32
                | i1.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod fminnm_z_p_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110001111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000111011000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fminnm_z_p_zs_";
    #[inline]
    pub const fn fminnm_z_p_zs_(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        i1: ::aarchmrs_types::BitValue<1>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b011101100u32 << 13u32
                | Pg.into_inner() << 10u32
                | 0b0000u32 << 6u32
                | i1.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod fmax_z_p_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110001111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000111101000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmax_z_p_zs_";
    #[inline]
    pub const fn fmax_z_p_zs_(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        i1: ::aarchmrs_types::BitValue<1>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b011110100u32 << 13u32
                | Pg.into_inner() << 10u32
                | 0b0000u32 << 6u32
                | i1.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod fmin_z_p_zs_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110001111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmin_z_p_zs_";
    #[inline]
    pub const fn fmin_z_p_zs_(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        i1: ::aarchmrs_types::BitValue<1>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b011111100u32 << 13u32
                | Pg.into_inner() << 10u32
                | 0b0000u32 << 6u32
                | i1.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
