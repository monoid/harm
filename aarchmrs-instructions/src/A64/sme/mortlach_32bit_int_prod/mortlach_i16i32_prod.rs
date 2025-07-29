/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smopa_za32_pp_zz_16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000100000000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smopa_za32_pp_zz_16";
    #[inline]
    pub const fn smopa_za32_pp_zz_16(
        Zm: ::aarchmrs_types::BitValue<5>,
        Pm: ::aarchmrs_types::BitValue<3>,
        Pn: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000100u32 << 21u32
                | Zm.into_inner() << 16u32
                | Pm.into_inner() << 13u32
                | Pn.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b010u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umopa_za32_pp_zz_16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001100000000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umopa_za32_pp_zz_16";
    #[inline]
    pub const fn umopa_za32_pp_zz_16(
        Zm: ::aarchmrs_types::BitValue<5>,
        Pm: ::aarchmrs_types::BitValue<3>,
        Pn: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001100u32 << 21u32
                | Zm.into_inner() << 16u32
                | Pm.into_inner() << 13u32
                | Pn.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b010u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod smops_za32_pp_zz_16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000100000000000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smops_za32_pp_zz_16";
    #[inline]
    pub const fn smops_za32_pp_zz_16(
        Zm: ::aarchmrs_types::BitValue<5>,
        Pm: ::aarchmrs_types::BitValue<3>,
        Pn: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000100u32 << 21u32
                | Zm.into_inner() << 16u32
                | Pm.into_inner() << 13u32
                | Pn.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b110u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umops_za32_pp_zz_16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001100000000000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umops_za32_pp_zz_16";
    #[inline]
    pub const fn umops_za32_pp_zz_16(
        Zm: ::aarchmrs_types::BitValue<5>,
        Pm: ::aarchmrs_types::BitValue<3>,
        Pn: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001100u32 << 21u32
                | Zm.into_inner() << 16u32
                | Pm.into_inner() << 13u32
                | Pn.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b110u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
