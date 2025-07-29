/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smop4a_za_zz_h1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000110000000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smop4a_za_zz_h1x1";
    #[inline]
    pub const fn smop4a_za_zz_h1x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000001100u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b001u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod sumop4a_za_zz_h1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000111000000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumop4a_za_zz_h1x1";
    #[inline]
    pub const fn sumop4a_za_zz_h1x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000001110u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b001u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod usmop4a_za_zz_h1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001110000000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmop4a_za_zz_h1x1";
    #[inline]
    pub const fn usmop4a_za_zz_h1x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000011100u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b001u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umop4a_za_zz_h1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001111000000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umop4a_za_zz_h1x1";
    #[inline]
    pub const fn umop4a_za_zz_h1x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000011110u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b001u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod smop4s_za_zz_h1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000110000000000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smop4s_za_zz_h1x1";
    #[inline]
    pub const fn smop4s_za_zz_h1x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000001100u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b011u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod sumop4s_za_zz_h1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000111000000000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumop4s_za_zz_h1x1";
    #[inline]
    pub const fn sumop4s_za_zz_h1x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000001110u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b011u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod usmop4s_za_zz_h1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001110000000000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmop4s_za_zz_h1x1";
    #[inline]
    pub const fn usmop4s_za_zz_h1x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000011100u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b011u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umop4s_za_zz_h1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001111000000000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umop4s_za_zz_h1x1";
    #[inline]
    pub const fn umop4s_za_zz_h1x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000011110u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b011u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod smop4a_za_zz_h1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000110100000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smop4a_za_zz_h1x2";
    #[inline]
    pub const fn smop4a_za_zz_h1x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000001101u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b001u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod sumop4a_za_zz_h1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000111100000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumop4a_za_zz_h1x2";
    #[inline]
    pub const fn sumop4a_za_zz_h1x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000001111u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b001u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod usmop4a_za_zz_h1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001110100000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmop4a_za_zz_h1x2";
    #[inline]
    pub const fn usmop4a_za_zz_h1x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000011101u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b001u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umop4a_za_zz_h1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001111100000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umop4a_za_zz_h1x2";
    #[inline]
    pub const fn umop4a_za_zz_h1x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000011111u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b001u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod smop4s_za_zz_h1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000110100000000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smop4s_za_zz_h1x2";
    #[inline]
    pub const fn smop4s_za_zz_h1x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000001101u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b011u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod sumop4s_za_zz_h1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000111100000000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumop4s_za_zz_h1x2";
    #[inline]
    pub const fn sumop4s_za_zz_h1x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000001111u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b011u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod usmop4s_za_zz_h1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001110100000000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmop4s_za_zz_h1x2";
    #[inline]
    pub const fn usmop4s_za_zz_h1x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000011101u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b011u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umop4s_za_zz_h1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001111100000000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umop4s_za_zz_h1x2";
    #[inline]
    pub const fn umop4s_za_zz_h1x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000011111u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b011u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod smop4a_za_zz_h2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000110000000000001000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smop4a_za_zz_h2x1";
    #[inline]
    pub const fn smop4a_za_zz_h2x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000001100u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b001u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod sumop4a_za_zz_h2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000111000000000001000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumop4a_za_zz_h2x1";
    #[inline]
    pub const fn sumop4a_za_zz_h2x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000001110u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b001u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod usmop4a_za_zz_h2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001110000000000001000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmop4a_za_zz_h2x1";
    #[inline]
    pub const fn usmop4a_za_zz_h2x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000011100u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b001u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umop4a_za_zz_h2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001111000000000001000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umop4a_za_zz_h2x1";
    #[inline]
    pub const fn umop4a_za_zz_h2x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000011110u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b001u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod smop4s_za_zz_h2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000110000000000001000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smop4s_za_zz_h2x1";
    #[inline]
    pub const fn smop4s_za_zz_h2x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000001100u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b011u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod sumop4s_za_zz_h2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000111000000000001000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumop4s_za_zz_h2x1";
    #[inline]
    pub const fn sumop4s_za_zz_h2x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000001110u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b011u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod usmop4s_za_zz_h2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001110000000000001000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmop4s_za_zz_h2x1";
    #[inline]
    pub const fn usmop4s_za_zz_h2x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000011100u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b011u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umop4s_za_zz_h2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001111000000000001000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umop4s_za_zz_h2x1";
    #[inline]
    pub const fn umop4s_za_zz_h2x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000011110u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b011u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod smop4a_za_zz_h2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000110100000000001000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smop4a_za_zz_h2x2";
    #[inline]
    pub const fn smop4a_za_zz_h2x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000001101u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b001u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod sumop4a_za_zz_h2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000111100000000001000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumop4a_za_zz_h2x2";
    #[inline]
    pub const fn sumop4a_za_zz_h2x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000001111u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b001u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod usmop4a_za_zz_h2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001110100000000001000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmop4a_za_zz_h2x2";
    #[inline]
    pub const fn usmop4a_za_zz_h2x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000011101u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b001u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umop4a_za_zz_h2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001111100000000001000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umop4a_za_zz_h2x2";
    #[inline]
    pub const fn umop4a_za_zz_h2x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000011111u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b001u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod smop4s_za_zz_h2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000110100000000001000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smop4s_za_zz_h2x2";
    #[inline]
    pub const fn smop4s_za_zz_h2x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000001101u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b011u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod sumop4s_za_zz_h2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000111100000000001000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sumop4s_za_zz_h2x2";
    #[inline]
    pub const fn sumop4s_za_zz_h2x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000001111u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b011u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod usmop4s_za_zz_h2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001110100000000001000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmop4s_za_zz_h2x2";
    #[inline]
    pub const fn usmop4s_za_zz_h2x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000011101u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b011u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod umop4s_za_zz_h2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001111100000000001000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umop4s_za_zz_h2x2";
    #[inline]
    pub const fn umop4s_za_zz_h2x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000011111u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b011u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
