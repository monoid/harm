/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STP_32_ldstpair_pre {
    #[inline]
    pub fn STP_32_ldstpair_pre(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010100110u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDP_32_ldstpair_pre {
    #[inline]
    pub fn LDP_32_ldstpair_pre(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010100111u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STP_S_ldstpair_pre {
    #[inline]
    pub fn STP_S_ldstpair_pre(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010110110u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDP_S_ldstpair_pre {
    #[inline]
    pub fn LDP_S_ldstpair_pre(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010110111u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STGP_64_ldstpair_pre {
    #[inline]
    pub fn STGP_64_ldstpair_pre(
        simm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110100110u32 << 22u32
                | u32::from(simm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDPSW_64_ldstpair_pre {
    #[inline]
    pub fn LDPSW_64_ldstpair_pre(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110100111u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STP_D_ldstpair_pre {
    #[inline]
    pub fn STP_D_ldstpair_pre(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110110110u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDP_D_ldstpair_pre {
    #[inline]
    pub fn LDP_D_ldstpair_pre(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110110111u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STP_64_ldstpair_pre {
    #[inline]
    pub fn STP_64_ldstpair_pre(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010100110u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDP_64_ldstpair_pre {
    #[inline]
    pub fn LDP_64_ldstpair_pre(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010100111u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STP_Q_ldstpair_pre {
    #[inline]
    pub fn STP_Q_ldstpair_pre(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010110110u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDP_Q_ldstpair_pre {
    #[inline]
    pub fn LDP_Q_ldstpair_pre(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010110111u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STTP_64_ldstpair_pre {
    #[inline]
    pub fn STTP_64_ldstpair_pre(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110100110u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTP_64_ldstpair_pre {
    #[inline]
    pub fn LDTP_64_ldstpair_pre(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110100111u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STTP_Q_ldstpair_pre {
    #[inline]
    pub fn STTP_Q_ldstpair_pre(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110110110u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDTP_Q_ldstpair_pre {
    #[inline]
    pub fn LDTP_Q_ldstpair_pre(
        imm7: impl Into<::aarchmrs_types::BitValue<7>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110110111u32 << 22u32
                | u32::from(imm7.into()) << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
