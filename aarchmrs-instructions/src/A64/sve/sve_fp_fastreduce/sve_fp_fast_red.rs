/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod faddv_v_p_z_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "faddv_v_p_z_";
    #[inline]
    pub const fn faddv_v_p_z_(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Vd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b000000001u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Vd.into_inner() << 0u32,
        )
    }
}
pub mod fmaxnmv_v_p_z_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000001000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmaxnmv_v_p_z_";
    #[inline]
    pub const fn fmaxnmv_v_p_z_(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Vd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b000100001u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Vd.into_inner() << 0u32,
        )
    }
}
pub mod fminnmv_v_p_z_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000001010010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fminnmv_v_p_z_";
    #[inline]
    pub const fn fminnmv_v_p_z_(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Vd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b000101001u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Vd.into_inner() << 0u32,
        )
    }
}
pub mod fmaxv_v_p_z_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000001100010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmaxv_v_p_z_";
    #[inline]
    pub const fn fmaxv_v_p_z_(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Vd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b000110001u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Vd.into_inner() << 0u32,
        )
    }
}
pub mod fminv_v_p_z_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000001110010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fminv_v_p_z_";
    #[inline]
    pub const fn fminv_v_p_z_(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Vd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b000111001u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Vd.into_inner() << 0u32,
        )
    }
}
