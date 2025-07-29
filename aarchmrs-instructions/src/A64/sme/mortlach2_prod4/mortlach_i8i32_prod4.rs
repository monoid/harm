/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smop4a_za_zz_b1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smop4a_za_zz_b1x1";
    #[inline]
    pub const fn smop4a_za_zz_b1x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000000000u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod sumop4a_za_zz_b1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumop4a_za_zz_b1x1";
    #[inline]
    pub const fn sumop4a_za_zz_b1x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000000001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod usmop4a_za_zz_b1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmop4a_za_zz_b1x1";
    #[inline]
    pub const fn usmop4a_za_zz_b1x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001000u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umop4a_za_zz_b1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umop4a_za_zz_b1x1";
    #[inline]
    pub const fn umop4a_za_zz_b1x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod smop4s_za_zz_b1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000000000001000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smop4s_za_zz_b1x1";
    #[inline]
    pub const fn smop4s_za_zz_b1x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000000000u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod sumop4s_za_zz_b1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000001000001000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumop4s_za_zz_b1x1";
    #[inline]
    pub const fn sumop4s_za_zz_b1x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000000001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod usmop4s_za_zz_b1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001000000001000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmop4s_za_zz_b1x1";
    #[inline]
    pub const fn usmop4s_za_zz_b1x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001000u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umop4s_za_zz_b1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000001000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umop4s_za_zz_b1x1";
    #[inline]
    pub const fn umop4s_za_zz_b1x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod smop4a_za_zz_b1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smop4a_za_zz_b1x2";
    #[inline]
    pub const fn smop4a_za_zz_b1x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000000000u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod sumop4a_za_zz_b1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumop4a_za_zz_b1x2";
    #[inline]
    pub const fn sumop4a_za_zz_b1x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000000001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod usmop4a_za_zz_b1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmop4a_za_zz_b1x2";
    #[inline]
    pub const fn usmop4a_za_zz_b1x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001000u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umop4a_za_zz_b1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umop4a_za_zz_b1x2";
    #[inline]
    pub const fn umop4a_za_zz_b1x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod smop4s_za_zz_b1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000000000001000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smop4s_za_zz_b1x2";
    #[inline]
    pub const fn smop4s_za_zz_b1x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000000000u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod sumop4s_za_zz_b1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000001000001000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumop4s_za_zz_b1x2";
    #[inline]
    pub const fn sumop4s_za_zz_b1x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000000001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod usmop4s_za_zz_b1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001000000001000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmop4s_za_zz_b1x2";
    #[inline]
    pub const fn usmop4s_za_zz_b1x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001000u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umop4s_za_zz_b1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000001000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umop4s_za_zz_b1x2";
    #[inline]
    pub const fn umop4s_za_zz_b1x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod smop4a_za_zz_b2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smop4a_za_zz_b2x1";
    #[inline]
    pub const fn smop4a_za_zz_b2x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000000000u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod sumop4a_za_zz_b2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumop4a_za_zz_b2x1";
    #[inline]
    pub const fn sumop4a_za_zz_b2x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000000001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod usmop4a_za_zz_b2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmop4a_za_zz_b2x1";
    #[inline]
    pub const fn usmop4a_za_zz_b2x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001000u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umop4a_za_zz_b2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umop4a_za_zz_b2x1";
    #[inline]
    pub const fn umop4a_za_zz_b2x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod smop4s_za_zz_b2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000000000001000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smop4s_za_zz_b2x1";
    #[inline]
    pub const fn smop4s_za_zz_b2x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000000000u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod sumop4s_za_zz_b2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000001000001000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumop4s_za_zz_b2x1";
    #[inline]
    pub const fn sumop4s_za_zz_b2x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000000001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod usmop4s_za_zz_b2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001000000001000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmop4s_za_zz_b2x1";
    #[inline]
    pub const fn usmop4s_za_zz_b2x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001000u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umop4s_za_zz_b2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000001000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umop4s_za_zz_b2x1";
    #[inline]
    pub const fn umop4s_za_zz_b2x1(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod smop4a_za_zz_b2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smop4a_za_zz_b2x2";
    #[inline]
    pub const fn smop4a_za_zz_b2x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000000000u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod sumop4a_za_zz_b2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumop4a_za_zz_b2x2";
    #[inline]
    pub const fn sumop4a_za_zz_b2x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000000001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod usmop4a_za_zz_b2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmop4a_za_zz_b2x2";
    #[inline]
    pub const fn usmop4a_za_zz_b2x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001000u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umop4a_za_zz_b2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umop4a_za_zz_b2x2";
    #[inline]
    pub const fn umop4a_za_zz_b2x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod smop4s_za_zz_b2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000000000001000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smop4s_za_zz_b2x2";
    #[inline]
    pub const fn smop4s_za_zz_b2x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000000000u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod sumop4s_za_zz_b2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000001000001000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumop4s_za_zz_b2x2";
    #[inline]
    pub const fn sumop4s_za_zz_b2x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000000001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod usmop4s_za_zz_b2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001000000001000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmop4s_za_zz_b2x2";
    #[inline]
    pub const fn usmop4s_za_zz_b2x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001000u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umop4s_za_zz_b2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000001001000001000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umop4s_za_zz_b2x2";
    #[inline]
    pub const fn umop4s_za_zz_b2x2(
        M: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<3>,
        N: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000001001u32 << 21u32
                | M.into_inner() << 20u32
                | Zm.into_inner() << 17u32
                | 0b0100000u32 << 10u32
                | N.into_inner() << 9u32
                | Zn.into_inner() << 6u32
                | 0b0100u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
