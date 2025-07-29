/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ldr_za_ri_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111001110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100001000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldr_za_ri_";
    #[inline]
    pub const fn ldr_za_ri_(
        Rv: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        off4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11100001000000000u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | off4.into_inner() << 0u32,
        )
    }
}
pub mod str_za_ri_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111001110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100001001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "str_za_ri_";
    #[inline]
    pub const fn str_za_ri_(
        Rv: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        off4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11100001001000000u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | off4.into_inner() << 0u32,
        )
    }
}
