/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod prfb_i_p_bi_s {
    #[inline]
    pub fn prfb_i_p_bi_s(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        prfop: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010111u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(prfop.into()) << 0u32,
        )
    }
}
pub mod prfh_i_p_bi_s {
    #[inline]
    pub fn prfh_i_p_bi_s(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        prfop: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010111u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b001u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(prfop.into()) << 0u32,
        )
    }
}
pub mod prfw_i_p_bi_s {
    #[inline]
    pub fn prfw_i_p_bi_s(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        prfop: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010111u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(prfop.into()) << 0u32,
        )
    }
}
pub mod prfd_i_p_bi_s {
    #[inline]
    pub fn prfd_i_p_bi_s(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        prfop: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1000010111u32 << 22u32
                | u32::from(imm6.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(prfop.into()) << 0u32,
        )
    }
}
