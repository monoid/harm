/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CBBGT_8_regs {
    #[inline]
    pub fn CBBGT_8_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBBGE_8_regs {
    #[inline]
    pub fn CBBGE_8_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBBHI_8_regs {
    #[inline]
    pub fn CBBHI_8_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBBHS_8_regs {
    #[inline]
    pub fn CBBHS_8_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBBEQ_8_regs {
    #[inline]
    pub fn CBBEQ_8_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBBNE_8_regs {
    #[inline]
    pub fn CBBNE_8_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBHGT_16_regs {
    #[inline]
    pub fn CBHGT_16_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b11u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBHGE_16_regs {
    #[inline]
    pub fn CBHGE_16_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b11u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBHHI_16_regs {
    #[inline]
    pub fn CBHHI_16_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b11u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBHHS_16_regs {
    #[inline]
    pub fn CBHHS_16_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b11u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBHEQ_16_regs {
    #[inline]
    pub fn CBHEQ_16_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b11u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBHNE_16_regs {
    #[inline]
    pub fn CBHNE_16_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b11u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
