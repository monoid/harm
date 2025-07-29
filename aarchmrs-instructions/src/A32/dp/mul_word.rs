/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MULS_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MULS_A1";
    #[inline]
    pub const fn MULS_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00000001u32 << 20u32
                | Rd.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rm.into_inner() << 8u32
                | 0b1001u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod MUL_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MUL_A1";
    #[inline]
    pub const fn MUL_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00000000u32 << 20u32
                | Rd.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rm.into_inner() << 8u32
                | 0b1001u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod MLAS_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000001100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MLAS_A1";
    #[inline]
    pub const fn MLAS_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Ra: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00000011u32 << 20u32
                | Rd.into_inner() << 16u32
                | Ra.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1001u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod MLA_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000001000000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MLA_A1";
    #[inline]
    pub const fn MLA_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Ra: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00000010u32 << 20u32
                | Rd.into_inner() << 16u32
                | Ra.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1001u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod UMAAL_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000010000000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UMAAL_A1";
    #[inline]
    pub const fn UMAAL_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00000100u32 << 20u32
                | RdHi.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1001u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod MLS_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000011000000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MLS_A1";
    #[inline]
    pub const fn MLS_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Ra: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00000110u32 << 20u32
                | Rd.into_inner() << 16u32
                | Ra.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1001u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod UMULLS_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000100100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UMULLS_A1";
    #[inline]
    pub const fn UMULLS_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00001001u32 << 20u32
                | RdHi.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1001u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod UMULL_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000100000000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UMULL_A1";
    #[inline]
    pub const fn UMULL_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00001000u32 << 20u32
                | RdHi.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1001u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod UMLALS_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000101100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UMLALS_A1";
    #[inline]
    pub const fn UMLALS_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00001011u32 << 20u32
                | RdHi.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1001u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod UMLAL_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000101000000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UMLAL_A1";
    #[inline]
    pub const fn UMLAL_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00001010u32 << 20u32
                | RdHi.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1001u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SMULLS_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000110100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMULLS_A1";
    #[inline]
    pub const fn SMULLS_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00001101u32 << 20u32
                | RdHi.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1001u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SMULL_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000110000000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMULL_A1";
    #[inline]
    pub const fn SMULL_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00001100u32 << 20u32
                | RdHi.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1001u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SMLALS_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLALS_A1";
    #[inline]
    pub const fn SMLALS_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00001111u32 << 20u32
                | RdHi.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1001u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SMLAL_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000111000000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLAL_A1";
    #[inline]
    pub const fn SMLAL_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00001110u32 << 20u32
                | RdHi.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1001u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
