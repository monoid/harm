/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod brkn_p_p_pp_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000110000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "brkn_p_p_pp_";
    #[inline]
    pub const fn brkn_p_p_pp_(
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pdm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010001100001u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pdm.into_inner() << 0u32,
        )
    }
}
pub mod brkns_p_p_pp_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101010110000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "brkns_p_p_pp_";
    #[inline]
    pub const fn brkns_p_p_pp_(
        Pg: ::aarchmrs_types::BitValue<4>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pdm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010101100001u32 << 14u32
                | Pg.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pdm.into_inner() << 0u32,
        )
    }
}
