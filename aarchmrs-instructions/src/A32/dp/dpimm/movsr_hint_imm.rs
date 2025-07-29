/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MSR_i_A1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000011001000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MSR_i_A1_AS";
    #[inline]
    pub const fn MSR_i_A1_AS(
        cond: ::aarchmrs_types::BitValue<4>,
        R: ::aarchmrs_types::BitValue<1>,
        mask: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00110u32 << 23u32
                | R.into_inner() << 22u32
                | 0b10u32 << 20u32
                | mask.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod NOP_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000011001000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "NOP_A1";
    #[inline]
    pub const fn NOP_A1(cond: ::aarchmrs_types::BitValue<4>) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32 | 0b0011001000001111000000000000u32 << 0u32,
        )
    }
}
pub mod YIELD_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000011001000001111000000000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "YIELD_A1";
    #[inline]
    pub const fn YIELD_A1(
        cond: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32 | 0b0011001000001111000000000001u32 << 0u32,
        )
    }
}
pub mod WFE_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000011001000001111000000000010u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "WFE_A1";
    #[inline]
    pub const fn WFE_A1(cond: ::aarchmrs_types::BitValue<4>) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32 | 0b0011001000001111000000000010u32 << 0u32,
        )
    }
}
pub mod WFI_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000011001000001111000000000011u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "WFI_A1";
    #[inline]
    pub const fn WFI_A1(cond: ::aarchmrs_types::BitValue<4>) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32 | 0b0011001000001111000000000011u32 << 0u32,
        )
    }
}
pub mod SEV_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000011001000001111000000000100u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SEV_A1";
    #[inline]
    pub const fn SEV_A1(cond: ::aarchmrs_types::BitValue<4>) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32 | 0b0011001000001111000000000100u32 << 0u32,
        )
    }
}
pub mod SEVL_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000011001000001111000000000101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SEVL_A1";
    #[inline]
    pub const fn SEVL_A1(cond: ::aarchmrs_types::BitValue<4>) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32 | 0b0011001000001111000000000101u32 << 0u32,
        )
    }
}
pub mod ESB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000011001000001111000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ESB_A1";
    #[inline]
    pub const fn ESB_A1(cond: ::aarchmrs_types::BitValue<4>) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32 | 0b0011001000001111000000010000u32 << 0u32,
        )
    }
}
pub mod TSB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000011001000001111000000010010u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TSB_A1";
    #[inline]
    pub const fn TSB_A1(cond: ::aarchmrs_types::BitValue<4>) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32 | 0b0011001000001111000000010010u32 << 0u32,
        )
    }
}
pub mod CSDB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000011001000001111000000010100u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CSDB_A1";
    #[inline]
    pub const fn CSDB_A1(cond: ::aarchmrs_types::BitValue<4>) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32 | 0b0011001000001111000000010100u32 << 0u32,
        )
    }
}
pub mod CLRBHB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000011001000001111000000010110u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CLRBHB_A1";
    #[inline]
    pub const fn CLRBHB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32 | 0b0011001000001111000000010110u32 << 0u32,
        )
    }
}
pub mod DBG_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111111111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000011001000001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "DBG_A1";
    #[inline]
    pub const fn DBG_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        option: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b001100100000111100001111u32 << 4u32
                | option.into_inner() << 0u32,
        )
    }
}
