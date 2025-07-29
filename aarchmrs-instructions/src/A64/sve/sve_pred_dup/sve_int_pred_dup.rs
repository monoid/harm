/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod psel_p_ppi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001100001000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "psel_p_ppi_";
    #[inline]
    pub const fn psel_p_ppi_(
        i1: ::aarchmrs_types::BitValue<1>,
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<3>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | i1.into_inner() << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 18u32
                | Rv.into_inner() << 16u32
                | 0b01u32 << 14u32
                | Pn.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pm.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
