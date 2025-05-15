/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STRB_32_ldst_pos {
    #[inline]
    pub fn STRB_32_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0011100100u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRB_32_ldst_pos {
    #[inline]
    pub fn LDRB_32_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0011100101u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRSB_64_ldst_pos {
    #[inline]
    pub fn LDRSB_64_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0011100110u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRSB_32_ldst_pos {
    #[inline]
    pub fn LDRSB_32_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0011100111u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_B_ldst_pos {
    #[inline]
    pub fn STR_B_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0011110100u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_B_ldst_pos {
    #[inline]
    pub fn LDR_B_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0011110101u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_Q_ldst_pos {
    #[inline]
    pub fn STR_Q_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0011110110u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_Q_ldst_pos {
    #[inline]
    pub fn LDR_Q_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0011110111u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STRH_32_ldst_pos {
    #[inline]
    pub fn STRH_32_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0111100100u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRH_32_ldst_pos {
    #[inline]
    pub fn LDRH_32_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0111100101u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRSH_64_ldst_pos {
    #[inline]
    pub fn LDRSH_64_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0111100110u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRSH_32_ldst_pos {
    #[inline]
    pub fn LDRSH_32_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0111100111u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_H_ldst_pos {
    #[inline]
    pub fn STR_H_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0111110100u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_H_ldst_pos {
    #[inline]
    pub fn LDR_H_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0111110101u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_32_ldst_pos {
    #[inline]
    pub fn STR_32_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1011100100u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_32_ldst_pos {
    #[inline]
    pub fn LDR_32_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1011100101u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRSW_64_ldst_pos {
    #[inline]
    pub fn LDRSW_64_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1011100110u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_S_ldst_pos {
    #[inline]
    pub fn STR_S_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1011110100u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_S_ldst_pos {
    #[inline]
    pub fn LDR_S_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1011110101u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_64_ldst_pos {
    #[inline]
    pub fn STR_64_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111100100u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_64_ldst_pos {
    #[inline]
    pub fn LDR_64_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111100101u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod PRFM_P_ldst_pos {
    #[inline]
    pub fn PRFM_P_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111100110u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_D_ldst_pos {
    #[inline]
    pub fn STR_D_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111110100u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_D_ldst_pos {
    #[inline]
    pub fn LDR_D_ldst_pos(
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111110101u32 << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
