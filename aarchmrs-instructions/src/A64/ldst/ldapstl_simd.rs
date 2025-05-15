/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STLUR_B_ldapstl_simd {
    #[inline]
    pub fn STLUR_B_ldapstl_simd(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011101000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAPUR_B_ldapstl_simd {
    #[inline]
    pub fn LDAPUR_B_ldapstl_simd(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011101010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STLUR_Q_ldapstl_simd {
    #[inline]
    pub fn STLUR_Q_ldapstl_simd(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011101100u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAPUR_Q_ldapstl_simd {
    #[inline]
    pub fn LDAPUR_Q_ldapstl_simd(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011101110u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STLUR_H_ldapstl_simd {
    #[inline]
    pub fn STLUR_H_ldapstl_simd(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011101000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAPUR_H_ldapstl_simd {
    #[inline]
    pub fn LDAPUR_H_ldapstl_simd(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011101010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STLUR_S_ldapstl_simd {
    #[inline]
    pub fn STLUR_S_ldapstl_simd(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011101000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAPUR_S_ldapstl_simd {
    #[inline]
    pub fn LDAPUR_S_ldapstl_simd(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011101010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STLUR_D_ldapstl_simd {
    #[inline]
    pub fn STLUR_D_ldapstl_simd(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011101000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAPUR_D_ldapstl_simd {
    #[inline]
    pub fn LDAPUR_D_ldapstl_simd(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011101010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
