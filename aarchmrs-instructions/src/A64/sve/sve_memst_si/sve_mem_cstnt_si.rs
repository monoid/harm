/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod stnt1b_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100100000100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "stnt1b_z_p_bi_contiguous";
    #[inline]
    pub const fn stnt1b_z_p_bi_contiguous(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111001000001u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod stnt1h_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100100100100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "stnt1h_z_p_bi_contiguous";
    #[inline]
    pub const fn stnt1h_z_p_bi_contiguous(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111001001001u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod stnt1w_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100101000100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "stnt1w_z_p_bi_contiguous";
    #[inline]
    pub const fn stnt1w_z_p_bi_contiguous(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111001010001u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod stnt1d_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100101100100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "stnt1d_z_p_bi_contiguous";
    #[inline]
    pub const fn stnt1d_z_p_bi_contiguous(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111001011001u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
