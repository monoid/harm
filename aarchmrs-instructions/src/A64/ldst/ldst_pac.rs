/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod LDRAA_64_ldst_pac {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000001000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRAA_64_ldst_pac";
    #[inline]
    pub const fn LDRAA_64_ldst_pac(
        S: ::aarchmrs_types::BitValue<1>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110000u32 << 23u32
                | S.into_inner() << 22u32
                | 0b1u32 << 21u32
                | imm9.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDRAA_64W_ldst_pac {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000001000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRAA_64W_ldst_pac";
    #[inline]
    pub const fn LDRAA_64W_ldst_pac(
        S: ::aarchmrs_types::BitValue<1>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110000u32 << 23u32
                | S.into_inner() << 22u32
                | 0b1u32 << 21u32
                | imm9.into_inner() << 12u32
                | 0b11u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDRAB_64_ldst_pac {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000101000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRAB_64_ldst_pac";
    #[inline]
    pub const fn LDRAB_64_ldst_pac(
        S: ::aarchmrs_types::BitValue<1>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110001u32 << 23u32
                | S.into_inner() << 22u32
                | 0b1u32 << 21u32
                | imm9.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDRAB_64W_ldst_pac {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000101000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRAB_64W_ldst_pac";
    #[inline]
    pub const fn LDRAB_64W_ldst_pac(
        S: ::aarchmrs_types::BitValue<1>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110001u32 << 23u32
                | S.into_inner() << 22u32
                | 0b1u32 << 21u32
                | imm9.into_inner() << 12u32
                | 0b11u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
