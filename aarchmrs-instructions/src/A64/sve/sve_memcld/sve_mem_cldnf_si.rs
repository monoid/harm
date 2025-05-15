/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ldnf1b_z_p_bi_u8 {
    #[inline]
    pub fn ldnf1b_z_p_bi_u8(
        dtype: impl Into<::aarchmrs_types::BitValue<4>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(dtype.into()) << 21u32
                | 0b1u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnf1b_z_p_bi_u16 {
    #[inline]
    pub fn ldnf1b_z_p_bi_u16(
        dtype: impl Into<::aarchmrs_types::BitValue<4>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(dtype.into()) << 21u32
                | 0b1u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnf1b_z_p_bi_u32 {
    #[inline]
    pub fn ldnf1b_z_p_bi_u32(
        dtype: impl Into<::aarchmrs_types::BitValue<4>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(dtype.into()) << 21u32
                | 0b1u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnf1b_z_p_bi_u64 {
    #[inline]
    pub fn ldnf1b_z_p_bi_u64(
        dtype: impl Into<::aarchmrs_types::BitValue<4>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(dtype.into()) << 21u32
                | 0b1u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnf1sw_z_p_bi_s64 {
    #[inline]
    pub fn ldnf1sw_z_p_bi_s64(
        dtype: impl Into<::aarchmrs_types::BitValue<4>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(dtype.into()) << 21u32
                | 0b1u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnf1h_z_p_bi_u16 {
    #[inline]
    pub fn ldnf1h_z_p_bi_u16(
        dtype: impl Into<::aarchmrs_types::BitValue<4>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(dtype.into()) << 21u32
                | 0b1u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnf1h_z_p_bi_u32 {
    #[inline]
    pub fn ldnf1h_z_p_bi_u32(
        dtype: impl Into<::aarchmrs_types::BitValue<4>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(dtype.into()) << 21u32
                | 0b1u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnf1h_z_p_bi_u64 {
    #[inline]
    pub fn ldnf1h_z_p_bi_u64(
        dtype: impl Into<::aarchmrs_types::BitValue<4>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(dtype.into()) << 21u32
                | 0b1u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnf1sh_z_p_bi_s64 {
    #[inline]
    pub fn ldnf1sh_z_p_bi_s64(
        dtype: impl Into<::aarchmrs_types::BitValue<4>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(dtype.into()) << 21u32
                | 0b1u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnf1sh_z_p_bi_s32 {
    #[inline]
    pub fn ldnf1sh_z_p_bi_s32(
        dtype: impl Into<::aarchmrs_types::BitValue<4>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(dtype.into()) << 21u32
                | 0b1u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnf1w_z_p_bi_u32 {
    #[inline]
    pub fn ldnf1w_z_p_bi_u32(
        dtype: impl Into<::aarchmrs_types::BitValue<4>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(dtype.into()) << 21u32
                | 0b1u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnf1w_z_p_bi_u64 {
    #[inline]
    pub fn ldnf1w_z_p_bi_u64(
        dtype: impl Into<::aarchmrs_types::BitValue<4>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(dtype.into()) << 21u32
                | 0b1u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnf1sb_z_p_bi_s64 {
    #[inline]
    pub fn ldnf1sb_z_p_bi_s64(
        dtype: impl Into<::aarchmrs_types::BitValue<4>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(dtype.into()) << 21u32
                | 0b1u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnf1sb_z_p_bi_s32 {
    #[inline]
    pub fn ldnf1sb_z_p_bi_s32(
        dtype: impl Into<::aarchmrs_types::BitValue<4>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(dtype.into()) << 21u32
                | 0b1u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnf1sb_z_p_bi_s16 {
    #[inline]
    pub fn ldnf1sb_z_p_bi_s16(
        dtype: impl Into<::aarchmrs_types::BitValue<4>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(dtype.into()) << 21u32
                | 0b1u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnf1d_z_p_bi_u64 {
    #[inline]
    pub fn ldnf1d_z_p_bi_u64(
        dtype: impl Into<::aarchmrs_types::BitValue<4>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(dtype.into()) << 21u32
                | 0b1u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
