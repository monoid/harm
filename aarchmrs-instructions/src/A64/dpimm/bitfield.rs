/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SBFM_32M_bitfield {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00010011000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SBFM_32M_bitfield";
    #[inline]
    pub const fn SBFM_32M_bitfield(
        immr: ::aarchmrs_types::BitValue<6>,
        imms: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001001100u32 << 22u32
                | immr.into_inner() << 16u32
                | imms.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod BFM_32M_bitfield {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00110011000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BFM_32M_bitfield";
    #[inline]
    pub const fn BFM_32M_bitfield(
        immr: ::aarchmrs_types::BitValue<6>,
        imms: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0011001100u32 << 22u32
                | immr.into_inner() << 16u32
                | imms.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UBFM_32M_bitfield {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01010011000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UBFM_32M_bitfield";
    #[inline]
    pub const fn UBFM_32M_bitfield(
        immr: ::aarchmrs_types::BitValue<6>,
        imms: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101001100u32 << 22u32
                | immr.into_inner() << 16u32
                | imms.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SBFM_64M_bitfield {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10010011010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SBFM_64M_bitfield";
    #[inline]
    pub const fn SBFM_64M_bitfield(
        immr: ::aarchmrs_types::BitValue<6>,
        imms: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001001101u32 << 22u32
                | immr.into_inner() << 16u32
                | imms.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod BFM_64M_bitfield {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10110011010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BFM_64M_bitfield";
    #[inline]
    pub const fn BFM_64M_bitfield(
        immr: ::aarchmrs_types::BitValue<6>,
        imms: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1011001101u32 << 22u32
                | immr.into_inner() << 16u32
                | imms.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UBFM_64M_bitfield {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010011010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UBFM_64M_bitfield";
    #[inline]
    pub const fn UBFM_64M_bitfield(
        immr: ::aarchmrs_types::BitValue<6>,
        imms: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101001101u32 << 22u32
                | immr.into_inner() << 16u32
                | imms.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
