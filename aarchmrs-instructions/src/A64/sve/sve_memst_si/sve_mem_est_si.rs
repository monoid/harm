/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod st2b_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100100001100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st2b_z_p_bi_contiguous";
    #[inline]
    pub const fn st2b_z_p_bi_contiguous(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111001000011u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod st3b_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100100010100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st3b_z_p_bi_contiguous";
    #[inline]
    pub const fn st3b_z_p_bi_contiguous(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111001000101u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod st4b_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100100011100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st4b_z_p_bi_contiguous";
    #[inline]
    pub const fn st4b_z_p_bi_contiguous(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111001000111u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod st2h_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100100101100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st2h_z_p_bi_contiguous";
    #[inline]
    pub const fn st2h_z_p_bi_contiguous(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111001001011u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod st3h_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100100110100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st3h_z_p_bi_contiguous";
    #[inline]
    pub const fn st3h_z_p_bi_contiguous(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111001001101u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod st4h_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100100111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st4h_z_p_bi_contiguous";
    #[inline]
    pub const fn st4h_z_p_bi_contiguous(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111001001111u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod st2w_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100101001100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st2w_z_p_bi_contiguous";
    #[inline]
    pub const fn st2w_z_p_bi_contiguous(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111001010011u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod st3w_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100101010100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st3w_z_p_bi_contiguous";
    #[inline]
    pub const fn st3w_z_p_bi_contiguous(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111001010101u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod st4w_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100101011100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st4w_z_p_bi_contiguous";
    #[inline]
    pub const fn st4w_z_p_bi_contiguous(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111001010111u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod st2d_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100101101100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st2d_z_p_bi_contiguous";
    #[inline]
    pub const fn st2d_z_p_bi_contiguous(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111001011011u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod st3d_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100101110100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st3d_z_p_bi_contiguous";
    #[inline]
    pub const fn st3d_z_p_bi_contiguous(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111001011101u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod st4d_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100101111100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "st4d_z_p_bi_contiguous";
    #[inline]
    pub const fn st4d_z_p_bi_contiguous(
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111001011111u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
