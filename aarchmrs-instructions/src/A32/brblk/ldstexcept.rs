/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod RFEDA_A1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110100001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000000100000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RFEDA_A1_AS";
    #[inline]
    pub const fn RFEDA_A1_AS(
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111100000u32 << 22u32
                | W.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000101000000000u32 << 0u32,
        )
    }
}
pub mod RFEDB_A1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110100001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000100000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RFEDB_A1_AS";
    #[inline]
    pub const fn RFEDB_A1_AS(
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111100100u32 << 22u32
                | W.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000101000000000u32 << 0u32,
        )
    }
}
pub mod RFEIA_A1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110100001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000100100000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RFEIA_A1_AS";
    #[inline]
    pub const fn RFEIA_A1_AS(
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111100010u32 << 22u32
                | W.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000101000000000u32 << 0u32,
        )
    }
}
pub mod RFEIB_A1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110100001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001100100000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RFEIB_A1_AS";
    #[inline]
    pub const fn RFEIB_A1_AS(
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111100110u32 << 22u32
                | W.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000101000000000u32 << 0u32,
        )
    }
}
pub mod SRSDA_A1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110111111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000010011010000010100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SRSDA_A1_AS";
    #[inline]
    pub const fn SRSDA_A1_AS(
        W: ::aarchmrs_types::BitValue<1>,
        mode: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111100001u32 << 22u32
                | W.into_inner() << 21u32
                | 0b0110100000101000u32 << 5u32
                | mode.into_inner() << 0u32,
        )
    }
}
pub mod SRSDB_A1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110111111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001010011010000010100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SRSDB_A1_AS";
    #[inline]
    pub const fn SRSDB_A1_AS(
        W: ::aarchmrs_types::BitValue<1>,
        mode: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111100101u32 << 22u32
                | W.into_inner() << 21u32
                | 0b0110100000101000u32 << 5u32
                | mode.into_inner() << 0u32,
        )
    }
}
pub mod SRSIA_A1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110111111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000110011010000010100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SRSIA_A1_AS";
    #[inline]
    pub const fn SRSIA_A1_AS(
        W: ::aarchmrs_types::BitValue<1>,
        mode: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111100011u32 << 22u32
                | W.into_inner() << 21u32
                | 0b0110100000101000u32 << 5u32
                | mode.into_inner() << 0u32,
        )
    }
}
pub mod SRSIB_A1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110111111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001110011010000010100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SRSIB_A1_AS";
    #[inline]
    pub const fn SRSIB_A1_AS(
        W: ::aarchmrs_types::BitValue<1>,
        mode: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111100111u32 << 22u32
                | W.into_inner() << 21u32
                | 0b0110100000101000u32 << 5u32
                | mode.into_inner() << 0u32,
        )
    }
}
