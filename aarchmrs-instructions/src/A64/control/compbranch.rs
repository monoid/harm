/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CBZ_32_compbranch {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00110100000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBZ_32_compbranch";
    #[inline]
    pub const fn CBZ_32_compbranch(
        imm19: ::aarchmrs_types::BitValue<19>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00110100u32 << 24u32 | imm19.into_inner() << 5u32 | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBNZ_32_compbranch {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00110101000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBNZ_32_compbranch";
    #[inline]
    pub const fn CBNZ_32_compbranch(
        imm19: ::aarchmrs_types::BitValue<19>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00110101u32 << 24u32 | imm19.into_inner() << 5u32 | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBZ_64_compbranch {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10110100000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBZ_64_compbranch";
    #[inline]
    pub const fn CBZ_64_compbranch(
        imm19: ::aarchmrs_types::BitValue<19>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10110100u32 << 24u32 | imm19.into_inner() << 5u32 | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBNZ_64_compbranch {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10110101000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBNZ_64_compbranch";
    #[inline]
    pub const fn CBNZ_64_compbranch(
        imm19: ::aarchmrs_types::BitValue<19>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10110101u32 << 24u32 | imm19.into_inner() << 5u32 | Rt.into_inner() << 0u32,
        )
    }
}
