/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod zip1_p_pp_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111101000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101001000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "zip1_p_pp_";
    #[inline]
    pub const fn zip1_p_pp_(
        size: ::aarchmrs_types::BitValue<2>,
        Pm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01000u32 << 11u32
                | H.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod uzp1_p_pp_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111101000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101001000000100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uzp1_p_pp_";
    #[inline]
    pub const fn uzp1_p_pp_(
        size: ::aarchmrs_types::BitValue<2>,
        Pm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01001u32 << 11u32
                | H.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod trn1_p_pp_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111101000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101001000000101000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "trn1_p_pp_";
    #[inline]
    pub const fn trn1_p_pp_(
        size: ::aarchmrs_types::BitValue<2>,
        Pm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01010u32 << 11u32
                | H.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod zip2_p_pp_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111101000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101001000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "zip2_p_pp_";
    #[inline]
    pub const fn zip2_p_pp_(
        size: ::aarchmrs_types::BitValue<2>,
        Pm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01000u32 << 11u32
                | H.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod uzp2_p_pp_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111101000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101001000000100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uzp2_p_pp_";
    #[inline]
    pub const fn uzp2_p_pp_(
        size: ::aarchmrs_types::BitValue<2>,
        Pm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01001u32 << 11u32
                | H.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod trn2_p_pp_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111101000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101001000000101000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "trn2_p_pp_";
    #[inline]
    pub const fn trn2_p_pp_(
        size: ::aarchmrs_types::BitValue<2>,
        Pm: ::aarchmrs_types::BitValue<4>,
        H: ::aarchmrs_types::BitValue<1>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Pm.into_inner() << 16u32
                | 0b01010u32 << 11u32
                | H.into_inner() << 10u32
                | 0b0u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
