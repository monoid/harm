/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CBGT_32_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110101000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBGT_32_imm";
    #[inline]
    pub const fn CBGT_32_imm(
        imm6: ::aarchmrs_types::BitValue<6>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110101000u32 << 21u32
                | imm6.into_inner() << 15u32
                | 0b0u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBLT_32_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110101001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBLT_32_imm";
    #[inline]
    pub const fn CBLT_32_imm(
        imm6: ::aarchmrs_types::BitValue<6>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110101001u32 << 21u32
                | imm6.into_inner() << 15u32
                | 0b0u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBHI_32_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110101010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBHI_32_imm";
    #[inline]
    pub const fn CBHI_32_imm(
        imm6: ::aarchmrs_types::BitValue<6>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110101010u32 << 21u32
                | imm6.into_inner() << 15u32
                | 0b0u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBLO_32_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110101011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBLO_32_imm";
    #[inline]
    pub const fn CBLO_32_imm(
        imm6: ::aarchmrs_types::BitValue<6>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110101011u32 << 21u32
                | imm6.into_inner() << 15u32
                | 0b0u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBEQ_32_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110101110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBEQ_32_imm";
    #[inline]
    pub const fn CBEQ_32_imm(
        imm6: ::aarchmrs_types::BitValue<6>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110101110u32 << 21u32
                | imm6.into_inner() << 15u32
                | 0b0u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBNE_32_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110101111000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBNE_32_imm";
    #[inline]
    pub const fn CBNE_32_imm(
        imm6: ::aarchmrs_types::BitValue<6>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110101111u32 << 21u32
                | imm6.into_inner() << 15u32
                | 0b0u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBGT_64_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110101000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBGT_64_imm";
    #[inline]
    pub const fn CBGT_64_imm(
        imm6: ::aarchmrs_types::BitValue<6>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110101000u32 << 21u32
                | imm6.into_inner() << 15u32
                | 0b0u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBLT_64_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110101001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBLT_64_imm";
    #[inline]
    pub const fn CBLT_64_imm(
        imm6: ::aarchmrs_types::BitValue<6>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110101001u32 << 21u32
                | imm6.into_inner() << 15u32
                | 0b0u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBHI_64_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110101010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBHI_64_imm";
    #[inline]
    pub const fn CBHI_64_imm(
        imm6: ::aarchmrs_types::BitValue<6>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110101010u32 << 21u32
                | imm6.into_inner() << 15u32
                | 0b0u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBLO_64_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110101011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBLO_64_imm";
    #[inline]
    pub const fn CBLO_64_imm(
        imm6: ::aarchmrs_types::BitValue<6>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110101011u32 << 21u32
                | imm6.into_inner() << 15u32
                | 0b0u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBEQ_64_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110101110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBEQ_64_imm";
    #[inline]
    pub const fn CBEQ_64_imm(
        imm6: ::aarchmrs_types::BitValue<6>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110101110u32 << 21u32
                | imm6.into_inner() << 15u32
                | 0b0u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBNE_64_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110101111000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBNE_64_imm";
    #[inline]
    pub const fn CBNE_64_imm(
        imm6: ::aarchmrs_types::BitValue<6>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110101111u32 << 21u32
                | imm6.into_inner() << 15u32
                | 0b0u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
