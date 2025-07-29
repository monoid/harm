/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod bfmop4a_za_zz_h1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmop4a_za_zz_h1x1";
    #[inline]
    pub const fn bfmop4a_za_zz_h1x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0000000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b00100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod bfmop4s_za_zz_h1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000000000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmop4s_za_zz_h1x1";
    #[inline]
    pub const fn bfmop4s_za_zz_h1x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0000000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b01100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod bfmop4a_za_zz_h1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmop4a_za_zz_h1x2";
    #[inline]
    pub const fn bfmop4a_za_zz_h1x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0000000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b00100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod bfmop4s_za_zz_h1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000000000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmop4s_za_zz_h1x2";
    #[inline]
    pub const fn bfmop4s_za_zz_h1x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0000000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b01100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod bfmop4a_za_zz_h2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmop4a_za_zz_h2x1";
    #[inline]
    pub const fn bfmop4a_za_zz_h2x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0000000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b00100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod bfmop4s_za_zz_h2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000000000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmop4s_za_zz_h2x1";
    #[inline]
    pub const fn bfmop4s_za_zz_h2x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0000000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b01100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod bfmop4a_za_zz_h2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmop4a_za_zz_h2x2";
    #[inline]
    pub const fn bfmop4a_za_zz_h2x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0000000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b00100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod bfmop4s_za_zz_h2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000000000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmop4s_za_zz_h2x2";
    #[inline]
    pub const fn bfmop4s_za_zz_h2x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0000000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b01100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
