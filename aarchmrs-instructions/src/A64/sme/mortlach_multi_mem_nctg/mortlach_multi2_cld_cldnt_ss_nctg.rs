/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1b_mzx_p_br_2x8 {
    #[inline]
    pub fn ld1b_mzx_p_br_2x8(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zt: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(T.into()) << 4u32
                | 0b0u32 << 3u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnt1b_mzx_p_br_2x8 {
    #[inline]
    pub fn ldnt1b_mzx_p_br_2x8(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zt: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(T.into()) << 4u32
                | 0b1u32 << 3u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1h_mzx_p_br_2x8 {
    #[inline]
    pub fn ld1h_mzx_p_br_2x8(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zt: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(T.into()) << 4u32
                | 0b0u32 << 3u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnt1h_mzx_p_br_2x8 {
    #[inline]
    pub fn ldnt1h_mzx_p_br_2x8(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zt: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(T.into()) << 4u32
                | 0b1u32 << 3u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1w_mzx_p_br_2x8 {
    #[inline]
    pub fn ld1w_mzx_p_br_2x8(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zt: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(T.into()) << 4u32
                | 0b0u32 << 3u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnt1w_mzx_p_br_2x8 {
    #[inline]
    pub fn ldnt1w_mzx_p_br_2x8(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zt: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(T.into()) << 4u32
                | 0b1u32 << 3u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ld1d_mzx_p_br_2x8 {
    #[inline]
    pub fn ld1d_mzx_p_br_2x8(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zt: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(T.into()) << 4u32
                | 0b0u32 << 3u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnt1d_mzx_p_br_2x8 {
    #[inline]
    pub fn ldnt1d_mzx_p_br_2x8(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zt: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100001000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(T.into()) << 4u32
                | 0b1u32 << 3u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
