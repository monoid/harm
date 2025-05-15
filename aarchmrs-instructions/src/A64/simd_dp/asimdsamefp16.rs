/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FMAXNM_asimdsamefp16_only {
    #[inline]
    pub fn FMAXNM_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMLA_asimdsamefp16_only {
    #[inline]
    pub fn FMLA_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000011u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FADD_asimdsamefp16_only {
    #[inline]
    pub fn FADD_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMULX_asimdsamefp16_only {
    #[inline]
    pub fn FMULX_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCMEQ_asimdsamefp16_only {
    #[inline]
    pub fn FCMEQ_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        E: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ac: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110u32 << 24u32
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
pub mod FMAX_asimdsamefp16_only {
    #[inline]
    pub fn FMAX_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110u32 << 24u32
                | u32::from(o1.into()) << 23u32
                | 0b10u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FRECPS_asimdsamefp16_only {
    #[inline]
    pub fn FRECPS_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMINNM_asimdsamefp16_only {
    #[inline]
    pub fn FMINNM_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMLS_asimdsamefp16_only {
    #[inline]
    pub fn FMLS_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000011u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FSUB_asimdsamefp16_only {
    #[inline]
    pub fn FSUB_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FAMAX_asimdsamefp16_only {
    #[inline]
    pub fn FAMAX_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMIN_asimdsamefp16_only {
    #[inline]
    pub fn FMIN_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110u32 << 24u32
                | u32::from(o1.into()) << 23u32
                | 0b10u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FRSQRTS_asimdsamefp16_only {
    #[inline]
    pub fn FRSQRTS_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMAXNMP_asimdsamefp16_only {
    #[inline]
    pub fn FMAXNMP_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FADDP_asimdsamefp16_only {
    #[inline]
    pub fn FADDP_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMUL_asimdsamefp16_only {
    #[inline]
    pub fn FMUL_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCMGE_asimdsamefp16_only {
    #[inline]
    pub fn FCMGE_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        E: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ac: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110u32 << 24u32
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
pub mod FACGE_asimdsamefp16_only {
    #[inline]
    pub fn FACGE_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        E: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ac: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110u32 << 24u32
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
pub mod FMAXP_asimdsamefp16_only {
    #[inline]
    pub fn FMAXP_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110u32 << 24u32
                | u32::from(o1.into()) << 23u32
                | 0b10u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FDIV_asimdsamefp16_only {
    #[inline]
    pub fn FDIV_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMINNMP_asimdsamefp16_only {
    #[inline]
    pub fn FMINNMP_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FABD_asimdsamefp16_only {
    #[inline]
    pub fn FABD_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FAMIN_asimdsamefp16_only {
    #[inline]
    pub fn FAMIN_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCMGT_asimdsamefp16_only {
    #[inline]
    pub fn FCMGT_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        E: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ac: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110u32 << 24u32
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
pub mod FACGT_asimdsamefp16_only {
    #[inline]
    pub fn FACGT_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        E: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ac: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110u32 << 24u32
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
pub mod FMINP_asimdsamefp16_only {
    #[inline]
    pub fn FMINP_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110u32 << 24u32
                | u32::from(o1.into()) << 23u32
                | 0b10u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FSCALE_asimdsamefp16_only {
    #[inline]
    pub fn FSCALE_asimdsamefp16_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
