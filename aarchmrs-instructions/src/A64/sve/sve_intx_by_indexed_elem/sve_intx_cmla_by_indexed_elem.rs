/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cmla_z_zzzi_h {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100101000000110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmla_z_zzzi_h";
    #[inline]
    pub const fn cmla_z_zzzi_h(
        i2: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        rot: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | i2.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b0110u32 << 12u32
                | rot.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod cmla_z_zzzi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100111000000110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cmla_z_zzzi_s";
    #[inline]
    pub const fn cmla_z_zzzi_s(
        i1: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        rot: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | i1.into_inner() << 20u32
                | Zm.into_inner() << 16u32
                | 0b0110u32 << 12u32
                | rot.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
