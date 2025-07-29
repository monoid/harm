/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STTXR_SR32_ldstexclr_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10001001000000000111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STTXR_SR32_ldstexclr_unpriv";
    #[inline]
    pub const fn STTXR_SR32_ldstexclr_unpriv(
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10001001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STLTXR_SR32_ldstexclr_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10001001000000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STLTXR_SR32_ldstexclr_unpriv";
    #[inline]
    pub const fn STLTXR_SR32_ldstexclr_unpriv(
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10001001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDTXR_LR32_ldstexclr_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10001001010111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDTXR_LR32_ldstexclr_unpriv";
    #[inline]
    pub const fn LDTXR_LR32_ldstexclr_unpriv(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000100101011111011111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDATXR_LR32_ldstexclr_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10001001010111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDATXR_LR32_ldstexclr_unpriv";
    #[inline]
    pub const fn LDATXR_LR32_ldstexclr_unpriv(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000100101011111111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STTXR_SR64_ldstexclr_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001001000000000111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STTXR_SR64_ldstexclr_unpriv";
    #[inline]
    pub const fn STTXR_SR64_ldstexclr_unpriv(
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b011111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STLTXR_SR64_ldstexclr_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001001000000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STLTXR_SR64_ldstexclr_unpriv";
    #[inline]
    pub const fn STLTXR_SR64_ldstexclr_unpriv(
        Rs: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001001000u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDTXR_LR64_ldstexclr_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001001010111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDTXR_LR64_ldstexclr_unpriv";
    #[inline]
    pub const fn LDTXR_LR64_ldstexclr_unpriv(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100100101011111011111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDATXR_LR64_ldstexclr_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001001010111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDATXR_LR64_ldstexclr_unpriv";
    #[inline]
    pub const fn LDATXR_LR64_ldstexclr_unpriv(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100100101011111111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
