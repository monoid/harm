/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld2b_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110011100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100001000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld2b_z_p_bi_contiguous";
    #[inline]
    pub const fn ld2b_z_p_bi_contiguous(
        msz: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | msz.into_inner() << 23u32
                | 0b010u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld3b_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110011100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100010000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld3b_z_p_bi_contiguous";
    #[inline]
    pub const fn ld3b_z_p_bi_contiguous(
        msz: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | msz.into_inner() << 23u32
                | 0b100u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld4b_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110011100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100011000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld4b_z_p_bi_contiguous";
    #[inline]
    pub const fn ld4b_z_p_bi_contiguous(
        msz: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | msz.into_inner() << 23u32
                | 0b110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld2h_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110011100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100001000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld2h_z_p_bi_contiguous";
    #[inline]
    pub const fn ld2h_z_p_bi_contiguous(
        msz: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | msz.into_inner() << 23u32
                | 0b010u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld3h_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110011100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100010000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld3h_z_p_bi_contiguous";
    #[inline]
    pub const fn ld3h_z_p_bi_contiguous(
        msz: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | msz.into_inner() << 23u32
                | 0b100u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld4h_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110011100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100011000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld4h_z_p_bi_contiguous";
    #[inline]
    pub const fn ld4h_z_p_bi_contiguous(
        msz: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | msz.into_inner() << 23u32
                | 0b110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld2w_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110011100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100001000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld2w_z_p_bi_contiguous";
    #[inline]
    pub const fn ld2w_z_p_bi_contiguous(
        msz: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | msz.into_inner() << 23u32
                | 0b010u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld3w_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110011100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100010000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld3w_z_p_bi_contiguous";
    #[inline]
    pub const fn ld3w_z_p_bi_contiguous(
        msz: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | msz.into_inner() << 23u32
                | 0b100u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld4w_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110011100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100011000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld4w_z_p_bi_contiguous";
    #[inline]
    pub const fn ld4w_z_p_bi_contiguous(
        msz: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | msz.into_inner() << 23u32
                | 0b110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld2d_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110011100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100001000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld2d_z_p_bi_contiguous";
    #[inline]
    pub const fn ld2d_z_p_bi_contiguous(
        msz: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | msz.into_inner() << 23u32
                | 0b010u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld3d_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110011100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100010000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld3d_z_p_bi_contiguous";
    #[inline]
    pub const fn ld3d_z_p_bi_contiguous(
        msz: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | msz.into_inner() << 23u32
                | 0b100u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld4d_z_p_bi_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110011100001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100011000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld4d_z_p_bi_contiguous";
    #[inline]
    pub const fn ld4d_z_p_bi_contiguous(
        msz: ::aarchmrs_types::BitValue<2>,
        imm4: ::aarchmrs_types::BitValue<4>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | msz.into_inner() << 23u32
                | 0b110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b111u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
