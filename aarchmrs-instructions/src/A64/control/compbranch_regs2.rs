/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CBBGT_8_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110100000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBBGT_8_regs";
    #[inline]
    pub const fn CBBGT_8_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b10u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBBGE_8_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110100001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBBGE_8_regs";
    #[inline]
    pub const fn CBBGE_8_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100001u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b10u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBBHI_8_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBBHI_8_regs";
    #[inline]
    pub const fn CBBHI_8_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100010u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b10u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBBHS_8_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110100011000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBBHS_8_regs";
    #[inline]
    pub const fn CBBHS_8_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100011u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b10u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBBEQ_8_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110100110000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBBEQ_8_regs";
    #[inline]
    pub const fn CBBEQ_8_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100110u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b10u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBBNE_8_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110100111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBBNE_8_regs";
    #[inline]
    pub const fn CBBNE_8_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100111u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b10u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBHGT_16_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110100000000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBHGT_16_regs";
    #[inline]
    pub const fn CBHGT_16_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b11u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBHGE_16_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110100001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBHGE_16_regs";
    #[inline]
    pub const fn CBHGE_16_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100001u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b11u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBHHI_16_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110100010000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBHHI_16_regs";
    #[inline]
    pub const fn CBHHI_16_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100010u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b11u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBHHS_16_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110100011000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBHHS_16_regs";
    #[inline]
    pub const fn CBHHS_16_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100011u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b11u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBHEQ_16_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110100110000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBHEQ_16_regs";
    #[inline]
    pub const fn CBHEQ_16_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100110u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b11u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CBHNE_16_regs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110100111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CBHNE_16_regs";
    #[inline]
    pub const fn CBHNE_16_regs(
        Rm: ::aarchmrs_types::BitValue<5>,
        imm9: ::aarchmrs_types::BitValue<9>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110100111u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b11u32 << 14u32
                | imm9.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
