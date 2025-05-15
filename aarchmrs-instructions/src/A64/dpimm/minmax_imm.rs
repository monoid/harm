/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SMAX_32_minmax_imm {
    #[inline]
    pub fn SMAX_32_minmax_imm(
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00010001110000u32 << 18u32
                | u32::from(imm8.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UMAX_32U_minmax_imm {
    #[inline]
    pub fn UMAX_32U_minmax_imm(
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00010001110001u32 << 18u32
                | u32::from(imm8.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SMIN_32_minmax_imm {
    #[inline]
    pub fn SMIN_32_minmax_imm(
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00010001110010u32 << 18u32
                | u32::from(imm8.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UMIN_32U_minmax_imm {
    #[inline]
    pub fn UMIN_32U_minmax_imm(
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00010001110011u32 << 18u32
                | u32::from(imm8.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SMAX_64_minmax_imm {
    #[inline]
    pub fn SMAX_64_minmax_imm(
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10010001110000u32 << 18u32
                | u32::from(imm8.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UMAX_64U_minmax_imm {
    #[inline]
    pub fn UMAX_64U_minmax_imm(
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10010001110001u32 << 18u32
                | u32::from(imm8.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SMIN_64_minmax_imm {
    #[inline]
    pub fn SMIN_64_minmax_imm(
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10010001110010u32 << 18u32
                | u32::from(imm8.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UMIN_64U_minmax_imm {
    #[inline]
    pub fn UMIN_64U_minmax_imm(
        imm8: impl Into<::aarchmrs_types::BitValue<8>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10010001110011u32 << 18u32
                | u32::from(imm8.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
