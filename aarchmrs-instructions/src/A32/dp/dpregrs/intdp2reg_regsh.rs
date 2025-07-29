/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod TST_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001000100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TST_rr_A1";
    #[inline]
    pub const fn TST_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rs.into_inner() << 8u32
                | 0b0u32 << 7u32
                | stype.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod TEQ_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001001100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TEQ_rr_A1";
    #[inline]
    pub const fn TEQ_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rs.into_inner() << 8u32
                | 0b0u32 << 7u32
                | stype.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod CMP_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001010100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMP_rr_A1";
    #[inline]
    pub const fn CMP_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rs.into_inner() << 8u32
                | 0b0u32 << 7u32
                | stype.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod CMN_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001011100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMN_rr_A1";
    #[inline]
    pub const fn CMN_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010111u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rs.into_inner() << 8u32
                | 0b0u32 << 7u32
                | stype.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
