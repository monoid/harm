/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1b_mz_p_br_4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1b_mz_p_br_4";
    #[inline]
    pub const fn ld1b_mz_p_br_4(
        Rm: ::aarchmrs_types::BitValue<5>,
        PNg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b100u32 << 13u32
                | PNg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod ldnt1b_mz_p_br_4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000000000001000000000000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnt1b_mz_p_br_4";
    #[inline]
    pub const fn ldnt1b_mz_p_br_4(
        Rm: ::aarchmrs_types::BitValue<5>,
        PNg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b100u32 << 13u32
                | PNg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 2u32
                | 0b01u32 << 0u32,
        )
    }
}
pub mod ld1h_mz_p_br_4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000000000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1h_mz_p_br_4";
    #[inline]
    pub const fn ld1h_mz_p_br_4(
        Rm: ::aarchmrs_types::BitValue<5>,
        PNg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b101u32 << 13u32
                | PNg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod ldnt1h_mz_p_br_4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000000000001010000000000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnt1h_mz_p_br_4";
    #[inline]
    pub const fn ldnt1h_mz_p_br_4(
        Rm: ::aarchmrs_types::BitValue<5>,
        PNg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b101u32 << 13u32
                | PNg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 2u32
                | 0b01u32 << 0u32,
        )
    }
}
pub mod ld1w_mz_p_br_4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000000000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1w_mz_p_br_4";
    #[inline]
    pub const fn ld1w_mz_p_br_4(
        Rm: ::aarchmrs_types::BitValue<5>,
        PNg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b110u32 << 13u32
                | PNg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod ldnt1w_mz_p_br_4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000000000001100000000000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnt1w_mz_p_br_4";
    #[inline]
    pub const fn ldnt1w_mz_p_br_4(
        Rm: ::aarchmrs_types::BitValue<5>,
        PNg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b110u32 << 13u32
                | PNg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 2u32
                | 0b01u32 << 0u32,
        )
    }
}
pub mod ld1d_mz_p_br_4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000000000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ld1d_mz_p_br_4";
    #[inline]
    pub const fn ld1d_mz_p_br_4(
        Rm: ::aarchmrs_types::BitValue<5>,
        PNg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b111u32 << 13u32
                | PNg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod ldnt1d_mz_p_br_4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000000011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10100000000000001110000000000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldnt1d_mz_p_br_4";
    #[inline]
    pub const fn ldnt1d_mz_p_br_4(
        Rm: ::aarchmrs_types::BitValue<5>,
        PNg: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Zt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10100000000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b111u32 << 13u32
                | PNg.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Zt.into_inner() << 2u32
                | 0b01u32 << 0u32,
        )
    }
}
