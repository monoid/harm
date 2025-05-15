/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod st1b_mzx_p_bi_4x4 {
    #[inline]
    pub fn st1b_mzx_p_bi_4x4(
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zt: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000010110u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(T.into()) << 4u32
                | 0b00u32 << 2u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod stnt1b_mzx_p_bi_4x4 {
    #[inline]
    pub fn stnt1b_mzx_p_bi_4x4(
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zt: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000010110u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(T.into()) << 4u32
                | 0b10u32 << 2u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod st1h_mzx_p_bi_4x4 {
    #[inline]
    pub fn st1h_mzx_p_bi_4x4(
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zt: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000010110u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(T.into()) << 4u32
                | 0b00u32 << 2u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod stnt1h_mzx_p_bi_4x4 {
    #[inline]
    pub fn stnt1h_mzx_p_bi_4x4(
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zt: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000010110u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(T.into()) << 4u32
                | 0b10u32 << 2u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod st1w_mzx_p_bi_4x4 {
    #[inline]
    pub fn st1w_mzx_p_bi_4x4(
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zt: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000010110u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(T.into()) << 4u32
                | 0b00u32 << 2u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod stnt1w_mzx_p_bi_4x4 {
    #[inline]
    pub fn stnt1w_mzx_p_bi_4x4(
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zt: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000010110u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(T.into()) << 4u32
                | 0b10u32 << 2u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod st1d_mzx_p_bi_4x4 {
    #[inline]
    pub fn st1d_mzx_p_bi_4x4(
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zt: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000010110u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(T.into()) << 4u32
                | 0b00u32 << 2u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod stnt1d_mzx_p_bi_4x4 {
    #[inline]
    pub fn stnt1d_mzx_p_bi_4x4(
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zt: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000010110u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(T.into()) << 4u32
                | 0b10u32 << 2u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
