/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmla_za_zzv_2x1_16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000000001110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmla_za_zzv_2x1_16";
    #[inline]
    pub const fn fmla_za_zzv_2x1_16(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        S: ::aarchmrs_types::BitValue<1>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b111u32 << 10u32
                | Zn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | S.into_inner() << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod bfmla_za_zzv_2x1_16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001011000000001110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmla_za_zzv_2x1_16";
    #[inline]
    pub const fn bfmla_za_zzv_2x1_16(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        S: ::aarchmrs_types::BitValue<1>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010110u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b111u32 << 10u32
                | Zn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | S.into_inner() << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod fmls_za_zzv_2x1_16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000000001110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmls_za_zzv_2x1_16";
    #[inline]
    pub const fn fmls_za_zzv_2x1_16(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        S: ::aarchmrs_types::BitValue<1>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b111u32 << 10u32
                | Zn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | S.into_inner() << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod bfmls_za_zzv_2x1_16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001011000000001110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmls_za_zzv_2x1_16";
    #[inline]
    pub const fn bfmls_za_zzv_2x1_16(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        S: ::aarchmrs_types::BitValue<1>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010110u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b111u32 << 10u32
                | Zn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | S.into_inner() << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
