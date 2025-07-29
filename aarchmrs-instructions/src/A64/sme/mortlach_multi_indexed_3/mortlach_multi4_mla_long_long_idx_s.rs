/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlall_za_zzi_s4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001000100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlall_za_zzi_s4xi";
    #[inline]
    pub const fn smlall_za_zzi_s4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i4h: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        i4l: ::aarchmrs_types::BitValue<2>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010001u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i4h.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0000u32 << 3u32
                | i4l.into_inner() << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
pub mod usmlall_za_zzi_s4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001000100001000000000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmlall_za_zzi_s4xi";
    #[inline]
    pub const fn usmlall_za_zzi_s4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i4h: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        i4l: ::aarchmrs_types::BitValue<2>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010001u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i4h.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0100u32 << 3u32
                | i4l.into_inner() << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
pub mod smlsll_za_zzi_s4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001000100001000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlsll_za_zzi_s4xi";
    #[inline]
    pub const fn smlsll_za_zzi_s4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i4h: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        i4l: ::aarchmrs_types::BitValue<2>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010001u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i4h.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0001u32 << 3u32
                | i4l.into_inner() << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
pub mod umlall_za_zzi_s4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001000100001000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlall_za_zzi_s4xi";
    #[inline]
    pub const fn umlall_za_zzi_s4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i4h: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        i4l: ::aarchmrs_types::BitValue<2>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010001u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i4h.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0010u32 << 3u32
                | i4l.into_inner() << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
pub mod sumlall_za_zzi_s4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001000100001000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumlall_za_zzi_s4xi";
    #[inline]
    pub const fn sumlall_za_zzi_s4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i4h: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        i4l: ::aarchmrs_types::BitValue<2>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010001u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i4h.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0110u32 << 3u32
                | i4l.into_inner() << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
pub mod umlsll_za_zzi_s4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001000001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001000100001000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlsll_za_zzi_s4xi";
    #[inline]
    pub const fn umlsll_za_zzi_s4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i4h: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        i4l: ::aarchmrs_types::BitValue<2>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010001u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i4h.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0011u32 << 3u32
                | i4l.into_inner() << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
