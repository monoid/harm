/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cpy_z_o_i_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101000100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cpy_z_o_i_";
    #[inline]
    pub const fn cpy_z_o_i_(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<4>,
        sh: ::aarchmrs_types::BitValue<1>,
        imm8: ::aarchmrs_types::BitValue<8>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b01u32 << 20u32
                | Pg.into_inner() << 16u32
                | 0b00u32 << 14u32
                | sh.into_inner() << 13u32
                | imm8.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod cpy_z_p_i_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001100001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101000100000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cpy_z_p_i_";
    #[inline]
    pub const fn cpy_z_p_i_(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<4>,
        sh: ::aarchmrs_types::BitValue<1>,
        imm8: ::aarchmrs_types::BitValue<8>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b01u32 << 20u32
                | Pg.into_inner() << 16u32
                | 0b01u32 << 14u32
                | sh.into_inner() << 13u32
                | imm8.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
