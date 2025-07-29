/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MOVN_32_movewide {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00010010100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOVN_32_movewide";
    #[inline]
    pub const fn MOVN_32_movewide(
        hw: ::aarchmrs_types::BitValue<1>,
        imm16: ::aarchmrs_types::BitValue<16>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001001010u32 << 22u32
                | hw.into_inner() << 21u32
                | imm16.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MOVZ_32_movewide {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01010010100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOVZ_32_movewide";
    #[inline]
    pub const fn MOVZ_32_movewide(
        hw: ::aarchmrs_types::BitValue<1>,
        imm16: ::aarchmrs_types::BitValue<16>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101001010u32 << 22u32
                | hw.into_inner() << 21u32
                | imm16.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MOVK_32_movewide {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110010100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOVK_32_movewide";
    #[inline]
    pub const fn MOVK_32_movewide(
        hw: ::aarchmrs_types::BitValue<1>,
        imm16: ::aarchmrs_types::BitValue<16>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0111001010u32 << 22u32
                | hw.into_inner() << 21u32
                | imm16.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MOVN_64_movewide {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10010010100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOVN_64_movewide";
    #[inline]
    pub const fn MOVN_64_movewide(
        hw: ::aarchmrs_types::BitValue<2>,
        imm16: ::aarchmrs_types::BitValue<16>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b100100101u32 << 23u32
                | hw.into_inner() << 21u32
                | imm16.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MOVZ_64_movewide {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010010100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOVZ_64_movewide";
    #[inline]
    pub const fn MOVZ_64_movewide(
        hw: ::aarchmrs_types::BitValue<2>,
        imm16: ::aarchmrs_types::BitValue<16>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110100101u32 << 23u32
                | hw.into_inner() << 21u32
                | imm16.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MOVK_64_movewide {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOVK_64_movewide";
    #[inline]
    pub const fn MOVK_64_movewide(
        hw: ::aarchmrs_types::BitValue<2>,
        imm16: ::aarchmrs_types::BitValue<16>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100101u32 << 23u32
                | hw.into_inner() << 21u32
                | imm16.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
