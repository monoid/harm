/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmopa_za32_pp_zz_16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001101000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmopa_za32_pp_zz_16";
    #[inline]
    pub const fn fmopa_za32_pp_zz_16(
        Zm: ::aarchmrs_types::BitValue<5>,
        Pm: ::aarchmrs_types::BitValue<3>,
        Pn: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001101u32 << 21u32
                | Zm.into_inner() << 16u32
                | Pm.into_inner() << 13u32
                | Pn.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod fmops_za32_pp_zz_16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001101000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmops_za32_pp_zz_16";
    #[inline]
    pub const fn fmops_za32_pp_zz_16(
        Zm: ::aarchmrs_types::BitValue<5>,
        Pm: ::aarchmrs_types::BitValue<3>,
        Pn: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001101u32 << 21u32
                | Zm.into_inner() << 16u32
                | Pm.into_inner() << 13u32
                | Pn.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
