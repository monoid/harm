/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CLREX_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110101011111111111000000011111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011111111111100001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CLREX_A1";
    #[inline]
    pub const fn CLREX_A1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110101011111111111000000011111u32 << 0u32)
    }
}
pub mod DSB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110101011111111111000001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011111111111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "DSB_A1";
    #[inline]
    pub const fn DSB_A1(
        option: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111010101111111111100000100u32 << 4u32 | option.into_inner() << 0u32,
        )
    }
}
pub mod SSBB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110101011111111111000001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011111111111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SSBB_A1";
    #[inline]
    pub const fn SSBB_A1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110101011111111111000001000000u32 << 0u32)
    }
}
pub mod PSSBB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110101011111111111000001000100u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011111111111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PSSBB_A1";
    #[inline]
    pub const fn PSSBB_A1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110101011111111111000001000100u32 << 0u32)
    }
}
pub mod DMB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110101011111111111000001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011111111111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "DMB_A1";
    #[inline]
    pub const fn DMB_A1(
        option: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111010101111111111100000101u32 << 4u32 | option.into_inner() << 0u32,
        )
    }
}
pub mod ISB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110101011111111111000001100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011111111111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ISB_A1";
    #[inline]
    pub const fn ISB_A1(
        option: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111010101111111111100000110u32 << 4u32 | option.into_inner() << 0u32,
        )
    }
}
pub mod SB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110101011111111111000001110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011111111111100001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SB_A1";
    #[inline]
    pub const fn SB_A1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110101011111111111000001110000u32 << 0u32)
    }
}
