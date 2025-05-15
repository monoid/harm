/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmad_z_p_zzz_ {
    #[inline]
    pub fn fmad_z_p_zzz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Za: impl Into<::aarchmrs_types::BitValue<5>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Za.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(N.into()) << 14u32
                | u32::from(op.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zm.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod fmsb_z_p_zzz_ {
    #[inline]
    pub fn fmsb_z_p_zzz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Za: impl Into<::aarchmrs_types::BitValue<5>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Za.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(N.into()) << 14u32
                | u32::from(op.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zm.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod fnmad_z_p_zzz_ {
    #[inline]
    pub fn fnmad_z_p_zzz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Za: impl Into<::aarchmrs_types::BitValue<5>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Za.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(N.into()) << 14u32
                | u32::from(op.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zm.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod fnmsb_z_p_zzz_ {
    #[inline]
    pub fn fnmsb_z_p_zzz_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Za: impl Into<::aarchmrs_types::BitValue<5>>,
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Za.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(N.into()) << 14u32
                | u32::from(op.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zm.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
