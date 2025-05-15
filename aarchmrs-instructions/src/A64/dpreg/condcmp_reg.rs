/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CCMN_32_condcmp_reg {
    #[inline]
    pub fn CCMN_32_condcmp_reg(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111010010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(nzcv.into()) << 0u32,
        )
    }
}
pub mod CCMP_32_condcmp_reg {
    #[inline]
    pub fn CCMP_32_condcmp_reg(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111010010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(nzcv.into()) << 0u32,
        )
    }
}
pub mod CCMN_64_condcmp_reg {
    #[inline]
    pub fn CCMN_64_condcmp_reg(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111010010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(nzcv.into()) << 0u32,
        )
    }
}
pub mod CCMP_64_condcmp_reg {
    #[inline]
    pub fn CCMP_64_condcmp_reg(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111010010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(nzcv.into()) << 0u32,
        )
    }
}
