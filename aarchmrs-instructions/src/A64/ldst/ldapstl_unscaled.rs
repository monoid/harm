/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STLURB_32_ldapstl_unscaled {
    #[inline]
    pub fn STLURB_32_ldapstl_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAPURB_32_ldapstl_unscaled {
    #[inline]
    pub fn LDAPURB_32_ldapstl_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAPURSB_64_ldapstl_unscaled {
    #[inline]
    pub fn LDAPURSB_64_ldapstl_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001100u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAPURSB_32_ldapstl_unscaled {
    #[inline]
    pub fn LDAPURSB_32_ldapstl_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011001110u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STLURH_32_ldapstl_unscaled {
    #[inline]
    pub fn STLURH_32_ldapstl_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAPURH_32_ldapstl_unscaled {
    #[inline]
    pub fn LDAPURH_32_ldapstl_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAPURSH_64_ldapstl_unscaled {
    #[inline]
    pub fn LDAPURSH_64_ldapstl_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001100u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAPURSH_32_ldapstl_unscaled {
    #[inline]
    pub fn LDAPURSH_32_ldapstl_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011001110u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STLUR_32_ldapstl_unscaled {
    #[inline]
    pub fn STLUR_32_ldapstl_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011001000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAPUR_32_ldapstl_unscaled {
    #[inline]
    pub fn LDAPUR_32_ldapstl_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011001010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAPURSW_64_ldapstl_unscaled {
    #[inline]
    pub fn LDAPURSW_64_ldapstl_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011001100u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STLUR_64_ldapstl_unscaled {
    #[inline]
    pub fn STLUR_64_ldapstl_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011001000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAPUR_64_ldapstl_unscaled {
    #[inline]
    pub fn LDAPUR_64_ldapstl_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011001010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
