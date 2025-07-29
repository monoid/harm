/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADDG_64_addsub_immtags {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10010001100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADDG_64_addsub_immtags";
    #[inline]
    pub const fn ADDG_64_addsub_immtags(
        imm6: ::aarchmrs_types::BitValue<6>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001000110u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b00u32 << 14u32
                | imm4.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SUBG_64_addsub_immtags {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010001100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUBG_64_addsub_immtags";
    #[inline]
    pub const fn SUBG_64_addsub_immtags(
        imm6: ::aarchmrs_types::BitValue<6>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101000110u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b00u32 << 14u32
                | imm4.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
