/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smax_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001010001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smax_z_zi_";
    #[inline]
    pub const fn smax_z_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        imm8: ::aarchmrs_types::BitValue<8>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b101000110u32 << 13u32
                | imm8.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod smin_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001010101100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smin_z_zi_";
    #[inline]
    pub const fn smin_z_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        imm8: ::aarchmrs_types::BitValue<8>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b101010110u32 << 13u32
                | imm8.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod umax_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001010011100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umax_z_zi_";
    #[inline]
    pub const fn umax_z_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        imm8: ::aarchmrs_types::BitValue<8>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b101001110u32 << 13u32
                | imm8.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod umin_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001010111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umin_z_zi_";
    #[inline]
    pub const fn umin_z_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        imm8: ::aarchmrs_types::BitValue<8>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b101011110u32 << 13u32
                | imm8.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
