/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld2b_z_p_br_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld2b_z_p_br_contiguous";
    #[inline]
    pub const fn ld2b_z_p_br_contiguous(
        Rm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100100001u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b110u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld3b_z_p_br_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100010000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld3b_z_p_br_contiguous";
    #[inline]
    pub const fn ld3b_z_p_br_contiguous(
        Rm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100100010u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b110u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld4b_z_p_br_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100011000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld4b_z_p_br_contiguous";
    #[inline]
    pub const fn ld4b_z_p_br_contiguous(
        Rm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100100011u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b110u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld2h_z_p_br_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100101000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld2h_z_p_br_contiguous";
    #[inline]
    pub const fn ld2h_z_p_br_contiguous(
        Rm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100100101u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b110u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld3h_z_p_br_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100110000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld3h_z_p_br_contiguous";
    #[inline]
    pub const fn ld3h_z_p_br_contiguous(
        Rm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100100110u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b110u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld4h_z_p_br_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100100111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld4h_z_p_br_contiguous";
    #[inline]
    pub const fn ld4h_z_p_br_contiguous(
        Rm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100100111u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b110u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld2w_z_p_br_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100101001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld2w_z_p_br_contiguous";
    #[inline]
    pub const fn ld2w_z_p_br_contiguous(
        Rm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100101001u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b110u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld3w_z_p_br_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100101010000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld3w_z_p_br_contiguous";
    #[inline]
    pub const fn ld3w_z_p_br_contiguous(
        Rm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100101010u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b110u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld4w_z_p_br_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100101011000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld4w_z_p_br_contiguous";
    #[inline]
    pub const fn ld4w_z_p_br_contiguous(
        Rm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100101011u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b110u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld2d_z_p_br_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100101101000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld2d_z_p_br_contiguous";
    #[inline]
    pub const fn ld2d_z_p_br_contiguous(
        Rm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100101101u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b110u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld3d_z_p_br_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100101110000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld3d_z_p_br_contiguous";
    #[inline]
    pub const fn ld3d_z_p_br_contiguous(
        Rm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100101110u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b110u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
pub mod ld4d_z_p_br_contiguous {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100101111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld4d_z_p_br_contiguous";
    #[inline]
    pub const fn ld4d_z_p_br_contiguous(
        Rm: ::aarchmrs_types::BitValue<5>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100101111u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b110u32 << 13u32
                | Pg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 0u32,
        )
    }
}
