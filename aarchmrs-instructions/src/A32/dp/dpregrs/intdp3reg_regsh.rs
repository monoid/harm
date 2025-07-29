/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ANDS_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ANDS_rr_A1";
    #[inline]
    pub const fn ANDS_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00000001u32 << 20u32
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
pub mod AND_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AND_rr_A1";
    #[inline]
    pub const fn AND_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00000000u32 << 20u32
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
pub mod EORS_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000001100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "EORS_rr_A1";
    #[inline]
    pub const fn EORS_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00000011u32 << 20u32
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
pub mod EOR_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000001000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "EOR_rr_A1";
    #[inline]
    pub const fn EOR_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00000010u32 << 20u32
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
pub mod SUBS_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000010100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUBS_rr_A1";
    #[inline]
    pub const fn SUBS_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00000101u32 << 20u32
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
pub mod SUB_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000010000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUB_rr_A1";
    #[inline]
    pub const fn SUB_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00000100u32 << 20u32
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
pub mod RSBS_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000011100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RSBS_rr_A1";
    #[inline]
    pub const fn RSBS_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00000111u32 << 20u32
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
pub mod RSB_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000011000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RSB_rr_A1";
    #[inline]
    pub const fn RSB_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00000110u32 << 20u32
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
pub mod ADDS_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000100100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADDS_rr_A1";
    #[inline]
    pub const fn ADDS_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00001001u32 << 20u32
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
pub mod ADD_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000100000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADD_rr_A1";
    #[inline]
    pub const fn ADD_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00001000u32 << 20u32
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
pub mod ADCS_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000101100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADCS_rr_A1";
    #[inline]
    pub const fn ADCS_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00001011u32 << 20u32
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
pub mod ADC_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000101000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADC_rr_A1";
    #[inline]
    pub const fn ADC_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00001010u32 << 20u32
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
pub mod SBCS_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000110100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SBCS_rr_A1";
    #[inline]
    pub const fn SBCS_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00001101u32 << 20u32
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
pub mod SBC_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000110000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SBC_rr_A1";
    #[inline]
    pub const fn SBC_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00001100u32 << 20u32
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
pub mod RSCS_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000111100000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RSCS_rr_A1";
    #[inline]
    pub const fn RSCS_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00001111u32 << 20u32
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
pub mod RSC_rr_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000010010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000111000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RSC_rr_A1";
    #[inline]
    pub const fn RSC_rr_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rs: ::aarchmrs_types::BitValue<4>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00001110u32 << 20u32
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
