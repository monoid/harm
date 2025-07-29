/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod st1b_za_p_rrr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100000001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st1b_za_p_rrr_";
    #[inline]
    pub const fn st1b_za_p_rrr_(
        Rm: ::aarchmrs_types::BitValue<5>,
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        off4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11100000001u32 << 21u32
                | Rm.into_inner() << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | off4.into_inner() << 0u32,
        )
    }
}
pub mod st1h_za_p_rrr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100000011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st1h_za_p_rrr_";
    #[inline]
    pub const fn st1h_za_p_rrr_(
        Rm: ::aarchmrs_types::BitValue<5>,
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        ZAt: ::aarchmrs_types::BitValue<1>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11100000011u32 << 21u32
                | Rm.into_inner() << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | ZAt.into_inner() << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod st1w_za_p_rrr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100000101000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st1w_za_p_rrr_";
    #[inline]
    pub const fn st1w_za_p_rrr_(
        Rm: ::aarchmrs_types::BitValue<5>,
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        ZAt: ::aarchmrs_types::BitValue<2>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11100000101u32 << 21u32
                | Rm.into_inner() << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | ZAt.into_inner() << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod st1d_za_p_rrr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100000111000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st1d_za_p_rrr_";
    #[inline]
    pub const fn st1d_za_p_rrr_(
        Rm: ::aarchmrs_types::BitValue<5>,
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        ZAt: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11100000111u32 << 21u32
                | Rm.into_inner() << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | ZAt.into_inner() << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
