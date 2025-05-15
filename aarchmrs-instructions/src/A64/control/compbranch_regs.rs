/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CBGT_32_regs {
    #[inline]
    pub fn CBGT_32_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBGE_32_regs {
    #[inline]
    pub fn CBGE_32_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBHI_32_regs {
    #[inline]
    pub fn CBHI_32_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBHS_32_regs {
    #[inline]
    pub fn CBHS_32_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBEQ_32_regs {
    #[inline]
    pub fn CBEQ_32_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBNE_32_regs {
    #[inline]
    pub fn CBNE_32_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBGT_64_regs {
    #[inline]
    pub fn CBGT_64_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110100000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBGE_64_regs {
    #[inline]
    pub fn CBGE_64_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110100001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBHI_64_regs {
    #[inline]
    pub fn CBHI_64_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110100010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBHS_64_regs {
    #[inline]
    pub fn CBHS_64_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110100011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBEQ_64_regs {
    #[inline]
    pub fn CBEQ_64_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110100110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBNE_64_regs {
    #[inline]
    pub fn CBNE_64_regs(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110100111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
