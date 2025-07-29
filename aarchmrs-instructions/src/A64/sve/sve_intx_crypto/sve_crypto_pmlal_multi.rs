/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod pmlal_mz_zzzw_1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "pmlal_mz_zzzw_1x2";
    #[inline]
    pub const fn pmlal_mz_zzzw_1x2(
        Zm: ::aarchmrs_types::BitValue<5>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101001u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b111111u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
