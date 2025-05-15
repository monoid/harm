/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod st1b_mz_p_bi_4 {
    #[inline]
    pub fn st1b_mz_p_bi_4(
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000000110u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod stnt1b_mz_p_bi_4 {
    #[inline]
    pub fn stnt1b_mz_p_bi_4(
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000000110u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 2u32
                | 0b01u32 << 0u32,
        )
    }
}
pub mod st1h_mz_p_bi_4 {
    #[inline]
    pub fn st1h_mz_p_bi_4(
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000000110u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod stnt1h_mz_p_bi_4 {
    #[inline]
    pub fn stnt1h_mz_p_bi_4(
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000000110u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 2u32
                | 0b01u32 << 0u32,
        )
    }
}
pub mod st1w_mz_p_bi_4 {
    #[inline]
    pub fn st1w_mz_p_bi_4(
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000000110u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod stnt1w_mz_p_bi_4 {
    #[inline]
    pub fn stnt1w_mz_p_bi_4(
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000000110u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 2u32
                | 0b01u32 << 0u32,
        )
    }
}
pub mod st1d_mz_p_bi_4 {
    #[inline]
    pub fn st1d_mz_p_bi_4(
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000000110u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod stnt1d_mz_p_bi_4 {
    #[inline]
    pub fn stnt1d_mz_p_bi_4(
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        PNg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101000000110u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(msz.into()) << 13u32
                | u32::from(PNg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 2u32
                | 0b01u32 << 0u32,
        )
    }
}
