/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod LDR_32_loadlit {
    #[inline]
    pub fn LDR_32_loadlit(
        imm19: impl Into<::aarchmrs_types::BitValue<19>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011000u32 << 24u32 | u32::from(imm19.into()) << 5u32 | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_S_loadlit {
    #[inline]
    pub fn LDR_S_loadlit(
        imm19: impl Into<::aarchmrs_types::BitValue<19>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011100u32 << 24u32 | u32::from(imm19.into()) << 5u32 | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_64_loadlit {
    #[inline]
    pub fn LDR_64_loadlit(
        imm19: impl Into<::aarchmrs_types::BitValue<19>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011000u32 << 24u32 | u32::from(imm19.into()) << 5u32 | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_D_loadlit {
    #[inline]
    pub fn LDR_D_loadlit(
        imm19: impl Into<::aarchmrs_types::BitValue<19>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011100u32 << 24u32 | u32::from(imm19.into()) << 5u32 | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRSW_64_loadlit {
    #[inline]
    pub fn LDRSW_64_loadlit(
        imm19: impl Into<::aarchmrs_types::BitValue<19>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011000u32 << 24u32 | u32::from(imm19.into()) << 5u32 | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_Q_loadlit {
    #[inline]
    pub fn LDR_Q_loadlit(
        imm19: impl Into<::aarchmrs_types::BitValue<19>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011100u32 << 24u32 | u32::from(imm19.into()) << 5u32 | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod PRFM_P_loadlit {
    #[inline]
    pub fn PRFM_P_loadlit(
        imm19: impl Into<::aarchmrs_types::BitValue<19>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011000u32 << 24u32 | u32::from(imm19.into()) << 5u32 | u32::from(Rt.into()) << 0u32,
        )
    }
}
