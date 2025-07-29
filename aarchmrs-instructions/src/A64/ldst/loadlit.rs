/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod LDR_32_loadlit {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDR_32_loadlit";
    #[inline]
    pub const fn LDR_32_loadlit(
        imm19: ::aarchmrs_types::BitValue<19>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011000u32 << 24u32 | imm19.into_inner() << 5u32 | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDR_S_loadlit {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011100000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDR_S_loadlit";
    #[inline]
    pub const fn LDR_S_loadlit(
        imm19: ::aarchmrs_types::BitValue<19>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011100u32 << 24u32 | imm19.into_inner() << 5u32 | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDR_64_loadlit {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDR_64_loadlit";
    #[inline]
    pub const fn LDR_64_loadlit(
        imm19: ::aarchmrs_types::BitValue<19>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011000u32 << 24u32 | imm19.into_inner() << 5u32 | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDR_D_loadlit {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011100000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDR_D_loadlit";
    #[inline]
    pub const fn LDR_D_loadlit(
        imm19: ::aarchmrs_types::BitValue<19>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011100u32 << 24u32 | imm19.into_inner() << 5u32 | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDRSW_64_loadlit {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSW_64_loadlit";
    #[inline]
    pub const fn LDRSW_64_loadlit(
        imm19: ::aarchmrs_types::BitValue<19>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011000u32 << 24u32 | imm19.into_inner() << 5u32 | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDR_Q_loadlit {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011100000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDR_Q_loadlit";
    #[inline]
    pub const fn LDR_Q_loadlit(
        imm19: ::aarchmrs_types::BitValue<19>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011100u32 << 24u32 | imm19.into_inner() << 5u32 | Rt.into_inner() << 0u32,
        )
    }
}
pub mod PRFM_P_loadlit {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11011000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PRFM_P_loadlit";
    #[inline]
    pub const fn PRFM_P_loadlit(
        imm19: ::aarchmrs_types::BitValue<19>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011000u32 << 24u32 | imm19.into_inner() << 5u32 | Rt.into_inner() << 0u32,
        )
    }
}
