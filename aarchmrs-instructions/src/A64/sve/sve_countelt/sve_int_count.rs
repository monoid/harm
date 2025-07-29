/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cntb_r_s_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cntb_r_s_";
    #[inline]
    pub const fn cntb_r_s_(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111000u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod cnth_r_s_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cnth_r_s_";
    #[inline]
    pub const fn cnth_r_s_(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111000u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod cntw_r_s_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cntw_r_s_";
    #[inline]
    pub const fn cntw_r_s_(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111000u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod cntd_r_s_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cntd_r_s_";
    #[inline]
    pub const fn cntd_r_s_(
        size: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111000u32 << 10u32
                | pattern.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
