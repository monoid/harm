/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1rb_z_p_bi_u8 {
    #[inline]
    pub fn ld1rb_z_p_bi_u8(
        dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | u32::from(dtypeh.into()) << 23u32
                | 0b1u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(dtypel.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rb_z_p_bi_u16 {
    #[inline]
    pub fn ld1rb_z_p_bi_u16(
        dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | u32::from(dtypeh.into()) << 23u32
                | 0b1u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(dtypel.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rb_z_p_bi_u32 {
    #[inline]
    pub fn ld1rb_z_p_bi_u32(
        dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | u32::from(dtypeh.into()) << 23u32
                | 0b1u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(dtypel.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rb_z_p_bi_u64 {
    #[inline]
    pub fn ld1rb_z_p_bi_u64(
        dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | u32::from(dtypeh.into()) << 23u32
                | 0b1u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(dtypel.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rsw_z_p_bi_s64 {
    #[inline]
    pub fn ld1rsw_z_p_bi_s64(
        dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | u32::from(dtypeh.into()) << 23u32
                | 0b1u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(dtypel.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rh_z_p_bi_u16 {
    #[inline]
    pub fn ld1rh_z_p_bi_u16(
        dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | u32::from(dtypeh.into()) << 23u32
                | 0b1u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(dtypel.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rh_z_p_bi_u32 {
    #[inline]
    pub fn ld1rh_z_p_bi_u32(
        dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | u32::from(dtypeh.into()) << 23u32
                | 0b1u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(dtypel.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rh_z_p_bi_u64 {
    #[inline]
    pub fn ld1rh_z_p_bi_u64(
        dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | u32::from(dtypeh.into()) << 23u32
                | 0b1u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(dtypel.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rsh_z_p_bi_s64 {
    #[inline]
    pub fn ld1rsh_z_p_bi_s64(
        dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | u32::from(dtypeh.into()) << 23u32
                | 0b1u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(dtypel.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rsh_z_p_bi_s32 {
    #[inline]
    pub fn ld1rsh_z_p_bi_s32(
        dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | u32::from(dtypeh.into()) << 23u32
                | 0b1u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(dtypel.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rw_z_p_bi_u32 {
    #[inline]
    pub fn ld1rw_z_p_bi_u32(
        dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | u32::from(dtypeh.into()) << 23u32
                | 0b1u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(dtypel.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rw_z_p_bi_u64 {
    #[inline]
    pub fn ld1rw_z_p_bi_u64(
        dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | u32::from(dtypeh.into()) << 23u32
                | 0b1u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(dtypel.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rsb_z_p_bi_s64 {
    #[inline]
    pub fn ld1rsb_z_p_bi_s64(
        dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | u32::from(dtypeh.into()) << 23u32
                | 0b1u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(dtypel.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rsb_z_p_bi_s32 {
    #[inline]
    pub fn ld1rsb_z_p_bi_s32(
        dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | u32::from(dtypeh.into()) << 23u32
                | 0b1u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(dtypel.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rsb_z_p_bi_s16 {
    #[inline]
    pub fn ld1rsb_z_p_bi_s16(
        dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | u32::from(dtypeh.into()) << 23u32
                | 0b1u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(dtypel.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rd_z_p_bi_u64 {
    #[inline]
    pub fn ld1rd_z_p_bi_u64(
        dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010u32 << 25u32
                | u32::from(dtypeh.into()) << 23u32
                | 0b1u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(dtypel.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
