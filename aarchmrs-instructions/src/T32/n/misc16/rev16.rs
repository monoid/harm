/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod REV_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001011101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "REV_T1";
    #[inline]
    pub const fn REV_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1011101000u32 << 6u32 | Rm.into_inner() << 3u32 | Rd.into_inner() << 0u32,
        )
    }
}
pub mod REV16_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001011101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "REV16_T1";
    #[inline]
    pub const fn REV16_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1011101001u32 << 6u32 | Rm.into_inner() << 3u32 | Rd.into_inner() << 0u32,
        )
    }
}
pub mod REVSH_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001011101011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "REVSH_T1";
    #[inline]
    pub const fn REVSH_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1011101011u32 << 6u32 | Rm.into_inner() << 3u32 | Rd.into_inner() << 0u32,
        )
    }
}
