/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod addsvl_r_ri_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100001000000101100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "addsvl_r_ri_";
    #[inline]
    pub const fn addsvl_r_ri_(
        Rn: ::aarchmrs_types::BitValue<5>,
        imm6: ::aarchmrs_types::BitValue<6>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100001u32 << 21u32
                | Rn.into_inner() << 16u32
                | 0b01011u32 << 11u32
                | imm6.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod addspl_r_ri_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100011000000101100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "addspl_r_ri_";
    #[inline]
    pub const fn addspl_r_ri_(
        Rn: ::aarchmrs_types::BitValue<5>,
        imm6: ::aarchmrs_types::BitValue<6>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100011u32 << 21u32
                | Rn.into_inner() << 16u32
                | 0b01011u32 << 11u32
                | imm6.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
