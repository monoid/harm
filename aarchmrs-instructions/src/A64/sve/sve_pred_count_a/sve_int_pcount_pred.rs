/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cntp_r_p_p_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111100001000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cntp_r_p_p_";
    #[inline]
    pub const fn cntp_r_p_p_(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10000010u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod firstp_r_p_p_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111100001000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000011000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "firstp_r_p_p_";
    #[inline]
    pub const fn firstp_r_p_p_(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10000110u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod lastp_r_p_p_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111100001000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000101000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "lastp_r_p_p_";
    #[inline]
    pub const fn lastp_r_p_p_(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10001010u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
