/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SQDMLAL_asisdelem_L {
    #[inline]
    pub fn SQDMLAL_asisdelem_L(
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
            0b01011111u32 << 24u32
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
pub mod SQDMLSL_asisdelem_L {
    #[inline]
    pub fn SQDMLSL_asisdelem_L(
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
            0b01011111u32 << 24u32
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
pub mod SQDMULL_asisdelem_L {
    #[inline]
    pub fn SQDMULL_asisdelem_L(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011111u32 << 24u32
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
pub mod SQDMULH_asisdelem_R {
    #[inline]
    pub fn SQDMULH_asisdelem_R(
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
            0b01011111u32 << 24u32
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
pub mod SQRDMULH_asisdelem_R {
    #[inline]
    pub fn SQRDMULH_asisdelem_R(
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
            0b01011111u32 << 24u32
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
pub mod FMLA_asisdelem_RH_H {
    #[inline]
    pub fn FMLA_asisdelem_RH_H(
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101111100u32 << 22u32
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
pub mod FMLS_asisdelem_RH_H {
    #[inline]
    pub fn FMLS_asisdelem_RH_H(
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101111100u32 << 22u32
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
pub mod FMUL_asisdelem_RH_H {
    #[inline]
    pub fn FMUL_asisdelem_RH_H(
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101111100u32 << 22u32
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
pub mod FMLA_asisdelem_R_SD {
    #[inline]
    pub fn FMLA_asisdelem_R_SD(
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
            0b010111111u32 << 23u32
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
pub mod FMLS_asisdelem_R_SD {
    #[inline]
    pub fn FMLS_asisdelem_R_SD(
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
            0b010111111u32 << 23u32
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
pub mod FMUL_asisdelem_R_SD {
    #[inline]
    pub fn FMUL_asisdelem_R_SD(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111111u32 << 23u32
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
pub mod SQRDMLAH_asisdelem_R {
    #[inline]
    pub fn SQRDMLAH_asisdelem_R(
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
            0b01111111u32 << 24u32
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
pub mod SQRDMLSH_asisdelem_R {
    #[inline]
    pub fn SQRDMLSH_asisdelem_R(
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
            0b01111111u32 << 24u32
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
pub mod FMULX_asisdelem_RH_H {
    #[inline]
    pub fn FMULX_asisdelem_RH_H(
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0111111100u32 << 22u32
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
pub mod FMULX_asisdelem_R_SD {
    #[inline]
    pub fn FMULX_asisdelem_R_SD(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        L: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<4>>,
        H: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111111u32 << 23u32
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
