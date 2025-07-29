/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ctermeq_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001111110000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101101000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ctermeq_rr_";
    #[inline]
    pub const fn ctermeq_rr_(
        sz: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001011u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b001000u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b00000u32 << 0u32,
        )
    }
}
pub mod ctermne_rr_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001111110000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101101000000010000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ctermne_rr_";
    #[inline]
    pub const fn ctermne_rr_(
        sz: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001011u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b001000u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b10000u32 << 0u32,
        )
    }
}
