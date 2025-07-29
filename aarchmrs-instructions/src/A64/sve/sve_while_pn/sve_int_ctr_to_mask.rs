/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod pext_pn_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000111000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "pext_pn_rr_";
    #[inline]
    pub const fn pext_pn_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        imm2: ::aarchmrs_types::BitValue<2>,
        PNn: ::aarchmrs_types::BitValue<3>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100000011100u32 << 10u32
                | imm2.into_inner() << 8u32
                | PNn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod pext_pp_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000111010000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "pext_pp_rr_";
    #[inline]
    pub const fn pext_pp_rr_(
        size: ::aarchmrs_types::BitValue<2>,
        i1: ::aarchmrs_types::BitValue<1>,
        PNn: ::aarchmrs_types::BitValue<3>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1000000111010u32 << 9u32
                | i1.into_inner() << 8u32
                | PNn.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
