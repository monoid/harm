/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sdot_za32_zzw_2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011001110000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001111000000001010000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sdot_za32_zzw_2x2";
    #[inline]
    pub const fn sdot_za32_zzw_2x2(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001111u32 << 21u32
                | Zm.into_inner() << 17u32
                | 0b00u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b101u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b001u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod udot_za32_zzw_2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011001110000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001111000000001010000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "udot_za32_zzw_2x2";
    #[inline]
    pub const fn udot_za32_zzw_2x2(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001111u32 << 21u32
                | Zm.into_inner() << 17u32
                | 0b00u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b101u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b011u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
