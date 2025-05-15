/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SMLAL_asimdelem_L {
    #[inline]
    pub fn SMLAL_asimdelem_L(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001111u32 << 24u32
                | u32::from(size.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(o2.into()) << 14u32
                | 0b10u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQDMLAL_asimdelem_L {
    #[inline]
    pub fn SQDMLAL_asimdelem_L(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001111u32 << 24u32
                | u32::from(size.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(o2.into()) << 14u32
                | 0b11u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SMLSL_asimdelem_L {
    #[inline]
    pub fn SMLSL_asimdelem_L(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001111u32 << 24u32
                | u32::from(size.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(o2.into()) << 14u32
                | 0b10u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQDMLSL_asimdelem_L {
    #[inline]
    pub fn SQDMLSL_asimdelem_L(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001111u32 << 24u32
                | u32::from(size.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(o2.into()) << 14u32
                | 0b11u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod MUL_asimdelem_R {
    #[inline]
    pub fn MUL_asimdelem_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001111u32 << 24u32
                | u32::from(size.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1000u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SMULL_asimdelem_L {
    #[inline]
    pub fn SMULL_asimdelem_L(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001111u32 << 24u32
                | u32::from(size.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1010u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQDMULL_asimdelem_L {
    #[inline]
    pub fn SQDMULL_asimdelem_L(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001111u32 << 24u32
                | u32::from(size.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1011u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQDMULH_asimdelem_R {
    #[inline]
    pub fn SQDMULH_asimdelem_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001111u32 << 24u32
                | u32::from(size.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b110u32 << 13u32
                | u32::from(op.into()) << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQRDMULH_asimdelem_R {
    #[inline]
    pub fn SQRDMULH_asimdelem_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001111u32 << 24u32
                | u32::from(size.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b110u32 << 13u32
                | u32::from(op.into()) << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SDOT_asimdelem_D {
    #[inline]
    pub fn SDOT_asimdelem_D(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001111u32 << 24u32
                | u32::from(size.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1110u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FDOT_asimdelem_D {
    #[inline]
    pub fn FDOT_asimdelem_D(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00111100u32 << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b0000u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMLA_asimdelem_RH_H {
    #[inline]
    pub fn FMLA_asimdelem_RH_H(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00111100u32 << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(o2.into()) << 14u32
                | 0b01u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMLS_asimdelem_RH_H {
    #[inline]
    pub fn FMLS_asimdelem_RH_H(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00111100u32 << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(o2.into()) << 14u32
                | 0b01u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMUL_asimdelem_RH_H {
    #[inline]
    pub fn FMUL_asimdelem_RH_H(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00111100u32 << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1001u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SUDOT_asimdelem_D {
    #[inline]
    pub fn SUDOT_asimdelem_D(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        US: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001111u32 << 24u32
                | u32::from(US.into()) << 23u32
                | 0b0u32 << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1111u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FDOT_asimdelem_G {
    #[inline]
    pub fn FDOT_asimdelem_G(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00111101u32 << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b0000u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod BFDOT_asimdelem_E {
    #[inline]
    pub fn BFDOT_asimdelem_E(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00111101u32 << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1111u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMLA_asimdelem_R_SD {
    #[inline]
    pub fn FMLA_asimdelem_R_SD(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b0011111u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(o2.into()) << 14u32
                | 0b01u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMLS_asimdelem_R_SD {
    #[inline]
    pub fn FMLS_asimdelem_R_SD(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b0011111u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(o2.into()) << 14u32
                | 0b01u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMUL_asimdelem_R_SD {
    #[inline]
    pub fn FMUL_asimdelem_R_SD(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b0011111u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1001u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMLAL_asimdelem_LH {
    #[inline]
    pub fn FMLAL_asimdelem_LH(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b0011111u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(S.into()) << 14u32
                | 0b00u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMLSL_asimdelem_LH {
    #[inline]
    pub fn FMLSL_asimdelem_LH(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b0011111u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(S.into()) << 14u32
                | 0b00u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod USDOT_asimdelem_D {
    #[inline]
    pub fn USDOT_asimdelem_D(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        US: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001111u32 << 24u32
                | u32::from(US.into()) << 23u32
                | 0b0u32 << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1111u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod BFMLAL_asimdelem_F {
    #[inline]
    pub fn BFMLAL_asimdelem_F(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00111111u32 << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1111u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod MLA_asimdelem_R {
    #[inline]
    pub fn MLA_asimdelem_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101111u32 << 24u32
                | u32::from(size.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(o2.into()) << 14u32
                | 0b00u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UMLAL_asimdelem_L {
    #[inline]
    pub fn UMLAL_asimdelem_L(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101111u32 << 24u32
                | u32::from(size.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(o2.into()) << 14u32
                | 0b10u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod MLS_asimdelem_R {
    #[inline]
    pub fn MLS_asimdelem_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101111u32 << 24u32
                | u32::from(size.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(o2.into()) << 14u32
                | 0b00u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UMLSL_asimdelem_L {
    #[inline]
    pub fn UMLSL_asimdelem_L(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101111u32 << 24u32
                | u32::from(size.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(o2.into()) << 14u32
                | 0b10u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UMULL_asimdelem_L {
    #[inline]
    pub fn UMULL_asimdelem_L(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101111u32 << 24u32
                | u32::from(size.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1010u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQRDMLAH_asimdelem_R {
    #[inline]
    pub fn SQRDMLAH_asimdelem_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101111u32 << 24u32
                | u32::from(size.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b11u32 << 14u32
                | u32::from(S.into()) << 13u32
                | 0b1u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UDOT_asimdelem_D {
    #[inline]
    pub fn UDOT_asimdelem_D(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101111u32 << 24u32
                | u32::from(size.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1110u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQRDMLSH_asimdelem_R {
    #[inline]
    pub fn SQRDMLSH_asimdelem_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101111u32 << 24u32
                | u32::from(size.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b11u32 << 14u32
                | u32::from(S.into()) << 13u32
                | 0b1u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMULX_asimdelem_RH_H {
    #[inline]
    pub fn FMULX_asimdelem_RH_H(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b10111100u32 << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1001u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCMLA_advsimd_elt {
    #[inline]
    pub fn FCMLA_advsimd_elt(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        rot: impl Into<::aarchmrs_types::BitValue<2>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101111u32 << 24u32
                | u32::from(size.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(rot.into()) << 13u32
                | 0b1u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMULX_asimdelem_R_SD {
    #[inline]
    pub fn FMULX_asimdelem_R_SD(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b1011111u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1001u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMLAL2_asimdelem_LH {
    #[inline]
    pub fn FMLAL2_asimdelem_LH(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b1011111u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(S.into()) << 14u32
                | 0b00u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMLSL2_asimdelem_LH {
    #[inline]
    pub fn FMLSL2_asimdelem_LH(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b1011111u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(S.into()) << 14u32
                | 0b00u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMLALB_asimdelem_H {
    #[inline]
    pub fn FMLALB_asimdelem_H(
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0000111111u32 << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b0000u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMLALLBB_asimdelem_J {
    #[inline]
    pub fn FMLALLBB_asimdelem_J(
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010111100u32 << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1000u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMLALLBT_asimdelem_J {
    #[inline]
    pub fn FMLALLBT_asimdelem_J(
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010111101u32 << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1000u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMLALT_asimdelem_H {
    #[inline]
    pub fn FMLALT_asimdelem_H(
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100111111u32 << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b0000u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMLALLTB_asimdelem_J {
    #[inline]
    pub fn FMLALLTB_asimdelem_J(
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110111100u32 << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1000u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMLALLTT_asimdelem_J {
    #[inline]
    pub fn FMLALLTT_asimdelem_J(
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110111101u32 << 22u32
                | u32::from(L.into()) << 21u32
                | u32::from(M.into()) << 20u32
                | u32::from(Rm.into()) << 16u32
                | 0b1000u32 << 12u32
                | u32::from(H.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
