/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADD_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADD_r_T2";
    #[inline]
    pub const fn ADD_r_T2(
        DN: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rdn: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100u32 << 8u32
                | DN.into_inner() << 7u32
                | Rm.into_inner() << 3u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod ADD_SP_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111101111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100010001101000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADD_SP_r_T1";
    #[inline]
    pub const fn ADD_SP_r_T1(
        DM: ::aarchmrs_types::BitValue<1>,
        Rdm: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100u32 << 8u32
                | DM.into_inner() << 7u32
                | 0b1101u32 << 3u32
                | Rdm.into_inner() << 0u32,
        )
    }
}
pub mod ADD_SP_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111110000111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100010010000101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADD_SP_r_T2";
    #[inline]
    pub const fn ADD_SP_r_T2(
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001001u32 << 7u32 | Rm.into_inner() << 3u32 | 0b101u32 << 0u32,
        )
    }
}
pub mod CMP_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100010100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMP_r_T2";
    #[inline]
    pub const fn CMP_r_T2(
        N: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101u32 << 8u32
                | N.into_inner() << 7u32
                | Rm.into_inner() << 3u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod MOV_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100011000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOV_r_T1";
    #[inline]
    pub const fn MOV_r_T1(
        D: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000110u32 << 8u32
                | D.into_inner() << 7u32
                | Rm.into_inner() << 3u32
                | Rd.into_inner() << 0u32,
        )
    }
}
