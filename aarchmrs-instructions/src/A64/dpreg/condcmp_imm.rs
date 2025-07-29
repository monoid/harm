/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CCMN_32_condcmp_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00111010010000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CCMN_32_condcmp_imm";
    #[inline]
    pub const fn CCMN_32_condcmp_imm(
        imm5: ::aarchmrs_types::BitValue<5>,
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<5>,
        nzcv: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111010010u32 << 21u32
                | imm5.into_inner() << 16u32
                | cond.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | nzcv.into_inner() << 0u32,
        )
    }
}
pub mod CCMP_32_condcmp_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111010010000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CCMP_32_condcmp_imm";
    #[inline]
    pub const fn CCMP_32_condcmp_imm(
        imm5: ::aarchmrs_types::BitValue<5>,
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<5>,
        nzcv: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111010010u32 << 21u32
                | imm5.into_inner() << 16u32
                | cond.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | nzcv.into_inner() << 0u32,
        )
    }
}
pub mod CCMN_64_condcmp_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10111010010000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CCMN_64_condcmp_imm";
    #[inline]
    pub const fn CCMN_64_condcmp_imm(
        imm5: ::aarchmrs_types::BitValue<5>,
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<5>,
        nzcv: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111010010u32 << 21u32
                | imm5.into_inner() << 16u32
                | cond.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | nzcv.into_inner() << 0u32,
        )
    }
}
pub mod CCMP_64_condcmp_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010010000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CCMP_64_condcmp_imm";
    #[inline]
    pub const fn CCMP_64_condcmp_imm(
        imm5: ::aarchmrs_types::BitValue<5>,
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<5>,
        nzcv: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111010010u32 << 21u32
                | imm5.into_inner() << 16u32
                | cond.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | nzcv.into_inner() << 0u32,
        )
    }
}
