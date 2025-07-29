/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smax_mz_zzw_4x4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000111111111111100011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smax_mz_zzw_4x4";
    #[inline]
    pub const fn smax_mz_zzw_4x4(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        Zdn: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 18u32
                | 0b0010111000000u32 << 5u32
                | Zdn.into_inner() << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod smin_mz_zzw_4x4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000111111111111100011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001011100000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smin_mz_zzw_4x4";
    #[inline]
    pub const fn smin_mz_zzw_4x4(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        Zdn: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 18u32
                | 0b0010111000001u32 << 5u32
                | Zdn.into_inner() << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod umax_mz_zzw_4x4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000111111111111100011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001011100000000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umax_mz_zzw_4x4";
    #[inline]
    pub const fn umax_mz_zzw_4x4(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        Zdn: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 18u32
                | 0b0010111000000u32 << 5u32
                | Zdn.into_inner() << 2u32
                | 0b01u32 << 0u32,
        )
    }
}
pub mod umin_mz_zzw_4x4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000111111111111100011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001011100000100001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umin_mz_zzw_4x4";
    #[inline]
    pub const fn umin_mz_zzw_4x4(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        Zdn: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 18u32
                | 0b0010111000001u32 << 5u32
                | Zdn.into_inner() << 2u32
                | 0b01u32 << 0u32,
        )
    }
}
