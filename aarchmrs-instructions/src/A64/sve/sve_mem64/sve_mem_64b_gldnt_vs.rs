/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ldnt1sb_z_p_ar_d_64_unscaled {
    #[inline]
    pub fn ldnt1sb_z_p_ar_d_64_unscaled(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000100000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(U.into()) << 14u32
                | 0b0u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnt1sh_z_p_ar_d_64_unscaled {
    #[inline]
    pub fn ldnt1sh_z_p_ar_d_64_unscaled(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000100100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(U.into()) << 14u32
                | 0b0u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnt1sw_z_p_ar_d_64_unscaled {
    #[inline]
    pub fn ldnt1sw_z_p_ar_d_64_unscaled(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000101000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(U.into()) << 14u32
                | 0b0u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnt1d_z_p_ar_d_64_unscaled {
    #[inline]
    pub fn ldnt1d_z_p_ar_d_64_unscaled(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000101100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b110u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnt1b_z_p_ar_d_64_unscaled {
    #[inline]
    pub fn ldnt1b_z_p_ar_d_64_unscaled(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000100000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(U.into()) << 14u32
                | 0b0u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnt1h_z_p_ar_d_64_unscaled {
    #[inline]
    pub fn ldnt1h_z_p_ar_d_64_unscaled(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000100100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(U.into()) << 14u32
                | 0b0u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnt1w_z_p_ar_d_64_unscaled {
    #[inline]
    pub fn ldnt1w_z_p_ar_d_64_unscaled(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000101000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(U.into()) << 14u32
                | 0b0u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
