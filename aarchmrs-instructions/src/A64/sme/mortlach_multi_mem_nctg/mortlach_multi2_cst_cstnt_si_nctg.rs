/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod st1b_mzx_p_bi_2x8 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000001000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st1b_mzx_p_bi_2x8";
    #[inline]
    pub const fn st1b_mzx_p_bi_2x8(
        imm4: ::aarchmrs_types::BitValue<4>,
        msz: ::aarchmrs_types::BitValue<2>,
        PNg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        T: ::aarchmrs_types::BitValue<1>,
        Zt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000010110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b0u32 << 15u32
                | msz.into_inner() << 13u32
                | PNg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | T.into_inner() << 4u32
                | 0b0u32 << 3u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod stnt1b_mzx_p_bi_2x8 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000001000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001011000000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "stnt1b_mzx_p_bi_2x8";
    #[inline]
    pub const fn stnt1b_mzx_p_bi_2x8(
        imm4: ::aarchmrs_types::BitValue<4>,
        msz: ::aarchmrs_types::BitValue<2>,
        PNg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        T: ::aarchmrs_types::BitValue<1>,
        Zt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000010110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b0u32 << 15u32
                | msz.into_inner() << 13u32
                | PNg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | T.into_inner() << 4u32
                | 0b1u32 << 3u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod st1h_mzx_p_bi_2x8 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000001000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st1h_mzx_p_bi_2x8";
    #[inline]
    pub const fn st1h_mzx_p_bi_2x8(
        imm4: ::aarchmrs_types::BitValue<4>,
        msz: ::aarchmrs_types::BitValue<2>,
        PNg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        T: ::aarchmrs_types::BitValue<1>,
        Zt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000010110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b0u32 << 15u32
                | msz.into_inner() << 13u32
                | PNg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | T.into_inner() << 4u32
                | 0b0u32 << 3u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod stnt1h_mzx_p_bi_2x8 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000001000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001011000000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "stnt1h_mzx_p_bi_2x8";
    #[inline]
    pub const fn stnt1h_mzx_p_bi_2x8(
        imm4: ::aarchmrs_types::BitValue<4>,
        msz: ::aarchmrs_types::BitValue<2>,
        PNg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        T: ::aarchmrs_types::BitValue<1>,
        Zt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000010110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b0u32 << 15u32
                | msz.into_inner() << 13u32
                | PNg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | T.into_inner() << 4u32
                | 0b1u32 << 3u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod st1w_mzx_p_bi_2x8 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000001000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st1w_mzx_p_bi_2x8";
    #[inline]
    pub const fn st1w_mzx_p_bi_2x8(
        imm4: ::aarchmrs_types::BitValue<4>,
        msz: ::aarchmrs_types::BitValue<2>,
        PNg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        T: ::aarchmrs_types::BitValue<1>,
        Zt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000010110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b0u32 << 15u32
                | msz.into_inner() << 13u32
                | PNg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | T.into_inner() << 4u32
                | 0b0u32 << 3u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod stnt1w_mzx_p_bi_2x8 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000001000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001011000000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "stnt1w_mzx_p_bi_2x8";
    #[inline]
    pub const fn stnt1w_mzx_p_bi_2x8(
        imm4: ::aarchmrs_types::BitValue<4>,
        msz: ::aarchmrs_types::BitValue<2>,
        PNg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        T: ::aarchmrs_types::BitValue<1>,
        Zt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000010110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b0u32 << 15u32
                | msz.into_inner() << 13u32
                | PNg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | T.into_inner() << 4u32
                | 0b1u32 << 3u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod st1d_mzx_p_bi_2x8 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000001000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st1d_mzx_p_bi_2x8";
    #[inline]
    pub const fn st1d_mzx_p_bi_2x8(
        imm4: ::aarchmrs_types::BitValue<4>,
        msz: ::aarchmrs_types::BitValue<2>,
        PNg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        T: ::aarchmrs_types::BitValue<1>,
        Zt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000010110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b0u32 << 15u32
                | msz.into_inner() << 13u32
                | PNg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | T.into_inner() << 4u32
                | 0b0u32 << 3u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod stnt1d_mzx_p_bi_2x8 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000001000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100001011000000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "stnt1d_mzx_p_bi_2x8";
    #[inline]
    pub const fn stnt1d_mzx_p_bi_2x8(
        imm4: ::aarchmrs_types::BitValue<4>,
        msz: ::aarchmrs_types::BitValue<2>,
        PNg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        T: ::aarchmrs_types::BitValue<1>,
        Zt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000010110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b0u32 << 15u32
                | msz.into_inner() << 13u32
                | PNg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | T.into_inner() << 4u32
                | 0b1u32 << 3u32
                | Zt.into_inner() << 0u32,
        )
    }
}
