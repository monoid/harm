/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmla_za_zzi_d2xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001100000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmla_za_zzi_d2xi";
    #[inline]
    pub const fn fmla_za_zzi_d2xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i1: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<4>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b00u32 << 11u32
                | i1.into_inner() << 10u32
                | Zn.into_inner() << 6u32
                | 0b000u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod sdot_za_zzi_d2xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001100000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sdot_za_zzi_d2xi";
    #[inline]
    pub const fn sdot_za_zzi_d2xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i1: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<4>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b00u32 << 11u32
                | i1.into_inner() << 10u32
                | Zn.into_inner() << 6u32
                | 0b001u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod fmls_za_zzi_d2xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001100000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmls_za_zzi_d2xi";
    #[inline]
    pub const fn fmls_za_zzi_d2xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i1: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<4>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b00u32 << 11u32
                | i1.into_inner() << 10u32
                | Zn.into_inner() << 6u32
                | 0b010u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod udot_za_zzi_d2xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001100000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100000000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "udot_za_zzi_d2xi";
    #[inline]
    pub const fn udot_za_zzi_d2xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i1: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<4>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b00u32 << 11u32
                | i1.into_inner() << 10u32
                | Zn.into_inner() << 6u32
                | 0b011u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
