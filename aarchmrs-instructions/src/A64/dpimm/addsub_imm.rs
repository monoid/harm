/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADD_32_addsub_imm {
    #[inline]
    pub fn ADD_32_addsub_imm(
        sh: impl Into<::aarchmrs_types::BitValue<1>>,
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000100010u32 << 23u32
                | u32::from(sh.into()) << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ADDS_32S_addsub_imm {
    #[inline]
    pub fn ADDS_32S_addsub_imm(
        sh: impl Into<::aarchmrs_types::BitValue<1>>,
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b001100010u32 << 23u32
                | u32::from(sh.into()) << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SUB_32_addsub_imm {
    #[inline]
    pub fn SUB_32_addsub_imm(
        sh: impl Into<::aarchmrs_types::BitValue<1>>,
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010100010u32 << 23u32
                | u32::from(sh.into()) << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SUBS_32S_addsub_imm {
    #[inline]
    pub fn SUBS_32S_addsub_imm(
        sh: impl Into<::aarchmrs_types::BitValue<1>>,
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011100010u32 << 23u32
                | u32::from(sh.into()) << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ADD_64_addsub_imm {
    #[inline]
    pub fn ADD_64_addsub_imm(
        sh: impl Into<::aarchmrs_types::BitValue<1>>,
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b100100010u32 << 23u32
                | u32::from(sh.into()) << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ADDS_64S_addsub_imm {
    #[inline]
    pub fn ADDS_64S_addsub_imm(
        sh: impl Into<::aarchmrs_types::BitValue<1>>,
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101100010u32 << 23u32
                | u32::from(sh.into()) << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SUB_64_addsub_imm {
    #[inline]
    pub fn SUB_64_addsub_imm(
        sh: impl Into<::aarchmrs_types::BitValue<1>>,
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110100010u32 << 23u32
                | u32::from(sh.into()) << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SUBS_64S_addsub_imm {
    #[inline]
    pub fn SUBS_64S_addsub_imm(
        sh: impl Into<::aarchmrs_types::BitValue<1>>,
        imm12: impl Into<::aarchmrs_types::BitValue<12>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100010u32 << 23u32
                | u32::from(sh.into()) << 22u32
                | u32::from(imm12.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
