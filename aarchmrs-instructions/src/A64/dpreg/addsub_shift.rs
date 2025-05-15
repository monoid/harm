/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADD_32_addsub_shift {
    #[inline]
    pub fn ADD_32_addsub_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00001011u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ADDS_32_addsub_shift {
    #[inline]
    pub fn ADDS_32_addsub_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00101011u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SUB_32_addsub_shift {
    #[inline]
    pub fn SUB_32_addsub_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01001011u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SUBS_32_addsub_shift {
    #[inline]
    pub fn SUBS_32_addsub_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01101011u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ADD_64_addsub_shift {
    #[inline]
    pub fn ADD_64_addsub_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10001011u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ADDS_64_addsub_shift {
    #[inline]
    pub fn ADDS_64_addsub_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10101011u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SUB_64_addsub_shift {
    #[inline]
    pub fn SUB_64_addsub_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001011u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SUBS_64_addsub_shift {
    #[inline]
    pub fn SUBS_64_addsub_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101011u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
