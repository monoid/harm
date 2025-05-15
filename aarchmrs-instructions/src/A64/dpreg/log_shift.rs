/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod AND_32_log_shift {
    #[inline]
    pub fn AND_32_log_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00001010u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod BIC_32_log_shift {
    #[inline]
    pub fn BIC_32_log_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00001010u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ORR_32_log_shift {
    #[inline]
    pub fn ORR_32_log_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00101010u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ORN_32_log_shift {
    #[inline]
    pub fn ORN_32_log_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00101010u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod EOR_32_log_shift {
    #[inline]
    pub fn EOR_32_log_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01001010u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod EON_32_log_shift {
    #[inline]
    pub fn EON_32_log_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01001010u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ANDS_32_log_shift {
    #[inline]
    pub fn ANDS_32_log_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01101010u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod BICS_32_log_shift {
    #[inline]
    pub fn BICS_32_log_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01101010u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod AND_64_log_shift {
    #[inline]
    pub fn AND_64_log_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10001010u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod BIC_64_log_shift {
    #[inline]
    pub fn BIC_64_log_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10001010u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ORR_64_log_shift {
    #[inline]
    pub fn ORR_64_log_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10101010u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ORN_64_log_shift {
    #[inline]
    pub fn ORN_64_log_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10101010u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod EOR_64_log_shift {
    #[inline]
    pub fn EOR_64_log_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001010u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod EON_64_log_shift {
    #[inline]
    pub fn EON_64_log_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001010u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ANDS_64_log_shift {
    #[inline]
    pub fn ANDS_64_log_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101010u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod BICS_64_log_shift {
    #[inline]
    pub fn BICS_64_log_shift(
        shift: impl Into<::aarchmrs_types::BitValue<2>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm6: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101010u32 << 24u32
                | u32::from(shift.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(imm6.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
