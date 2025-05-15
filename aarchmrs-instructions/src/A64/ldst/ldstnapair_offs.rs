/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STNP_32_ldstnapair_offs {
    #[inline]
    pub fn STNP_32_ldstnapair_offs(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010100000u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDNP_32_ldstnapair_offs {
    #[inline]
    pub fn LDNP_32_ldstnapair_offs(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010100001u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STNP_S_ldstnapair_offs {
    #[inline]
    pub fn STNP_S_ldstnapair_offs(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010110000u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDNP_S_ldstnapair_offs {
    #[inline]
    pub fn LDNP_S_ldstnapair_offs(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010110001u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STNP_D_ldstnapair_offs {
    #[inline]
    pub fn STNP_D_ldstnapair_offs(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110110000u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDNP_D_ldstnapair_offs {
    #[inline]
    pub fn LDNP_D_ldstnapair_offs(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110110001u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STNP_64_ldstnapair_offs {
    #[inline]
    pub fn STNP_64_ldstnapair_offs(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010100000u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDNP_64_ldstnapair_offs {
    #[inline]
    pub fn LDNP_64_ldstnapair_offs(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010100001u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STNP_Q_ldstnapair_offs {
    #[inline]
    pub fn STNP_Q_ldstnapair_offs(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010110000u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDNP_Q_ldstnapair_offs {
    #[inline]
    pub fn LDNP_Q_ldstnapair_offs(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010110001u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STTNP_64_ldstnapair_offs {
    #[inline]
    pub fn STTNP_64_ldstnapair_offs(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110100000u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTNP_64_ldstnapair_offs {
    #[inline]
    pub fn LDTNP_64_ldstnapair_offs(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110100001u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STTNP_Q_ldstnapair_offs {
    #[inline]
    pub fn STTNP_Q_ldstnapair_offs(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110110000u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTNP_Q_ldstnapair_offs {
    #[inline]
    pub fn LDTNP_Q_ldstnapair_offs(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110110001u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
