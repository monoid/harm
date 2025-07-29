/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fcvtn_z8_mz2_h2b {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000010100011000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtn_z8_mz2_h2b";
    #[inline]
    pub const fn fcvtn_z8_mz2_h2b(
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110010100001010001100u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fcvtnb_z8_mz2_s2b {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000010100011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtnb_z8_mz2_s2b";
    #[inline]
    pub const fn fcvtnb_z8_mz2_s2b(
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110010100001010001101u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod bfcvtn_z8_mz2_bf2b {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000010100011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfcvtn_z8_mz2_bf2b";
    #[inline]
    pub const fn bfcvtn_z8_mz2_bf2b(
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110010100001010001110u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fcvtnt_z8_mz2_s2b {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100101000010100011110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtnt_z8_mz2_s2b";
    #[inline]
    pub const fn fcvtnt_z8_mz2_s2b(
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110010100001010001111u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
