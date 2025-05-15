/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CBGT_32_imm {
    #[inline]
    pub fn CBGT_32_imm(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110101000u32 << 21u32
                | u32::from(imm6.into()) << 15u32
                | 0b0u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBLT_32_imm {
    #[inline]
    pub fn CBLT_32_imm(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110101001u32 << 21u32
                | u32::from(imm6.into()) << 15u32
                | 0b0u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBHI_32_imm {
    #[inline]
    pub fn CBHI_32_imm(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110101010u32 << 21u32
                | u32::from(imm6.into()) << 15u32
                | 0b0u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBLO_32_imm {
    #[inline]
    pub fn CBLO_32_imm(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110101011u32 << 21u32
                | u32::from(imm6.into()) << 15u32
                | 0b0u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBEQ_32_imm {
    #[inline]
    pub fn CBEQ_32_imm(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110101110u32 << 21u32
                | u32::from(imm6.into()) << 15u32
                | 0b0u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBNE_32_imm {
    #[inline]
    pub fn CBNE_32_imm(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110101111u32 << 21u32
                | u32::from(imm6.into()) << 15u32
                | 0b0u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBGT_64_imm {
    #[inline]
    pub fn CBGT_64_imm(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110101000u32 << 21u32
                | u32::from(imm6.into()) << 15u32
                | 0b0u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBLT_64_imm {
    #[inline]
    pub fn CBLT_64_imm(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110101001u32 << 21u32
                | u32::from(imm6.into()) << 15u32
                | 0b0u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBHI_64_imm {
    #[inline]
    pub fn CBHI_64_imm(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110101010u32 << 21u32
                | u32::from(imm6.into()) << 15u32
                | 0b0u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBLO_64_imm {
    #[inline]
    pub fn CBLO_64_imm(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110101011u32 << 21u32
                | u32::from(imm6.into()) << 15u32
                | 0b0u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBEQ_64_imm {
    #[inline]
    pub fn CBEQ_64_imm(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110101110u32 << 21u32
                | u32::from(imm6.into()) << 15u32
                | 0b0u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBNE_64_imm {
    #[inline]
    pub fn CBNE_64_imm(
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110101111u32 << 21u32
                | u32::from(imm6.into()) << 15u32
                | 0b0u32 << 14u32
                | u32::from(imm9.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
