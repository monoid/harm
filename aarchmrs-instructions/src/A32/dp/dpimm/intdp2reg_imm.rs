/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod AND_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AND_i_A1";
    #[inline]
    pub const fn AND_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00100000u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod ANDS_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010000100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ANDS_i_A1";
    #[inline]
    pub const fn ANDS_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00100001u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod EOR_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "EOR_i_A1";
    #[inline]
    pub const fn EOR_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00100010u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod EORS_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010001100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "EORS_i_A1";
    #[inline]
    pub const fn EORS_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00100011u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod SUB_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUB_i_A1";
    #[inline]
    pub const fn SUB_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00100100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod SUBS_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010010100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUBS_i_A1";
    #[inline]
    pub const fn SUBS_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00100101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod SUB_SP_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010010011010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUB_SP_i_A1";
    #[inline]
    pub const fn SUB_SP_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b001001001101u32 << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod SUBS_SP_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010010111010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUBS_SP_i_A1";
    #[inline]
    pub const fn SUBS_SP_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b001001011101u32 << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod ADR_A2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010010011110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADR_A2";
    #[inline]
    pub const fn ADR_A2(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b001001001111u32 << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod RSB_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RSB_i_A1";
    #[inline]
    pub const fn RSB_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00100110u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod RSBS_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RSBS_i_A1";
    #[inline]
    pub const fn RSBS_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00100111u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod ADD_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADD_i_A1";
    #[inline]
    pub const fn ADD_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00101000u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod ADDS_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010100100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADDS_i_A1";
    #[inline]
    pub const fn ADDS_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00101001u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod ADD_SP_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010100011010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADD_SP_i_A1";
    #[inline]
    pub const fn ADD_SP_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b001010001101u32 << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod ADDS_SP_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010100111010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADDS_SP_i_A1";
    #[inline]
    pub const fn ADDS_SP_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b001010011101u32 << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod ADR_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010100011110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADR_A1";
    #[inline]
    pub const fn ADR_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b001010001111u32 << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod ADC_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010101000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADC_i_A1";
    #[inline]
    pub const fn ADC_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00101010u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod ADCS_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010101100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADCS_i_A1";
    #[inline]
    pub const fn ADCS_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00101011u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod SBC_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SBC_i_A1";
    #[inline]
    pub const fn SBC_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00101100u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod SBCS_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010110100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SBCS_i_A1";
    #[inline]
    pub const fn SBCS_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00101101u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod RSC_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010111000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RSC_i_A1";
    #[inline]
    pub const fn RSC_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00101110u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod RSCS_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000010111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RSCS_i_A1";
    #[inline]
    pub const fn RSCS_i_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00101111u32 << 20u32
                | Rn.into_inner() << 16u32
                | Rd.into_inner() << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
