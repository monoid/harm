/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlal_za_zzi_4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100001001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlal_za_zzi_4xi";
    #[inline]
    pub const fn smlal_za_zzi_4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i3h: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        U: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        i3l: ::aarchmrs_types::BitValue<1>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b1u32 << 12u32
                | i3h.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b00u32 << 5u32
                | U.into_inner() << 4u32
                | S.into_inner() << 3u32
                | i3l.into_inner() << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod smlsl_za_zzi_4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100001001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlsl_za_zzi_4xi";
    #[inline]
    pub const fn smlsl_za_zzi_4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i3h: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        U: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        i3l: ::aarchmrs_types::BitValue<1>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b1u32 << 12u32
                | i3h.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b00u32 << 5u32
                | U.into_inner() << 4u32
                | S.into_inner() << 3u32
                | i3l.into_inner() << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod umlal_za_zzi_4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100001001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlal_za_zzi_4xi";
    #[inline]
    pub const fn umlal_za_zzi_4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i3h: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        U: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        i3l: ::aarchmrs_types::BitValue<1>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b1u32 << 12u32
                | i3h.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b00u32 << 5u32
                | U.into_inner() << 4u32
                | S.into_inner() << 3u32
                | i3l.into_inner() << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod umlsl_za_zzi_4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100001001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlsl_za_zzi_4xi";
    #[inline]
    pub const fn umlsl_za_zzi_4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i3h: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        U: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        i3l: ::aarchmrs_types::BitValue<1>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b1u32 << 12u32
                | i3h.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b00u32 << 5u32
                | U.into_inner() << 4u32
                | S.into_inner() << 3u32
                | i3l.into_inner() << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
