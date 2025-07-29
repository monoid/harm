/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SETF8_only_setf {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00111010000000000000100000001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETF8_only_setf";
    #[inline]
    pub const fn SETF8_only_setf(
        Rn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0011101000000000000010u32 << 10u32 | Rn.into_inner() << 5u32 | 0b01101u32 << 0u32,
        )
    }
}
pub mod SETF16_only_setf {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00111010000000000100100000001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETF16_only_setf";
    #[inline]
    pub const fn SETF16_only_setf(
        Rn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0011101000000000010010u32 << 10u32 | Rn.into_inner() << 5u32 | 0b01101u32 << 0u32,
        )
    }
}
