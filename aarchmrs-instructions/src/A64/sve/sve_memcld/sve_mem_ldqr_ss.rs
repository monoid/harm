/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1rqb_z_p_br_contiguous {
    #[inline]
    pub fn ld1rqb_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        ssz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | u32::from(ssz.into()) << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rob_z_p_br_contiguous {
    #[inline]
    pub fn ld1rob_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        ssz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | u32::from(ssz.into()) << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rqh_z_p_br_contiguous {
    #[inline]
    pub fn ld1rqh_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        ssz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | u32::from(ssz.into()) << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1roh_z_p_br_contiguous {
    #[inline]
    pub fn ld1roh_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        ssz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | u32::from(ssz.into()) << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rqw_z_p_br_contiguous {
    #[inline]
    pub fn ld1rqw_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        ssz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | u32::from(ssz.into()) << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1row_z_p_br_contiguous {
    #[inline]
    pub fn ld1row_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        ssz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | u32::from(ssz.into()) << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rqd_z_p_br_contiguous {
    #[inline]
    pub fn ld1rqd_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        ssz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | u32::from(ssz.into()) << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1rod_z_p_br_contiguous {
    #[inline]
    pub fn ld1rod_z_p_br_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        ssz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | u32::from(ssz.into()) << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
