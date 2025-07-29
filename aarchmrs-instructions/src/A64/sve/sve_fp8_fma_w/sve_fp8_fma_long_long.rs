/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlallbb_z32_z8z8z8_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100001000001000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmlallbb_z32_z8z8z8_";
    #[inline]
    pub const fn fmlallbb_z32_z8z8z8_(
        Zm: ::aarchmrs_types::BitValue<5>,
        TT: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100001u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b10u32 << 14u32
                | TT.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod fmlallbt_z32_z8z8z8_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100001000001000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmlallbt_z32_z8z8z8_";
    #[inline]
    pub const fn fmlallbt_z32_z8z8z8_(
        Zm: ::aarchmrs_types::BitValue<5>,
        TT: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100001u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b10u32 << 14u32
                | TT.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod fmlalltb_z32_z8z8z8_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100001000001000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmlalltb_z32_z8z8z8_";
    #[inline]
    pub const fn fmlalltb_z32_z8z8z8_(
        Zm: ::aarchmrs_types::BitValue<5>,
        TT: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100001u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b10u32 << 14u32
                | TT.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod fmlalltt_z32_z8z8z8_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100001000001000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmlalltt_z32_z8z8z8_";
    #[inline]
    pub const fn fmlalltt_z32_z8z8z8_(
        Zm: ::aarchmrs_types::BitValue<5>,
        TT: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100001u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b10u32 << 14u32
                | TT.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
