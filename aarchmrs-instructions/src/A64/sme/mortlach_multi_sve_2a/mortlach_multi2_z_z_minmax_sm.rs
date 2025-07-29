/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smax_mz_zzv_2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111111111100001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smax_mz_zzv_2x1";
    #[inline]
    pub const fn smax_mz_zzv_2x1(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<4>,
        Zdn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b10100000000u32 << 5u32
                | Zdn.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod smin_mz_zzv_2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111111111100001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001010000000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smin_mz_zzv_2x1";
    #[inline]
    pub const fn smin_mz_zzv_2x1(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<4>,
        Zdn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b10100000001u32 << 5u32
                | Zdn.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod umax_mz_zzv_2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111111111100001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001010000000000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umax_mz_zzv_2x1";
    #[inline]
    pub const fn umax_mz_zzv_2x1(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<4>,
        Zdn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b10100000000u32 << 5u32
                | Zdn.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod umin_mz_zzv_2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111111111100001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001010000000100001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umin_mz_zzv_2x1";
    #[inline]
    pub const fn umin_mz_zzv_2x1(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<4>,
        Zdn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b10100000001u32 << 5u32
                | Zdn.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
