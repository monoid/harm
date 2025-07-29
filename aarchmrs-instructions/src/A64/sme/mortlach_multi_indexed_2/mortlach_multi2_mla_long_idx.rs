/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlal_za_zzi_2xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlal_za_zzi_2xi";
    #[inline]
    pub const fn smlal_za_zzi_2xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i3h: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        i3l: ::aarchmrs_types::BitValue<1>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b1u32 << 12u32
                | i3h.into_inner() << 10u32
                | Zn.into_inner() << 6u32
                | 0b000u32 << 3u32
                | i3l.into_inner() << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod smlsl_za_zzi_2xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100000001000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlsl_za_zzi_2xi";
    #[inline]
    pub const fn smlsl_za_zzi_2xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i3h: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        i3l: ::aarchmrs_types::BitValue<1>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b1u32 << 12u32
                | i3h.into_inner() << 10u32
                | Zn.into_inner() << 6u32
                | 0b001u32 << 3u32
                | i3l.into_inner() << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod umlal_za_zzi_2xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100000001000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlal_za_zzi_2xi";
    #[inline]
    pub const fn umlal_za_zzi_2xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i3h: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        i3l: ::aarchmrs_types::BitValue<1>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b1u32 << 12u32
                | i3h.into_inner() << 10u32
                | Zn.into_inner() << 6u32
                | 0b010u32 << 3u32
                | i3l.into_inner() << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod umlsl_za_zzi_2xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100000001000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlsl_za_zzi_2xi";
    #[inline]
    pub const fn umlsl_za_zzi_2xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i3h: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        i3l: ::aarchmrs_types::BitValue<1>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b1u32 << 12u32
                | i3h.into_inner() << 10u32
                | Zn.into_inner() << 6u32
                | 0b011u32 << 3u32
                | i3l.into_inner() << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
