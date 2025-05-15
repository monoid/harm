/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADD_32_addsub_ext {
    #[inline]
    pub fn ADD_32_addsub_ext(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00001011001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(imm3.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ADDS_32S_addsub_ext {
    #[inline]
    pub fn ADDS_32S_addsub_ext(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00101011001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(imm3.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SUB_32_addsub_ext {
    #[inline]
    pub fn SUB_32_addsub_ext(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01001011001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(imm3.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SUBS_32S_addsub_ext {
    #[inline]
    pub fn SUBS_32S_addsub_ext(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01101011001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(imm3.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ADD_64_addsub_ext {
    #[inline]
    pub fn ADD_64_addsub_ext(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10001011001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(imm3.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ADDS_64S_addsub_ext {
    #[inline]
    pub fn ADDS_64S_addsub_ext(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10101011001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(imm3.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SUB_64_addsub_ext {
    #[inline]
    pub fn SUB_64_addsub_ext(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001011001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(imm3.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SUBS_64S_addsub_ext {
    #[inline]
    pub fn SUBS_64S_addsub_ext(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        option: impl Into<::aarchmrs_types::BitValue<3>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101011001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(option.into()) << 13u32
                | u32::from(imm3.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
