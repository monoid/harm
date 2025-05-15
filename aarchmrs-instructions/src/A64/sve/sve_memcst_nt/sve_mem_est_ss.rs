/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod st2b_z_p_br_contiguous {
    #[inline]
    pub fn st2b_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | 0b01u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod st3b_z_p_br_contiguous {
    #[inline]
    pub fn st3b_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | 0b10u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod st4b_z_p_br_contiguous {
    #[inline]
    pub fn st4b_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | 0b11u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod st2h_z_p_br_contiguous {
    #[inline]
    pub fn st2h_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | 0b01u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod st3h_z_p_br_contiguous {
    #[inline]
    pub fn st3h_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | 0b10u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod st4h_z_p_br_contiguous {
    #[inline]
    pub fn st4h_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | 0b11u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod st2w_z_p_br_contiguous {
    #[inline]
    pub fn st2w_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | 0b01u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod st3w_z_p_br_contiguous {
    #[inline]
    pub fn st3w_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | 0b10u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod st4w_z_p_br_contiguous {
    #[inline]
    pub fn st4w_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | 0b11u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod st2d_z_p_br_contiguous {
    #[inline]
    pub fn st2d_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | 0b01u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod st3d_z_p_br_contiguous {
    #[inline]
    pub fn st3d_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | 0b10u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod st4d_z_p_br_contiguous {
    #[inline]
    pub fn st4d_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | 0b11u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
