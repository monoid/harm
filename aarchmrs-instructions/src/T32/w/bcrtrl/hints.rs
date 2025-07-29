/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod NOP_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101011111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010100000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "NOP_T2";
    #[inline]
    pub const fn NOP_T2() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110011101011111000000000000000u32 << 0u32)
    }
}
pub mod YIELD_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101011111000000000000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010100000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "YIELD_T2";
    #[inline]
    pub const fn YIELD_T2() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110011101011111000000000000001u32 << 0u32)
    }
}
pub mod WFE_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101011111000000000000010u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010100000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "WFE_T2";
    #[inline]
    pub const fn WFE_T2() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110011101011111000000000000010u32 << 0u32)
    }
}
pub mod WFI_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101011111000000000000011u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010100000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "WFI_T2";
    #[inline]
    pub const fn WFI_T2() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110011101011111000000000000011u32 << 0u32)
    }
}
pub mod SEV_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101011111000000000000100u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010100000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SEV_T2";
    #[inline]
    pub const fn SEV_T2() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110011101011111000000000000100u32 << 0u32)
    }
}
pub mod SEVL_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101011111000000000000101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010100000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SEVL_T2";
    #[inline]
    pub const fn SEVL_T2() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110011101011111000000000000101u32 << 0u32)
    }
}
pub mod ESB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101011111000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010100000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ESB_T1";
    #[inline]
    pub const fn ESB_T1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110011101011111000000000010000u32 << 0u32)
    }
}
pub mod TSB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101011111000000000010010u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010100000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TSB_T1";
    #[inline]
    pub const fn TSB_T1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110011101011111000000000010010u32 << 0u32)
    }
}
pub mod CSDB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101011111000000000010100u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010100000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CSDB_T1";
    #[inline]
    pub const fn CSDB_T1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110011101011111000000000010100u32 << 0u32)
    }
}
pub mod CLRBHB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101011111000000000010110u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010100000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CLRBHB_T1";
    #[inline]
    pub const fn CLRBHB_T1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110011101011111000000000010110u32 << 0u32)
    }
}
pub mod DBG_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101011111000000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110010100000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "DBG_T1";
    #[inline]
    pub const fn DBG_T1(
        option: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001110101111100000001111u32 << 4u32 | option.into_inner() << 0u32,
        )
    }
}
