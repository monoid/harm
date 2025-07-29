/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STR_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STR_i_T1";
    #[inline]
    pub const fn STR_i_T1(
        imm5: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<3>,
        Rt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100u32 << 11u32
                | imm5.into_inner() << 6u32
                | Rn.into_inner() << 3u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDR_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000110100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDR_i_T1";
    #[inline]
    pub const fn LDR_i_T1(
        imm5: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<3>,
        Rt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01101u32 << 11u32
                | imm5.into_inner() << 6u32
                | Rn.into_inner() << 3u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STRB_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRB_i_T1";
    #[inline]
    pub const fn STRB_i_T1(
        imm5: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<3>,
        Rt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110u32 << 11u32
                | imm5.into_inner() << 6u32
                | Rn.into_inner() << 3u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDRB_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRB_i_T1";
    #[inline]
    pub const fn LDRB_i_T1(
        imm5: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<3>,
        Rt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111u32 << 11u32
                | imm5.into_inner() << 6u32
                | Rn.into_inner() << 3u32
                | Rt.into_inner() << 0u32,
        )
    }
}
