/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ldnt1b_z_p_bi_contiguous {
    #[inline]
    pub fn ldnt1b_z_p_bi_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | 0b000u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnt1h_z_p_bi_contiguous {
    #[inline]
    pub fn ldnt1h_z_p_bi_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | 0b000u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnt1w_z_p_bi_contiguous {
    #[inline]
    pub fn ldnt1w_z_p_bi_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | 0b000u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
pub mod ldnt1d_z_p_bi_contiguous {
    #[inline]
    pub fn ldnt1d_z_p_bi_contiguous(
        msz: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010010u32 << 25u32
                | u32::from(msz.into()) << 23u32
                | 0b000u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Zt.into()) << 0u32,
        )
    }
}
