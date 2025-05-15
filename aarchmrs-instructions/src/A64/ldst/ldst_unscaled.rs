/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STURB_32_ldst_unscaled {
    #[inline]
    pub fn STURB_32_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDURB_32_ldst_unscaled {
    #[inline]
    pub fn LDURB_32_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDURSB_64_ldst_unscaled {
    #[inline]
    pub fn LDURSB_64_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000100u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDURSB_32_ldst_unscaled {
    #[inline]
    pub fn LDURSB_32_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000110u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STUR_B_ldst_unscaled {
    #[inline]
    pub fn STUR_B_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUR_B_ldst_unscaled {
    #[inline]
    pub fn LDUR_B_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STUR_Q_ldst_unscaled {
    #[inline]
    pub fn STUR_Q_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100100u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUR_Q_ldst_unscaled {
    #[inline]
    pub fn LDUR_Q_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111100110u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STURH_32_ldst_unscaled {
    #[inline]
    pub fn STURH_32_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDURH_32_ldst_unscaled {
    #[inline]
    pub fn LDURH_32_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDURSH_64_ldst_unscaled {
    #[inline]
    pub fn LDURSH_64_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000100u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDURSH_32_ldst_unscaled {
    #[inline]
    pub fn LDURSH_32_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000110u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STUR_H_ldst_unscaled {
    #[inline]
    pub fn STUR_H_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUR_H_ldst_unscaled {
    #[inline]
    pub fn LDUR_H_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111100010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STUR_32_ldst_unscaled {
    #[inline]
    pub fn STUR_32_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUR_32_ldst_unscaled {
    #[inline]
    pub fn LDUR_32_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDURSW_64_ldst_unscaled {
    #[inline]
    pub fn LDURSW_64_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000100u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STUR_S_ldst_unscaled {
    #[inline]
    pub fn STUR_S_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUR_S_ldst_unscaled {
    #[inline]
    pub fn LDUR_S_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111100010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STUR_64_ldst_unscaled {
    #[inline]
    pub fn STUR_64_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUR_64_ldst_unscaled {
    #[inline]
    pub fn LDUR_64_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod PRFUM_P_ldst_unscaled {
    #[inline]
    pub fn PRFUM_P_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000100u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STUR_D_ldst_unscaled {
    #[inline]
    pub fn STUR_D_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100000u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDUR_D_ldst_unscaled {
    #[inline]
    pub fn LDUR_D_ldst_unscaled(
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111100010u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
