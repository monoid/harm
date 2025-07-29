/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod VMOV_i_A2_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100000000100100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000010100000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_i_A2_H";
    #[inline]
    pub const fn VMOV_i_A2_H(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4H.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b10010000u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod VMOV_i_A2_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000010100000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_i_A2_S";
    #[inline]
    pub const fn VMOV_i_A2_S(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4H.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b10100000u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
pub mod VMOV_i_A2_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101100000000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100000000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000010100000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_i_A2_D";
    #[inline]
    pub const fn VMOV_i_A2_D(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        imm4H: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm4L: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4H.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b10110000u32 << 4u32
                | imm4L.into_inner() << 0u32,
        )
    }
}
