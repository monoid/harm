/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod AND_32_log_shift {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AND_32_log_shift";
    #[inline]
    pub const fn AND_32_log_shift(
        shift: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        imm6: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00001010u32 << 24u32
                | shift.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Rm.into_inner() << 16u32
                | imm6.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod BIC_32_log_shift {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001010001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BIC_32_log_shift";
    #[inline]
    pub const fn BIC_32_log_shift(
        shift: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        imm6: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00001010u32 << 24u32
                | shift.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | imm6.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ORR_32_log_shift {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORR_32_log_shift";
    #[inline]
    pub const fn ORR_32_log_shift(
        shift: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        imm6: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00101010u32 << 24u32
                | shift.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Rm.into_inner() << 16u32
                | imm6.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ORN_32_log_shift {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101010001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORN_32_log_shift";
    #[inline]
    pub const fn ORN_32_log_shift(
        shift: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        imm6: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00101010u32 << 24u32
                | shift.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | imm6.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod EOR_32_log_shift {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01001010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "EOR_32_log_shift";
    #[inline]
    pub const fn EOR_32_log_shift(
        shift: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        imm6: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01001010u32 << 24u32
                | shift.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Rm.into_inner() << 16u32
                | imm6.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod EON_32_log_shift {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01001010001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "EON_32_log_shift";
    #[inline]
    pub const fn EON_32_log_shift(
        shift: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        imm6: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01001010u32 << 24u32
                | shift.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | imm6.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ANDS_32_log_shift {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01101010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ANDS_32_log_shift";
    #[inline]
    pub const fn ANDS_32_log_shift(
        shift: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        imm6: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01101010u32 << 24u32
                | shift.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Rm.into_inner() << 16u32
                | imm6.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod BICS_32_log_shift {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01101010001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BICS_32_log_shift";
    #[inline]
    pub const fn BICS_32_log_shift(
        shift: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        imm6: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01101010u32 << 24u32
                | shift.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | imm6.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod AND_64_log_shift {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10001010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AND_64_log_shift";
    #[inline]
    pub const fn AND_64_log_shift(
        shift: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        imm6: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10001010u32 << 24u32
                | shift.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Rm.into_inner() << 16u32
                | imm6.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod BIC_64_log_shift {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10001010001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BIC_64_log_shift";
    #[inline]
    pub const fn BIC_64_log_shift(
        shift: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        imm6: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10001010u32 << 24u32
                | shift.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | imm6.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ORR_64_log_shift {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10101010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORR_64_log_shift";
    #[inline]
    pub const fn ORR_64_log_shift(
        shift: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        imm6: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10101010u32 << 24u32
                | shift.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Rm.into_inner() << 16u32
                | imm6.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ORN_64_log_shift {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10101010001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORN_64_log_shift";
    #[inline]
    pub const fn ORN_64_log_shift(
        shift: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        imm6: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10101010u32 << 24u32
                | shift.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | imm6.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod EOR_64_log_shift {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "EOR_64_log_shift";
    #[inline]
    pub const fn EOR_64_log_shift(
        shift: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        imm6: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001010u32 << 24u32
                | shift.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Rm.into_inner() << 16u32
                | imm6.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod EON_64_log_shift {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001010001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "EON_64_log_shift";
    #[inline]
    pub const fn EON_64_log_shift(
        shift: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        imm6: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001010u32 << 24u32
                | shift.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | imm6.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ANDS_64_log_shift {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ANDS_64_log_shift";
    #[inline]
    pub const fn ANDS_64_log_shift(
        shift: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        imm6: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101010u32 << 24u32
                | shift.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Rm.into_inner() << 16u32
                | imm6.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod BICS_64_log_shift {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BICS_64_log_shift";
    #[inline]
    pub const fn BICS_64_log_shift(
        shift: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        imm6: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101010u32 << 24u32
                | shift.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Rm.into_inner() << 16u32
                | imm6.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
