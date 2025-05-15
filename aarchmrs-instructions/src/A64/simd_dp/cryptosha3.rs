/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SHA1C_QSV_cryptosha3 {
    #[inline]
    pub fn SHA1C_QSV_cryptosha3(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SHA1P_QSV_cryptosha3 {
    #[inline]
    pub fn SHA1P_QSV_cryptosha3(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SHA1M_QSV_cryptosha3 {
    #[inline]
    pub fn SHA1M_QSV_cryptosha3(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SHA1SU0_VVV_cryptosha3 {
    #[inline]
    pub fn SHA1SU0_VVV_cryptosha3(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SHA256H_QQV_cryptosha3 {
    #[inline]
    pub fn SHA256H_QQV_cryptosha3(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        P: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(P.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SHA256H2_QQV_cryptosha3 {
    #[inline]
    pub fn SHA256H2_QQV_cryptosha3(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        P: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(P.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SHA256SU1_VVV_cryptosha3 {
    #[inline]
    pub fn SHA256SU1_VVV_cryptosha3(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
