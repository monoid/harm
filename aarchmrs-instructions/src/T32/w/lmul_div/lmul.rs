/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SMULL_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111011100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMULL_T1";
    #[inline]
    pub const fn SMULL_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110111000u32 << 20u32
                | Rn.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | RdHi.into_inner() << 8u32
                | 0b0000u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UMULL_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111011101000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UMULL_T1";
    #[inline]
    pub const fn UMULL_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110111010u32 << 20u32
                | Rn.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | RdHi.into_inner() << 8u32
                | 0b0000u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SMLAL_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111011110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLAL_T1";
    #[inline]
    pub const fn SMLAL_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110111100u32 << 20u32
                | Rn.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | RdHi.into_inner() << 8u32
                | 0b0000u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SMLALBB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111011110000000000000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLALBB_T1";
    #[inline]
    pub const fn SMLALBB_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110111100u32 << 20u32
                | Rn.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | RdHi.into_inner() << 8u32
                | 0b1000u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SMLALBT_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111011110000000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLALBT_T1";
    #[inline]
    pub const fn SMLALBT_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110111100u32 << 20u32
                | Rn.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | RdHi.into_inner() << 8u32
                | 0b1001u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SMLALTB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111011110000000000000010100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLALTB_T1";
    #[inline]
    pub const fn SMLALTB_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110111100u32 << 20u32
                | Rn.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | RdHi.into_inner() << 8u32
                | 0b1010u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SMLALTT_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111011110000000000000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLALTT_T1";
    #[inline]
    pub const fn SMLALTT_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110111100u32 << 20u32
                | Rn.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | RdHi.into_inner() << 8u32
                | 0b1011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SMLALD_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111011110000000000000011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLALD_T1";
    #[inline]
    pub const fn SMLALD_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110111100u32 << 20u32
                | Rn.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | RdHi.into_inner() << 8u32
                | 0b1100u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SMLALDX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111011110000000000000011010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLALDX_T1";
    #[inline]
    pub const fn SMLALDX_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110111100u32 << 20u32
                | Rn.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | RdHi.into_inner() << 8u32
                | 0b1101u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SMLSLD_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111011110100000000000011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLSLD_T1";
    #[inline]
    pub const fn SMLSLD_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110111101u32 << 20u32
                | Rn.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | RdHi.into_inner() << 8u32
                | 0b1100u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SMLSLDX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111011110100000000000011010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLSLDX_T1";
    #[inline]
    pub const fn SMLSLDX_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110111101u32 << 20u32
                | Rn.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | RdHi.into_inner() << 8u32
                | 0b1101u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UMLAL_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111011111000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UMLAL_T1";
    #[inline]
    pub const fn UMLAL_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110111110u32 << 20u32
                | Rn.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | RdHi.into_inner() << 8u32
                | 0b0000u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UMAAL_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111011111000000000000001100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UMAAL_T1";
    #[inline]
    pub const fn UMAAL_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110111110u32 << 20u32
                | Rn.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | RdHi.into_inner() << 8u32
                | 0b0110u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
