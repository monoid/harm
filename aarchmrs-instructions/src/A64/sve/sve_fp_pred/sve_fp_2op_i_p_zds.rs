/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fadd_z_p_zs_ {
    #[inline]
    pub fn fadd_z_p_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        i1: impl Into<::aarchmrs_types::BitValue<1>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b011000100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | 0b0000u32 << 6u32
                | u32::from(i1.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod fsub_z_p_zs_ {
    #[inline]
    pub fn fsub_z_p_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        i1: impl Into<::aarchmrs_types::BitValue<1>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b011001100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | 0b0000u32 << 6u32
                | u32::from(i1.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod fmul_z_p_zs_ {
    #[inline]
    pub fn fmul_z_p_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        i1: impl Into<::aarchmrs_types::BitValue<1>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b011010100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | 0b0000u32 << 6u32
                | u32::from(i1.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod fsubr_z_p_zs_ {
    #[inline]
    pub fn fsubr_z_p_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        i1: impl Into<::aarchmrs_types::BitValue<1>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b011011100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | 0b0000u32 << 6u32
                | u32::from(i1.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod fmaxnm_z_p_zs_ {
    #[inline]
    pub fn fmaxnm_z_p_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        i1: impl Into<::aarchmrs_types::BitValue<1>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b011100100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | 0b0000u32 << 6u32
                | u32::from(i1.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod fminnm_z_p_zs_ {
    #[inline]
    pub fn fminnm_z_p_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        i1: impl Into<::aarchmrs_types::BitValue<1>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b011101100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | 0b0000u32 << 6u32
                | u32::from(i1.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod fmax_z_p_zs_ {
    #[inline]
    pub fn fmax_z_p_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        i1: impl Into<::aarchmrs_types::BitValue<1>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b011110100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | 0b0000u32 << 6u32
                | u32::from(i1.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod fmin_z_p_zs_ {
    #[inline]
    pub fn fmin_z_p_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        i1: impl Into<::aarchmrs_types::BitValue<1>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b011111100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | 0b0000u32 << 6u32
                | u32::from(i1.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
