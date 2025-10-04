/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CLREX_BN_barriers {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000011111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101000000110011000001011111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CLREX_BN_barriers";
    #[inline]
    pub const fn CLREX_BN_barriers(
        CRm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010101000000110011u32 << 12u32 | CRm.into_inner() << 8u32 | 0b01011111u32 << 0u32,
        )
    }
}
pub mod DSB_BO_barriers {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000011111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101000000110011000010011111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "DSB_BO_barriers";
    #[inline]
    pub const fn DSB_BO_barriers(
        CRm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010101000000110011u32 << 12u32 | CRm.into_inner() << 8u32 | 0b10011111u32 << 0u32,
        )
    }
}
pub mod DMB_BO_barriers {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000011111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101000000110011000010111111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "DMB_BO_barriers";
    #[inline]
    pub const fn DMB_BO_barriers(
        CRm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010101000000110011u32 << 12u32 | CRm.into_inner() << 8u32 | 0b10111111u32 << 0u32,
        )
    }
}
pub mod ISB_BI_barriers {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000011111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101000000110011000011011111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ISB_BI_barriers";
    #[inline]
    pub const fn ISB_BI_barriers(
        CRm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010101000000110011u32 << 12u32 | CRm.into_inner() << 8u32 | 0b11011111u32 << 0u32,
        )
    }
}
pub mod SB_only_barriers {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101000000110011000011111111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SB_only_barriers";
    #[inline]
    pub const fn SB_only_barriers() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110011000011111111u32 << 0u32)
    }
}
pub mod DSB_BOn_barriers {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111001111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010101000000110011001000111111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "DSB_BOn_barriers";
    #[inline]
    pub const fn DSB_BOn_barriers(
        imm2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010101000000110011u32 << 12u32
                | imm2.into_inner() << 10u32
                | 0b1000111111u32 << 0u32,
        )
    }
}
