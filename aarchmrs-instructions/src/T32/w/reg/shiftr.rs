/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MOVS_rr_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010000100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOVS_rr_T2";
    #[inline]
    pub const fn MOVS_rr_T2(
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110100u32 << 23u32
                | stype.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rm.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0000u32 << 4u32
                | Rs.into_inner() << 0u32,
        )
    }
}
pub mod MOV_rr_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOV_rr_T2";
    #[inline]
    pub const fn MOV_rr_T2(
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110100u32 << 23u32
                | stype.into_inner() << 21u32
                | 0b0u32 << 20u32
                | Rm.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0000u32 << 4u32
                | Rs.into_inner() << 0u32,
        )
    }
}
