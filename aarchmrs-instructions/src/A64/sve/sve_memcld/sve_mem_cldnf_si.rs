/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ldnf1b_z_p_bi_u8 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100000100001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnf1b_z_p_bi_u8";
    #[inline]
    pub const fn ldnf1b_z_p_bi_u8(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101001000001u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldnf1b_z_p_bi_u16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100001100001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnf1b_z_p_bi_u16";
    #[inline]
    pub const fn ldnf1b_z_p_bi_u16(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101001000011u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldnf1b_z_p_bi_u32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100010100001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnf1b_z_p_bi_u32";
    #[inline]
    pub const fn ldnf1b_z_p_bi_u32(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101001000101u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldnf1b_z_p_bi_u64 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100011100001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnf1b_z_p_bi_u64";
    #[inline]
    pub const fn ldnf1b_z_p_bi_u64(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101001000111u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldnf1sw_z_p_bi_s64 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100100100001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnf1sw_z_p_bi_s64";
    #[inline]
    pub const fn ldnf1sw_z_p_bi_s64(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101001001001u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldnf1h_z_p_bi_u16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100101100001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnf1h_z_p_bi_u16";
    #[inline]
    pub const fn ldnf1h_z_p_bi_u16(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101001001011u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldnf1h_z_p_bi_u32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100110100001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnf1h_z_p_bi_u32";
    #[inline]
    pub const fn ldnf1h_z_p_bi_u32(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101001001101u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldnf1h_z_p_bi_u64 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100111100001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnf1h_z_p_bi_u64";
    #[inline]
    pub const fn ldnf1h_z_p_bi_u64(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101001001111u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldnf1sh_z_p_bi_s64 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100101000100001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnf1sh_z_p_bi_s64";
    #[inline]
    pub const fn ldnf1sh_z_p_bi_s64(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101001010001u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldnf1sh_z_p_bi_s32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100101001100001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnf1sh_z_p_bi_s32";
    #[inline]
    pub const fn ldnf1sh_z_p_bi_s32(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101001010011u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldnf1w_z_p_bi_u32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100101010100001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnf1w_z_p_bi_u32";
    #[inline]
    pub const fn ldnf1w_z_p_bi_u32(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101001010101u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldnf1w_z_p_bi_u64 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100101011100001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnf1w_z_p_bi_u64";
    #[inline]
    pub const fn ldnf1w_z_p_bi_u64(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101001010111u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldnf1sb_z_p_bi_s64 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100101100100001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnf1sb_z_p_bi_s64";
    #[inline]
    pub const fn ldnf1sb_z_p_bi_s64(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101001011001u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldnf1sb_z_p_bi_s32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100101101100001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnf1sb_z_p_bi_s32";
    #[inline]
    pub const fn ldnf1sb_z_p_bi_s32(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101001011011u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldnf1sb_z_p_bi_s16 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100101110100001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnf1sb_z_p_bi_s16";
    #[inline]
    pub const fn ldnf1sb_z_p_bi_s16(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101001011101u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ldnf1d_z_p_bi_u64 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100101111100001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnf1d_z_p_bi_u64";
    #[inline]
    pub const fn ldnf1d_z_p_bi_u64(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101001011111u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
