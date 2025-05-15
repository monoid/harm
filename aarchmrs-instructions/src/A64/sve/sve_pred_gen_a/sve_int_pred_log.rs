/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod and_p_p_pp_z {
    #[inline]
    pub fn and_p_p_pp_z(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b00u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod bic_p_p_pp_z {
    #[inline]
    pub fn bic_p_p_pp_z(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b00u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod orr_p_p_pp_z {
    #[inline]
    pub fn orr_p_p_pp_z(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001011u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b00u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod orn_p_p_pp_z {
    #[inline]
    pub fn orn_p_p_pp_z(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001011u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b00u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod eor_p_p_pp_z {
    #[inline]
    pub fn eor_p_p_pp_z(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b00u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b1u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod sel_p_p_pp_ {
    #[inline]
    pub fn sel_p_p_pp_(
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010000u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b1u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod nor_p_p_pp_z {
    #[inline]
    pub fn nor_p_p_pp_z(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001011u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b00u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b1u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod nand_p_p_pp_z {
    #[inline]
    pub fn nand_p_p_pp_z(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001011u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b00u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b1u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod ands_p_p_pp_z {
    #[inline]
    pub fn ands_p_p_pp_z(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b00u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod bics_p_p_pp_z {
    #[inline]
    pub fn bics_p_p_pp_z(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b00u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod orrs_p_p_pp_z {
    #[inline]
    pub fn orrs_p_p_pp_z(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001011u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b00u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod orns_p_p_pp_z {
    #[inline]
    pub fn orns_p_p_pp_z(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001011u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b00u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b0u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod eors_p_p_pp_z {
    #[inline]
    pub fn eors_p_p_pp_z(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001010u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b00u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b1u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod nors_p_p_pp_z {
    #[inline]
    pub fn nors_p_p_pp_z(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001011u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b00u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b1u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
pub mod nands_p_p_pp_z {
    #[inline]
    pub fn nands_p_p_pp_z(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Pm: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<4>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Pd: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001001011u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b00u32 << 20u32
                | u32::from(Pm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(Pg.into()) << 10u32
                | 0b1u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(Pd.into()) << 0u32,
        )
    }
}
