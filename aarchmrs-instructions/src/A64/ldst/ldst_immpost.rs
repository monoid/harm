/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STRB_32_ldst_immpost {
    #[inline]
    pub fn STRB_32_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRB_32_ldst_immpost {
    #[inline]
    pub fn LDRB_32_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRSB_64_ldst_immpost {
    #[inline]
    pub fn LDRSB_64_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000100u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRSB_32_ldst_immpost {
    #[inline]
    pub fn LDRSB_32_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000110u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_B_ldst_immpost {
    #[inline]
    pub fn STR_B_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_B_ldst_immpost {
    #[inline]
    pub fn LDR_B_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_Q_ldst_immpost {
    #[inline]
    pub fn STR_Q_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100100u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_Q_ldst_immpost {
    #[inline]
    pub fn LDR_Q_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100110u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STRH_32_ldst_immpost {
    #[inline]
    pub fn STRH_32_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRH_32_ldst_immpost {
    #[inline]
    pub fn LDRH_32_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRSH_64_ldst_immpost {
    #[inline]
    pub fn LDRSH_64_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000100u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRSH_32_ldst_immpost {
    #[inline]
    pub fn LDRSH_32_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000110u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_H_ldst_immpost {
    #[inline]
    pub fn STR_H_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_H_ldst_immpost {
    #[inline]
    pub fn LDR_H_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_32_ldst_immpost {
    #[inline]
    pub fn STR_32_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_32_ldst_immpost {
    #[inline]
    pub fn LDR_32_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRSW_64_ldst_immpost {
    #[inline]
    pub fn LDRSW_64_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000100u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_S_ldst_immpost {
    #[inline]
    pub fn STR_S_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_S_ldst_immpost {
    #[inline]
    pub fn LDR_S_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_64_ldst_immpost {
    #[inline]
    pub fn STR_64_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_64_ldst_immpost {
    #[inline]
    pub fn LDR_64_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_D_ldst_immpost {
    #[inline]
    pub fn STR_D_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_D_ldst_immpost {
    #[inline]
    pub fn LDR_D_ldst_immpost(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
