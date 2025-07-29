/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SVC_EX_exception {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010100000000000000000000000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SVC_EX_exception";
    #[inline]
    pub const fn SVC_EX_exception(
        imm16: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010100000u32 << 21u32 | imm16.into_inner() << 5u32 | 0b00001u32 << 0u32,
        )
    }
}
pub mod HVC_EX_exception {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010100000000000000000000000010u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "HVC_EX_exception";
    #[inline]
    pub const fn HVC_EX_exception(
        imm16: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010100000u32 << 21u32 | imm16.into_inner() << 5u32 | 0b00010u32 << 0u32,
        )
    }
}
pub mod SMC_EX_exception {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010100000000000000000000000011u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMC_EX_exception";
    #[inline]
    pub const fn SMC_EX_exception(
        imm16: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010100000u32 << 21u32 | imm16.into_inner() << 5u32 | 0b00011u32 << 0u32,
        )
    }
}
pub mod BRK_EX_exception {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010100001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BRK_EX_exception";
    #[inline]
    pub const fn BRK_EX_exception(
        imm16: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010100001u32 << 21u32 | imm16.into_inner() << 5u32 | 0b00000u32 << 0u32,
        )
    }
}
pub mod HLT_EX_exception {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010100010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "HLT_EX_exception";
    #[inline]
    pub const fn HLT_EX_exception(
        imm16: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010100010u32 << 21u32 | imm16.into_inner() << 5u32 | 0b00000u32 << 0u32,
        )
    }
}
pub mod TCANCEL_EX_exception {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010100011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TCANCEL_EX_exception";
    #[inline]
    pub const fn TCANCEL_EX_exception(
        imm16: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010100011u32 << 21u32 | imm16.into_inner() << 5u32 | 0b00000u32 << 0u32,
        )
    }
}
pub mod DCPS1_DC_exception {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010100101000000000000000000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "DCPS1_DC_exception";
    #[inline]
    pub const fn DCPS1_DC_exception(
        imm16: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010100101u32 << 21u32 | imm16.into_inner() << 5u32 | 0b00001u32 << 0u32,
        )
    }
}
pub mod DCPS2_DC_exception {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010100101000000000000000000010u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "DCPS2_DC_exception";
    #[inline]
    pub const fn DCPS2_DC_exception(
        imm16: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010100101u32 << 21u32 | imm16.into_inner() << 5u32 | 0b00010u32 << 0u32,
        )
    }
}
pub mod DCPS3_DC_exception {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010100101000000000000000000011u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "DCPS3_DC_exception";
    #[inline]
    pub const fn DCPS3_DC_exception(
        imm16: ::aarchmrs_types::BitValue<16>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010100101u32 << 21u32 | imm16.into_inner() << 5u32 | 0b00011u32 << 0u32,
        )
    }
}
