/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STRB_32B_ldst_regoff {
    #[inline]
    pub fn STRB_32B_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STRB_32BL_ldst_regoff {
    #[inline]
    pub fn STRB_32BL_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRB_32B_ldst_regoff {
    #[inline]
    pub fn LDRB_32B_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRB_32BL_ldst_regoff {
    #[inline]
    pub fn LDRB_32BL_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRSB_64B_ldst_regoff {
    #[inline]
    pub fn LDRSB_64B_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000101u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRSB_64BL_ldst_regoff {
    #[inline]
    pub fn LDRSB_64BL_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000101u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRSB_32B_ldst_regoff {
    #[inline]
    pub fn LDRSB_32B_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRSB_32BL_ldst_regoff {
    #[inline]
    pub fn LDRSB_32BL_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_B_ldst_regoff {
    #[inline]
    pub fn STR_B_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_BL_ldst_regoff {
    #[inline]
    pub fn STR_BL_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_B_ldst_regoff {
    #[inline]
    pub fn LDR_B_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_BL_ldst_regoff {
    #[inline]
    pub fn LDR_BL_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_Q_ldst_regoff {
    #[inline]
    pub fn STR_Q_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100101u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_Q_ldst_regoff {
    #[inline]
    pub fn LDR_Q_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STRH_32_ldst_regoff {
    #[inline]
    pub fn STRH_32_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRH_32_ldst_regoff {
    #[inline]
    pub fn LDRH_32_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRSH_64_ldst_regoff {
    #[inline]
    pub fn LDRSH_64_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000101u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRSH_32_ldst_regoff {
    #[inline]
    pub fn LDRSH_32_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_H_ldst_regoff {
    #[inline]
    pub fn STR_H_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_H_ldst_regoff {
    #[inline]
    pub fn LDR_H_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_32_ldst_regoff {
    #[inline]
    pub fn STR_32_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_32_ldst_regoff {
    #[inline]
    pub fn LDR_32_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRSW_64_ldst_regoff {
    #[inline]
    pub fn LDRSW_64_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000101u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_S_ldst_regoff {
    #[inline]
    pub fn STR_S_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_S_ldst_regoff {
    #[inline]
    pub fn LDR_S_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_64_ldst_regoff {
    #[inline]
    pub fn STR_64_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_64_ldst_regoff {
    #[inline]
    pub fn LDR_64_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod PRFM_P_ldst_regoff {
    #[inline]
    pub fn PRFM_P_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000101u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod RPRFM_R_ldst_regoff {
    #[inline]
    pub fn RPRFM_R_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000101u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STR_D_ldst_regoff {
    #[inline]
    pub fn STR_D_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDR_D_ldst_regoff {
    #[inline]
    pub fn LDR_D_ldst_regoff(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(S.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
