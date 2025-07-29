/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smopa_za_pp_zz_32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smopa_za_pp_zz_32";
    #[inline]
    pub const fn smopa_za_pp_zz_32(
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
                | 0b000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod sumopa_za_pp_zz_32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000101000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumopa_za_pp_zz_32";
    #[inline]
    pub const fn sumopa_za_pp_zz_32(
        Zm: ::aarchmrs_types::BitValue<5>,
        Pm: ::aarchmrs_types::BitValue<3>,
        Pn: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000101u32 << 21u32
                | Zm.into_inner() << 16u32
                | Pm.into_inner() << 13u32
                | Pn.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod usmopa_za_pp_zz_32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmopa_za_pp_zz_32";
    #[inline]
    pub const fn usmopa_za_pp_zz_32(
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
                | 0b000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umopa_za_pp_zz_32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001101000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umopa_za_pp_zz_32";
    #[inline]
    pub const fn umopa_za_pp_zz_32(
        Zm: ::aarchmrs_types::BitValue<5>,
        Pm: ::aarchmrs_types::BitValue<3>,
        Pn: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001101u32 << 21u32
                | Zm.into_inner() << 16u32
                | Pm.into_inner() << 13u32
                | Pn.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod smops_za_pp_zz_32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000100000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smops_za_pp_zz_32";
    #[inline]
    pub const fn smops_za_pp_zz_32(
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
                | 0b100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod sumops_za_pp_zz_32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000101000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumops_za_pp_zz_32";
    #[inline]
    pub const fn sumops_za_pp_zz_32(
        Zm: ::aarchmrs_types::BitValue<5>,
        Pm: ::aarchmrs_types::BitValue<3>,
        Pn: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000101u32 << 21u32
                | Zm.into_inner() << 16u32
                | Pm.into_inner() << 13u32
                | Pn.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod usmops_za_pp_zz_32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001100000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmops_za_pp_zz_32";
    #[inline]
    pub const fn usmops_za_pp_zz_32(
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
                | 0b100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umops_za_pp_zz_32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001101000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umops_za_pp_zz_32";
    #[inline]
    pub const fn umops_za_pp_zz_32(
        Zm: ::aarchmrs_types::BitValue<5>,
        Pm: ::aarchmrs_types::BitValue<3>,
        Pn: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001101u32 << 21u32
                | Zm.into_inner() << 16u32
                | Pm.into_inner() << 13u32
                | Pn.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
