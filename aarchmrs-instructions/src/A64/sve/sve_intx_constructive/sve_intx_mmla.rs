/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smmla_z_zzz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101000000001001100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smmla_z_zzz_";
    #[inline]
    pub const fn smmla_z_zzz_(
        Zm: ::aarchmrs_types::BitValue<5>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101000u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b100110u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod usmmla_z_zzz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101100000001001100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "usmmla_z_zzz_";
    #[inline]
    pub const fn usmmla_z_zzz_(
        Zm: ::aarchmrs_types::BitValue<5>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101100u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b100110u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod ummla_z_zzz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101110000001001100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ummla_z_zzz_";
    #[inline]
    pub const fn ummla_z_zzz_(
        Zm: ::aarchmrs_types::BitValue<5>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101110u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b100110u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
