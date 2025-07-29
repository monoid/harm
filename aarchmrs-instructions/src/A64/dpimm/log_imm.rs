/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod AND_32_log_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00010010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AND_32_log_imm";
    #[inline]
    pub const fn AND_32_log_imm(
        immr: ::aarchmrs_types::BitValue<6>,
        imms: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001001000u32 << 22u32
                | immr.into_inner() << 16u32
                | imms.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ORR_32_log_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00110010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORR_32_log_imm";
    #[inline]
    pub const fn ORR_32_log_imm(
        immr: ::aarchmrs_types::BitValue<6>,
        imms: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0011001000u32 << 22u32
                | immr.into_inner() << 16u32
                | imms.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod EOR_32_log_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01010010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "EOR_32_log_imm";
    #[inline]
    pub const fn EOR_32_log_imm(
        immr: ::aarchmrs_types::BitValue<6>,
        imms: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101001000u32 << 22u32
                | immr.into_inner() << 16u32
                | imms.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ANDS_32S_log_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ANDS_32S_log_imm";
    #[inline]
    pub const fn ANDS_32S_log_imm(
        immr: ::aarchmrs_types::BitValue<6>,
        imms: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0111001000u32 << 22u32
                | immr.into_inner() << 16u32
                | imms.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod AND_64_log_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10010010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AND_64_log_imm";
    #[inline]
    pub const fn AND_64_log_imm(
        N: ::aarchmrs_types::BitValue<1>,
        immr: ::aarchmrs_types::BitValue<6>,
        imms: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b100100100u32 << 23u32
                | N.into_inner() << 22u32
                | immr.into_inner() << 16u32
                | imms.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ORR_64_log_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10110010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORR_64_log_imm";
    #[inline]
    pub const fn ORR_64_log_imm(
        N: ::aarchmrs_types::BitValue<1>,
        immr: ::aarchmrs_types::BitValue<6>,
        imms: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101100100u32 << 23u32
                | N.into_inner() << 22u32
                | immr.into_inner() << 16u32
                | imms.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod EOR_64_log_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "EOR_64_log_imm";
    #[inline]
    pub const fn EOR_64_log_imm(
        N: ::aarchmrs_types::BitValue<1>,
        immr: ::aarchmrs_types::BitValue<6>,
        imms: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110100100u32 << 23u32
                | N.into_inner() << 22u32
                | immr.into_inner() << 16u32
                | imms.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ANDS_64S_log_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ANDS_64S_log_imm";
    #[inline]
    pub const fn ANDS_64S_log_imm(
        N: ::aarchmrs_types::BitValue<1>,
        immr: ::aarchmrs_types::BitValue<6>,
        imms: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | N.into_inner() << 22u32
                | immr.into_inner() << 16u32
                | imms.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
