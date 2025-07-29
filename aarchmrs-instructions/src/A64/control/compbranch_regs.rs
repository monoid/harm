/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CBGT_32_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110100000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBGT_32_regs";
    #[inline]
    pub const fn CBGT_32_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b00u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBGE_32_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110100001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBGE_32_regs";
    #[inline]
    pub const fn CBGE_32_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100001u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b00u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBHI_32_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110100010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBHI_32_regs";
    #[inline]
    pub const fn CBHI_32_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100010u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b00u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBHS_32_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110100011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBHS_32_regs";
    #[inline]
    pub const fn CBHS_32_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100011u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b00u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBEQ_32_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110100110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBEQ_32_regs";
    #[inline]
    pub const fn CBEQ_32_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100110u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b00u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBNE_32_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110100111000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBNE_32_regs";
    #[inline]
    pub const fn CBNE_32_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100111u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b00u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBGT_64_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110100000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBGT_64_regs";
    #[inline]
    pub const fn CBGT_64_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110100000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b00u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBGE_64_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110100001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBGE_64_regs";
    #[inline]
    pub const fn CBGE_64_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110100001u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b00u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBHI_64_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110100010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBHI_64_regs";
    #[inline]
    pub const fn CBHI_64_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110100010u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b00u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBHS_64_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110100011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBHS_64_regs";
    #[inline]
    pub const fn CBHS_64_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110100011u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b00u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBEQ_64_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110100110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBEQ_64_regs";
    #[inline]
    pub const fn CBEQ_64_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110100110u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b00u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBNE_64_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110100111000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBNE_64_regs";
    #[inline]
    pub const fn CBNE_64_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110100111u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b00u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
