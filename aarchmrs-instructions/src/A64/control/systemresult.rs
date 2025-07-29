/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod TSTART_BR_systemresult {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101001000110011000001100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TSTART_BR_systemresult";
    #[inline]
    pub const fn TSTART_BR_systemresult(
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110101010010001100110000011u32 << 5u32 | Rt.into_inner() << 0u32,
        )
    }
}
pub mod TTEST_BR_systemresult {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101001000110011000101100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TTEST_BR_systemresult";
    #[inline]
    pub const fn TTEST_BR_systemresult(
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110101010010001100110001011u32 << 5u32 | Rt.into_inner() << 0u32,
        )
    }
}
