/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmax_mz_zzv_2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111111111100001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001010000100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmax_mz_zzv_2x1";
    #[inline]
    pub const fn fmax_mz_zzv_2x1(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<4>,
        Zdn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b10100001000u32 << 5u32
                | Zdn.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod bfmax_mz_zzv_2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111111111100001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001010000100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmax_mz_zzv_2x1";
    #[inline]
    pub const fn bfmax_mz_zzv_2x1(
        Zm: ::aarchmrs_types::BitValue<4>,
        Zdn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b10100001000u32 << 5u32
                | Zdn.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod fmin_mz_zzv_2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111111111100001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001010000100000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmin_mz_zzv_2x1";
    #[inline]
    pub const fn fmin_mz_zzv_2x1(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<4>,
        Zdn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b10100001000u32 << 5u32
                | Zdn.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod bfmin_mz_zzv_2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111111111100001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001010000100000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmin_mz_zzv_2x1";
    #[inline]
    pub const fn bfmin_mz_zzv_2x1(
        Zm: ::aarchmrs_types::BitValue<4>,
        Zdn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b10100001000u32 << 5u32
                | Zdn.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod fmaxnm_mz_zzv_2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111111111100001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001010000100100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmaxnm_mz_zzv_2x1";
    #[inline]
    pub const fn fmaxnm_mz_zzv_2x1(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<4>,
        Zdn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b10100001001u32 << 5u32
                | Zdn.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod bfmaxnm_mz_zzv_2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111111111100001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001010000100100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmaxnm_mz_zzv_2x1";
    #[inline]
    pub const fn bfmaxnm_mz_zzv_2x1(
        Zm: ::aarchmrs_types::BitValue<4>,
        Zdn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b10100001001u32 << 5u32
                | Zdn.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod fminnm_mz_zzv_2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001111111111100001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001010000100100001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fminnm_mz_zzv_2x1";
    #[inline]
    pub const fn fminnm_mz_zzv_2x1(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<4>,
        Zdn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b10100001001u32 << 5u32
                | Zdn.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod bfminnm_mz_zzv_2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111111111100001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001010000100100001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfminnm_mz_zzv_2x1";
    #[inline]
    pub const fn bfminnm_mz_zzv_2x1(
        Zm: ::aarchmrs_types::BitValue<4>,
        Zdn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b10100001001u32 << 5u32
                | Zdn.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
