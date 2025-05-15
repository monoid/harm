/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FMULX_asisdsamefp16_only {
    #[inline]
    pub fn FMULX_asisdsamefp16_only(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCMEQ_asisdsamefp16_only {
    #[inline]
    pub fn FCMEQ_asisdsamefp16_only(
        E: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ac: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110u32 << 24u32
                | u32::from(E.into()) << 23u32
                | 0b10u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0010u32 << 12u32
                | u32::from(ac.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FRECPS_asisdsamefp16_only {
    #[inline]
    pub fn FRECPS_asisdsamefp16_only(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FRSQRTS_asisdsamefp16_only {
    #[inline]
    pub fn FRSQRTS_asisdsamefp16_only(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCMGE_asisdsamefp16_only {
    #[inline]
    pub fn FCMGE_asisdsamefp16_only(
        E: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ac: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(E.into()) << 23u32
                | 0b10u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0010u32 << 12u32
                | u32::from(ac.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FACGE_asisdsamefp16_only {
    #[inline]
    pub fn FACGE_asisdsamefp16_only(
        E: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ac: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(E.into()) << 23u32
                | 0b10u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0010u32 << 12u32
                | u32::from(ac.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FABD_asisdsamefp16_only {
    #[inline]
    pub fn FABD_asisdsamefp16_only(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCMGT_asisdsamefp16_only {
    #[inline]
    pub fn FCMGT_asisdsamefp16_only(
        E: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ac: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(E.into()) << 23u32
                | 0b10u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0010u32 << 12u32
                | u32::from(ac.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FACGT_asisdsamefp16_only {
    #[inline]
    pub fn FACGT_asisdsamefp16_only(
        E: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ac: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(E.into()) << 23u32
                | 0b10u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0010u32 << 12u32
                | u32::from(ac.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
