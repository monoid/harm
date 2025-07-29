/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STTRB_32_ldst_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00111000000000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STTRB_32_ldst_unpriv";
    #[inline]
    pub const fn STTRB_32_ldst_unpriv(
        imm9: ::aarchmrs_types::BitValue<9>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000000u32 << 21u32
                | imm9.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDTRB_32_ldst_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00111000010000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDTRB_32_ldst_unpriv";
    #[inline]
    pub const fn LDTRB_32_ldst_unpriv(
        imm9: ::aarchmrs_types::BitValue<9>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000010u32 << 21u32
                | imm9.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDTRSB_64_ldst_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00111000100000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDTRSB_64_ldst_unpriv";
    #[inline]
    pub const fn LDTRSB_64_ldst_unpriv(
        imm9: ::aarchmrs_types::BitValue<9>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000100u32 << 21u32
                | imm9.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDTRSB_32_ldst_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00111000110000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDTRSB_32_ldst_unpriv";
    #[inline]
    pub const fn LDTRSB_32_ldst_unpriv(
        imm9: ::aarchmrs_types::BitValue<9>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00111000110u32 << 21u32
                | imm9.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STTRH_32_ldst_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111000000000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STTRH_32_ldst_unpriv";
    #[inline]
    pub const fn STTRH_32_ldst_unpriv(
        imm9: ::aarchmrs_types::BitValue<9>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000000u32 << 21u32
                | imm9.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDTRH_32_ldst_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111000010000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDTRH_32_ldst_unpriv";
    #[inline]
    pub const fn LDTRH_32_ldst_unpriv(
        imm9: ::aarchmrs_types::BitValue<9>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000010u32 << 21u32
                | imm9.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDTRSH_64_ldst_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111000100000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDTRSH_64_ldst_unpriv";
    #[inline]
    pub const fn LDTRSH_64_ldst_unpriv(
        imm9: ::aarchmrs_types::BitValue<9>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000100u32 << 21u32
                | imm9.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDTRSH_32_ldst_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111000110000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDTRSH_32_ldst_unpriv";
    #[inline]
    pub const fn LDTRSH_32_ldst_unpriv(
        imm9: ::aarchmrs_types::BitValue<9>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111000110u32 << 21u32
                | imm9.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STTR_32_ldst_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10111000000000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STTR_32_ldst_unpriv";
    #[inline]
    pub const fn STTR_32_ldst_unpriv(
        imm9: ::aarchmrs_types::BitValue<9>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000000u32 << 21u32
                | imm9.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDTR_32_ldst_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10111000010000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDTR_32_ldst_unpriv";
    #[inline]
    pub const fn LDTR_32_ldst_unpriv(
        imm9: ::aarchmrs_types::BitValue<9>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000010u32 << 21u32
                | imm9.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDTRSW_64_ldst_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10111000100000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDTRSW_64_ldst_unpriv";
    #[inline]
    pub const fn LDTRSW_64_ldst_unpriv(
        imm9: ::aarchmrs_types::BitValue<9>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111000100u32 << 21u32
                | imm9.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STTR_64_ldst_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000000000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STTR_64_ldst_unpriv";
    #[inline]
    pub const fn STTR_64_ldst_unpriv(
        imm9: ::aarchmrs_types::BitValue<9>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000000u32 << 21u32
                | imm9.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDTR_64_ldst_unpriv {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111000010000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDTR_64_ldst_unpriv";
    #[inline]
    pub const fn LDTR_64_ldst_unpriv(
        imm9: ::aarchmrs_types::BitValue<9>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11111000010u32 << 21u32
                | imm9.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
