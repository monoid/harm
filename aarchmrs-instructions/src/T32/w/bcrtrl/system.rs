/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CLREX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101111111000111100101111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010111100001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CLREX_T1";
    #[inline]
    pub const fn CLREX_T1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110011101111111000111100101111u32 << 0u32)
    }
}
pub mod DSB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101111111000111101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "DSB_T1";
    #[inline]
    pub const fn DSB_T1(
        option: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001110111111100011110100u32 << 4u32 | option.into_inner() << 0u32,
        )
    }
}
pub mod SSBB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101111111000111101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SSBB_T1";
    #[inline]
    pub const fn SSBB_T1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110011101111111000111101000000u32 << 0u32)
    }
}
pub mod PSSBB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101111111000111101000100u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PSSBB_T1";
    #[inline]
    pub const fn PSSBB_T1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110011101111111000111101000100u32 << 0u32)
    }
}
pub mod DMB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101111111000111101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "DMB_T1";
    #[inline]
    pub const fn DMB_T1(
        option: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001110111111100011110101u32 << 4u32 | option.into_inner() << 0u32,
        )
    }
}
pub mod ISB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101111111000111101100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ISB_T1";
    #[inline]
    pub const fn ISB_T1(
        option: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001110111111100011110110u32 << 4u32 | option.into_inner() << 0u32,
        )
    }
}
pub mod SB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101111111000111101110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010111100001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SB_T1";
    #[inline]
    pub const fn SB_T1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110011101111111000111101110000u32 << 0u32)
    }
}
