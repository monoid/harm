/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod VDUP_s_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101100000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VDUP_s_A1_D";
    #[inline]
    pub const fn VDUP_s_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b110000u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VDUP_s_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011101100000000110001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VDUP_s_A1_Q";
    #[inline]
    pub const fn VDUP_s_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | imm4.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b110001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
