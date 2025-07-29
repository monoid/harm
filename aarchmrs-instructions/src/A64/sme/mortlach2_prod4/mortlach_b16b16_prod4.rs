/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod bfmop4a_za_zz_h1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmop4a_za_zz_h1x1";
    #[inline]
    pub const fn bfmop4a_za_zz_h1x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b100000010010u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b00100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod bfmop4s_za_zz_h1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000000000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmop4s_za_zz_h1x1";
    #[inline]
    pub const fn bfmop4s_za_zz_h1x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b100000010010u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b01100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod bfmop4a_za_zz_h1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001100000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmop4a_za_zz_h1x2";
    #[inline]
    pub const fn bfmop4a_za_zz_h1x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b100000010011u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b00100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod bfmop4s_za_zz_h1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001100000000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmop4s_za_zz_h1x2";
    #[inline]
    pub const fn bfmop4s_za_zz_h1x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b100000010011u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b01100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod bfmop4a_za_zz_h2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000000000001000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmop4a_za_zz_h2x1";
    #[inline]
    pub const fn bfmop4a_za_zz_h2x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b100000010010u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b00100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod bfmop4s_za_zz_h2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000000000001000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmop4s_za_zz_h2x1";
    #[inline]
    pub const fn bfmop4s_za_zz_h2x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b100000010010u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b01100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod bfmop4a_za_zz_h2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001100000000001000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmop4a_za_zz_h2x2";
    #[inline]
    pub const fn bfmop4a_za_zz_h2x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b100000010011u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b00100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod bfmop4s_za_zz_h2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001100000000001000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmop4s_za_zz_h2x2";
    #[inline]
    pub const fn bfmop4s_za_zz_h2x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b100000010011u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b01100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
