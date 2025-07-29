/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmad_z_p_zzz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmad_z_p_zzz_";
    #[inline]
    pub const fn fmad_z_p_zzz_(
        size: ::aarchmrs_types::BitValue<2>,
        Za: ::aarchmrs_types::BitValue<5>,
        N: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Za.into_inner() << 16u32
                | 0b1u32 << 15u32
                | N.into_inner() << 14u32
                | op.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zm.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod fmsb_z_p_zzz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmsb_z_p_zzz_";
    #[inline]
    pub const fn fmsb_z_p_zzz_(
        size: ::aarchmrs_types::BitValue<2>,
        Za: ::aarchmrs_types::BitValue<5>,
        N: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Za.into_inner() << 16u32
                | 0b1u32 << 15u32
                | N.into_inner() << 14u32
                | op.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zm.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod fnmad_z_p_zzz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fnmad_z_p_zzz_";
    #[inline]
    pub const fn fnmad_z_p_zzz_(
        size: ::aarchmrs_types::BitValue<2>,
        Za: ::aarchmrs_types::BitValue<5>,
        N: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Za.into_inner() << 16u32
                | 0b1u32 << 15u32
                | N.into_inner() << 14u32
                | op.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zm.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod fnmsb_z_p_zzz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fnmsb_z_p_zzz_";
    #[inline]
    pub const fn fnmsb_z_p_zzz_(
        size: ::aarchmrs_types::BitValue<2>,
        Za: ::aarchmrs_types::BitValue<5>,
        N: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Za.into_inner() << 16u32
                | 0b1u32 << 15u32
                | N.into_inner() << 14u32
                | op.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zm.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
