/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ORRS_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001100100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORRS_rr_A1";
    #[inline]
    pub const fn ORRS_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00011001u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | Rs.into_inner() << 8u32
                | 0b0u32 << 7u32
                | stype.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ORR_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001100000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORR_rr_A1";
    #[inline]
    pub const fn ORR_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00011000u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | Rs.into_inner() << 8u32
                | 0b0u32 << 7u32
                | stype.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod MOVS_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001101100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOVS_rr_A1";
    #[inline]
    pub const fn MOVS_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b000110110000u32 << 16u32
                | Rd.into_inner() << 12u32
                | Rs.into_inner() << 8u32
                | 0b0u32 << 7u32
                | stype.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod MOV_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001101000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOV_rr_A1";
    #[inline]
    pub const fn MOV_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b000110100000u32 << 16u32
                | Rd.into_inner() << 12u32
                | Rs.into_inner() << 8u32
                | 0b0u32 << 7u32
                | stype.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod BICS_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001110100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BICS_rr_A1";
    #[inline]
    pub const fn BICS_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00011101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | Rs.into_inner() << 8u32
                | 0b0u32 << 7u32
                | stype.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod BIC_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001110000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BIC_rr_A1";
    #[inline]
    pub const fn BIC_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00011100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | Rs.into_inner() << 8u32
                | 0b0u32 << 7u32
                | stype.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod MVNS_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001111100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MVNS_rr_A1";
    #[inline]
    pub const fn MVNS_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b000111110000u32 << 16u32
                | Rd.into_inner() << 12u32
                | Rs.into_inner() << 8u32
                | 0b0u32 << 7u32
                | stype.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod MVN_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001111000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MVN_rr_A1";
    #[inline]
    pub const fn MVN_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b000111100000u32 << 16u32
                | Rd.into_inner() << 12u32
                | Rs.into_inner() << 8u32
                | 0b0u32 << 7u32
                | stype.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
