/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FCCMP_S_floatccmp {
    #[inline]
    pub fn FCCMP_S_floatccmp(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(nzcv.into()) << 0u32,
        )
    }
}
pub mod FCCMPE_S_floatccmp {
    #[inline]
    pub fn FCCMPE_S_floatccmp(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(nzcv.into()) << 0u32,
        )
    }
}
pub mod FCCMP_D_floatccmp {
    #[inline]
    pub fn FCCMP_D_floatccmp(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(nzcv.into()) << 0u32,
        )
    }
}
pub mod FCCMPE_D_floatccmp {
    #[inline]
    pub fn FCCMPE_D_floatccmp(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(nzcv.into()) << 0u32,
        )
    }
}
pub mod FCCMP_H_floatccmp {
    #[inline]
    pub fn FCCMP_H_floatccmp(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(nzcv.into()) << 0u32,
        )
    }
}
pub mod FCCMPE_H_floatccmp {
    #[inline]
    pub fn FCCMPE_H_floatccmp(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(nzcv.into()) << 0u32,
        )
    }
}
