/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fadd_za_zw_4x4_16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111001110001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001101001010001110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fadd_za_zw_4x4_16";
    #[inline]
    pub const fn fadd_za_zw_4x4_16(
        Rv: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001101001010u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b111u32 << 10u32
                | Zm.into_inner() << 7u32
                | 0b0000u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod bfadd_za_zw_4x4_16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111001110001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001111001010001110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfadd_za_zw_4x4_16";
    #[inline]
    pub const fn bfadd_za_zw_4x4_16(
        Rv: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001111001010u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b111u32 << 10u32
                | Zm.into_inner() << 7u32
                | 0b0000u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod fsub_za_zw_4x4_16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111001110001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001101001010001110000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fsub_za_zw_4x4_16";
    #[inline]
    pub const fn fsub_za_zw_4x4_16(
        Rv: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001101001010u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b111u32 << 10u32
                | Zm.into_inner() << 7u32
                | 0b0001u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod bfsub_za_zw_4x4_16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111001110001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001111001010001110000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfsub_za_zw_4x4_16";
    #[inline]
    pub const fn bfsub_za_zw_4x4_16(
        Rv: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001111001010u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b111u32 << 10u32
                | Zm.into_inner() << 7u32
                | 0b0001u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
