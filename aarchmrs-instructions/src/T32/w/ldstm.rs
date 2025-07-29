/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SRS_T1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110111111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000000011011100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SRS_T1_AS";
    #[inline]
    pub const fn SRS_T1_AS(
        W: ::aarchmrs_types::BitValue<1>,
        mode: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110100000u32 << 22u32
                | W.into_inner() << 21u32
                | 0b0110111000000000u32 << 5u32
                | mode.into_inner() << 0u32,
        )
    }
}
pub mod RFE_T1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110100001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000000100001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RFE_T1_AS";
    #[inline]
    pub const fn RFE_T1_AS(
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110100000u32 << 22u32
                | W.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1100000000000000u32 << 0u32,
        )
    }
}
pub mod STM_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STM_T2";
    #[inline]
    pub const fn STM_T2(
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        register_list: ::aarchmrs_types::BitValue<14>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110100010u32 << 22u32
                | W.into_inner() << 21u32
                | 0b0u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | M.into_inner() << 14u32
                | register_list.into_inner() << 0u32,
        )
    }
}
pub mod LDM_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000100100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDM_T2";
    #[inline]
    pub const fn LDM_T2(
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        P: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        register_list: ::aarchmrs_types::BitValue<14>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110100010u32 << 22u32
                | W.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rn.into_inner() << 16u32
                | P.into_inner() << 15u32
                | M.into_inner() << 14u32
                | register_list.into_inner() << 0u32,
        )
    }
}
pub mod STMDB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101001000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STMDB_T1";
    #[inline]
    pub const fn STMDB_T1(
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        register_list: ::aarchmrs_types::BitValue<14>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110100100u32 << 22u32
                | W.into_inner() << 21u32
                | 0b0u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | M.into_inner() << 14u32
                | register_list.into_inner() << 0u32,
        )
    }
}
pub mod LDMDB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101001000100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDMDB_T1";
    #[inline]
    pub const fn LDMDB_T1(
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        P: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        register_list: ::aarchmrs_types::BitValue<14>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110100100u32 << 22u32
                | W.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rn.into_inner() << 16u32
                | P.into_inner() << 15u32
                | M.into_inner() << 14u32
                | register_list.into_inner() << 0u32,
        )
    }
}
pub mod SRS_T2_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110111111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101001100011011100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SRS_T2_AS";
    #[inline]
    pub const fn SRS_T2_AS(
        W: ::aarchmrs_types::BitValue<1>,
        mode: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110100110u32 << 22u32
                | W.into_inner() << 21u32
                | 0b0110111000000000u32 << 5u32
                | mode.into_inner() << 0u32,
        )
    }
}
pub mod RFE_T2_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110100001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101001100100001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RFE_T2_AS";
    #[inline]
    pub const fn RFE_T2_AS(
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110100110u32 << 22u32
                | W.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1100000000000000u32 << 0u32,
        )
    }
}
