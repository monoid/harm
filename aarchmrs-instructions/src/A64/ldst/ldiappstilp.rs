/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STILP_32SE_ldiappstilp {
    #[inline]
    pub fn STILP_32SE_ldiappstilp(
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011001000u32 << 21u32
                | u32::from(Rt2.into()) << 16u32
                | 0b000010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STILP_32S_ldiappstilp {
    #[inline]
    pub fn STILP_32S_ldiappstilp(
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011001000u32 << 21u32
                | u32::from(Rt2.into()) << 16u32
                | 0b000110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDIAPP_32LE_ldiappstilp {
    #[inline]
    pub fn LDIAPP_32LE_ldiappstilp(
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011001010u32 << 21u32
                | u32::from(Rt2.into()) << 16u32
                | 0b000010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDIAPP_32L_ldiappstilp {
    #[inline]
    pub fn LDIAPP_32L_ldiappstilp(
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011001010u32 << 21u32
                | u32::from(Rt2.into()) << 16u32
                | 0b000110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STILP_64SS_ldiappstilp {
    #[inline]
    pub fn STILP_64SS_ldiappstilp(
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011001000u32 << 21u32
                | u32::from(Rt2.into()) << 16u32
                | 0b000010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STILP_64S_ldiappstilp {
    #[inline]
    pub fn STILP_64S_ldiappstilp(
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011001000u32 << 21u32
                | u32::from(Rt2.into()) << 16u32
                | 0b000110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDIAPP_64LS_ldiappstilp {
    #[inline]
    pub fn LDIAPP_64LS_ldiappstilp(
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011001010u32 << 21u32
                | u32::from(Rt2.into()) << 16u32
                | 0b000010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDIAPP_64L_ldiappstilp {
    #[inline]
    pub fn LDIAPP_64L_ldiappstilp(
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011001010u32 << 21u32
                | u32::from(Rt2.into()) << 16u32
                | 0b000110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
