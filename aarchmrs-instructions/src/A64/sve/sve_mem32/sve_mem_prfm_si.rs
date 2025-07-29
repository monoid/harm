/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod prfb_i_p_bi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000001110000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000101110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "prfb_i_p_bi_s";
    #[inline]
    pub const fn prfb_i_p_bi_s(
        imm6: ::aarchmrs_types::BitValue<6>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        prfop: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010111u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b000u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | prfop.into_inner() << 0u32,
        )
    }
}
pub mod prfh_i_p_bi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000001110000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000101110000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "prfh_i_p_bi_s";
    #[inline]
    pub const fn prfh_i_p_bi_s(
        imm6: ::aarchmrs_types::BitValue<6>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        prfop: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010111u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b001u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | prfop.into_inner() << 0u32,
        )
    }
}
pub mod prfw_i_p_bi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000001110000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000101110000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "prfw_i_p_bi_s";
    #[inline]
    pub const fn prfw_i_p_bi_s(
        imm6: ::aarchmrs_types::BitValue<6>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        prfop: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010111u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b010u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | prfop.into_inner() << 0u32,
        )
    }
}
pub mod prfd_i_p_bi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000001110000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000101110000000110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "prfd_i_p_bi_s";
    #[inline]
    pub const fn prfd_i_p_bi_s(
        imm6: ::aarchmrs_types::BitValue<6>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        prfop: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010111u32 << 22u32
                | imm6.into_inner() << 16u32
                | 0b011u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | prfop.into_inner() << 0u32,
        )
    }
}
