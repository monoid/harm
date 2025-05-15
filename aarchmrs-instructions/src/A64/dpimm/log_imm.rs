/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod AND_32_log_imm {
    #[inline]
    pub fn AND_32_log_imm(
        immr: impl Into<::aarchmrs_types::BitValue<6>>,
        imms: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001001000u32 << 22u32
                | u32::from(immr.into()) << 16u32
                | u32::from(imms.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ORR_32_log_imm {
    #[inline]
    pub fn ORR_32_log_imm(
        immr: impl Into<::aarchmrs_types::BitValue<6>>,
        imms: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0011001000u32 << 22u32
                | u32::from(immr.into()) << 16u32
                | u32::from(imms.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod EOR_32_log_imm {
    #[inline]
    pub fn EOR_32_log_imm(
        immr: impl Into<::aarchmrs_types::BitValue<6>>,
        imms: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101001000u32 << 22u32
                | u32::from(immr.into()) << 16u32
                | u32::from(imms.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ANDS_32S_log_imm {
    #[inline]
    pub fn ANDS_32S_log_imm(
        immr: impl Into<::aarchmrs_types::BitValue<6>>,
        imms: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0111001000u32 << 22u32
                | u32::from(immr.into()) << 16u32
                | u32::from(imms.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod AND_64_log_imm {
    #[inline]
    pub fn AND_64_log_imm(
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        immr: impl Into<::aarchmrs_types::BitValue<6>>,
        imms: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b100100100u32 << 23u32
                | u32::from(N.into()) << 22u32
                | u32::from(immr.into()) << 16u32
                | u32::from(imms.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ORR_64_log_imm {
    #[inline]
    pub fn ORR_64_log_imm(
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        immr: impl Into<::aarchmrs_types::BitValue<6>>,
        imms: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101100100u32 << 23u32
                | u32::from(N.into()) << 22u32
                | u32::from(immr.into()) << 16u32
                | u32::from(imms.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod EOR_64_log_imm {
    #[inline]
    pub fn EOR_64_log_imm(
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        immr: impl Into<::aarchmrs_types::BitValue<6>>,
        imms: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110100100u32 << 23u32
                | u32::from(N.into()) << 22u32
                | u32::from(immr.into()) << 16u32
                | u32::from(imms.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ANDS_64S_log_imm {
    #[inline]
    pub fn ANDS_64S_log_imm(
        N: impl Into<::aarchmrs_types::BitValue<1>>,
        immr: impl Into<::aarchmrs_types::BitValue<6>>,
        imms: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | u32::from(N.into()) << 22u32
                | u32::from(immr.into()) << 16u32
                | u32::from(imms.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
