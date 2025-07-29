/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod eor3_z_zzz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000000011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "eor3_z_zzz_";
    #[inline]
    pub const fn eor3_z_zzz_(
        Zm: ::aarchmrs_types::BitValue<5>,
        Zk: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100001u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b001110u32 << 10u32
                | Zk.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod bcax_z_zzz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100011000000011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bcax_z_zzz_";
    #[inline]
    pub const fn bcax_z_zzz_(
        Zm: ::aarchmrs_types::BitValue<5>,
        Zk: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100011u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b001110u32 << 10u32
                | Zk.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod bsl_z_zzz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000000011110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bsl_z_zzz_";
    #[inline]
    pub const fn bsl_z_zzz_(
        Zm: ::aarchmrs_types::BitValue<5>,
        Zk: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100001u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b001111u32 << 10u32
                | Zk.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod bsl1n_z_zzz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100011000000011110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bsl1n_z_zzz_";
    #[inline]
    pub const fn bsl1n_z_zzz_(
        Zm: ::aarchmrs_types::BitValue<5>,
        Zk: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100011u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b001111u32 << 10u32
                | Zk.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod bsl2n_z_zzz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100101000000011110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bsl2n_z_zzz_";
    #[inline]
    pub const fn bsl2n_z_zzz_(
        Zm: ::aarchmrs_types::BitValue<5>,
        Zk: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100101u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b001111u32 << 10u32
                | Zk.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod nbsl_z_zzz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100111000000011110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "nbsl_z_zzz_";
    #[inline]
    pub const fn nbsl_z_zzz_(
        Zm: ::aarchmrs_types::BitValue<5>,
        Zk: ::aarchmrs_types::BitValue<5>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100111u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b001111u32 << 10u32
                | Zk.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
