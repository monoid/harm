/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STREXB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110000000000111101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STREXB_T1";
    #[inline]
    pub const fn STREXB_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b11110100u32 << 4u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod STREXH_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STREXH_T1";
    #[inline]
    pub const fn STREXH_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b11110101u32 << 4u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod STREXD_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110000000000000001110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STREXD_T1";
    #[inline]
    pub const fn STREXD_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rt2: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | Rt2.into_inner() << 8u32
                | 0b0111u32 << 4u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod LDREXB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110100000000111101001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDREXB_T1";
    #[inline]
    pub const fn LDREXB_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b111101001111u32 << 0u32,
        )
    }
}
pub mod LDREXH_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110100000000111101011111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDREXH_T1";
    #[inline]
    pub const fn LDREXH_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | 0b111101011111u32 << 0u32,
        )
    }
}
pub mod LDREXD_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000011111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110100000000000001111111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDREXD_T1";
    #[inline]
    pub const fn LDREXD_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<4>,
        Rt2: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rt.into_inner() << 12u32
                | Rt2.into_inner() << 8u32
                | 0b01111111u32 << 0u32,
        )
    }
}
