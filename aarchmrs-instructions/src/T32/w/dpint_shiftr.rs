/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod AND_r_T2_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010000000000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AND_r_T2_RRX";
    #[inline]
    pub const fn AND_r_T2_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010100000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod AND_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AND_r_T2";
    #[inline]
    pub const fn AND_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010100000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ANDS_r_T2_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010000100000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ANDS_r_T2_RRX";
    #[inline]
    pub const fn ANDS_r_T2_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010100001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ANDS_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010000100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ANDS_r_T2";
    #[inline]
    pub const fn ANDS_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010100001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod TST_r_T2_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010000100000000111100110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TST_r_T2_RRX";
    #[inline]
    pub const fn TST_r_T2_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010100001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b000011110011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod TST_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010000100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TST_r_T2";
    #[inline]
    pub const fn TST_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010100001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod BIC_r_T2_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010001000000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BIC_r_T2_RRX";
    #[inline]
    pub const fn BIC_r_T2_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010100010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod BIC_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BIC_r_T2";
    #[inline]
    pub const fn BIC_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010100010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod BICS_r_T2_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010001100000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BICS_r_T2_RRX";
    #[inline]
    pub const fn BICS_r_T2_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010100011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod BICS_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010001100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BICS_r_T2";
    #[inline]
    pub const fn BICS_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010100011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ORR_r_T2_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010010000000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORR_r_T2_RRX";
    #[inline]
    pub const fn ORR_r_T2_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010100100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ORR_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORR_r_T2";
    #[inline]
    pub const fn ORR_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010100100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ORRS_r_T2_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010010100000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORRS_r_T2_RRX";
    #[inline]
    pub const fn ORRS_r_T2_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010100101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ORRS_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010010100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORRS_r_T2";
    #[inline]
    pub const fn ORRS_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010100101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod MOV_r_T3_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010010011110000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOV_r_T3_RRX";
    #[inline]
    pub const fn MOV_r_T3_RRX(
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101010010011110000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod MOV_r_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010010011110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOV_r_T3";
    #[inline]
    pub const fn MOV_r_T3(
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101010010011110u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod MOVS_r_T3_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010010111110000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOVS_r_T3_RRX";
    #[inline]
    pub const fn MOVS_r_T3_RRX(
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101010010111110000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod MOVS_r_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010010111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOVS_r_T3";
    #[inline]
    pub const fn MOVS_r_T3(
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101010010111110u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ORN_r_T1_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010011000000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORN_r_T1_RRX";
    #[inline]
    pub const fn ORN_r_T1_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010100110u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ORN_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORN_r_T1";
    #[inline]
    pub const fn ORN_r_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010100110u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ORNS_r_T1_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010011100000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORNS_r_T1_RRX";
    #[inline]
    pub const fn ORNS_r_T1_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010100111u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ORNS_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORNS_r_T1";
    #[inline]
    pub const fn ORNS_r_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010100111u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod MVN_r_T2_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010011011110000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MVN_r_T2_RRX";
    #[inline]
    pub const fn MVN_r_T2_RRX(
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101010011011110000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod MVN_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010011011110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MVN_r_T2";
    #[inline]
    pub const fn MVN_r_T2(
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101010011011110u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod MVNS_r_T2_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010011111110000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MVNS_r_T2_RRX";
    #[inline]
    pub const fn MVNS_r_T2_RRX(
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101010011111110000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod MVNS_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010011111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MVNS_r_T2";
    #[inline]
    pub const fn MVNS_r_T2(
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101010011111110u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod EOR_r_T2_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010100000000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "EOR_r_T2_RRX";
    #[inline]
    pub const fn EOR_r_T2_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010101000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod EOR_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "EOR_r_T2";
    #[inline]
    pub const fn EOR_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010101000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod EORS_r_T2_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010100100000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "EORS_r_T2_RRX";
    #[inline]
    pub const fn EORS_r_T2_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010101001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod EORS_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010100100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "EORS_r_T2";
    #[inline]
    pub const fn EORS_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010101001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod TEQ_r_T1_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010100100000000111100110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TEQ_r_T1_RRX";
    #[inline]
    pub const fn TEQ_r_T1_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010101001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b000011110011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod TEQ_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010100100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TEQ_r_T1";
    #[inline]
    pub const fn TEQ_r_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010101001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod PKHBT_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PKHBT_T1";
    #[inline]
    pub const fn PKHBT_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010101100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | 0b00u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod PKHTB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101010110000000000000000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "PKHTB_T1";
    #[inline]
    pub const fn PKHTB_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010101100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | 0b10u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ADD_r_T3_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011000000000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADD_r_T3_RRX";
    #[inline]
    pub const fn ADD_r_T3_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010110000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ADD_r_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADD_r_T3";
    #[inline]
    pub const fn ADD_r_T3(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010110000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ADDS_r_T3_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011000100000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADDS_r_T3_RRX";
    #[inline]
    pub const fn ADDS_r_T3_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010110001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ADDS_r_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011000100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADDS_r_T3";
    #[inline]
    pub const fn ADDS_r_T3(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010110001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ADD_SP_r_T3_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011000011010000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADD_SP_r_T3_RRX";
    #[inline]
    pub const fn ADD_SP_r_T3_RRX(
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101011000011010000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ADD_SP_r_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011000011010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADD_SP_r_T3";
    #[inline]
    pub const fn ADD_SP_r_T3(
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101011000011010u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ADDS_SP_r_T3_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011000111010000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADDS_SP_r_T3_RRX";
    #[inline]
    pub const fn ADDS_SP_r_T3_RRX(
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101011000111010000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ADDS_SP_r_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011000111010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADDS_SP_r_T3";
    #[inline]
    pub const fn ADDS_SP_r_T3(
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101011000111010u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod CMN_r_T2_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011000100000000111100110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMN_r_T2_RRX";
    #[inline]
    pub const fn CMN_r_T2_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010110001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b000011110011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod CMN_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011000100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMN_r_T2";
    #[inline]
    pub const fn CMN_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010110001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ADC_r_T2_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011010000000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADC_r_T2_RRX";
    #[inline]
    pub const fn ADC_r_T2_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010110100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ADC_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADC_r_T2";
    #[inline]
    pub const fn ADC_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010110100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ADCS_r_T2_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011010100000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADCS_r_T2_RRX";
    #[inline]
    pub const fn ADCS_r_T2_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010110101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ADCS_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011010100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADCS_r_T2";
    #[inline]
    pub const fn ADCS_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010110101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SBC_r_T2_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011011000000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SBC_r_T2_RRX";
    #[inline]
    pub const fn SBC_r_T2_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010110110u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SBC_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SBC_r_T2";
    #[inline]
    pub const fn SBC_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010110110u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SBCS_r_T2_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011011100000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SBCS_r_T2_RRX";
    #[inline]
    pub const fn SBCS_r_T2_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010110111u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SBCS_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SBCS_r_T2";
    #[inline]
    pub const fn SBCS_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010110111u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SUB_r_T2_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011101000000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUB_r_T2_RRX";
    #[inline]
    pub const fn SUB_r_T2_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010111010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SUB_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011101000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUB_r_T2";
    #[inline]
    pub const fn SUB_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010111010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SUBS_r_T2_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011101100000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUBS_r_T2_RRX";
    #[inline]
    pub const fn SUBS_r_T2_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010111011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SUBS_r_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011101100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUBS_r_T2";
    #[inline]
    pub const fn SUBS_r_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010111011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SUB_SP_r_T1_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011101011010000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUB_SP_r_T1_RRX";
    #[inline]
    pub const fn SUB_SP_r_T1_RRX(
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101011101011010000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SUB_SP_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011101011010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUB_SP_r_T1";
    #[inline]
    pub const fn SUB_SP_r_T1(
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101011101011010u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SUBS_SP_r_T1_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011101111010000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUBS_SP_r_T1_RRX";
    #[inline]
    pub const fn SUBS_SP_r_T1_RRX(
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101011101111010000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SUBS_SP_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011101111010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUBS_SP_r_T1";
    #[inline]
    pub const fn SUBS_SP_r_T1(
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11101011101111010u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod CMP_r_T3_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011101100000000111100110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMP_r_T3_RRX";
    #[inline]
    pub const fn CMP_r_T3_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010111011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b000011110011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod CMP_r_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011101100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMP_r_T3";
    #[inline]
    pub const fn CMP_r_T3(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010111011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod RSB_r_T1_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011110000000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RSB_r_T1_RRX";
    #[inline]
    pub const fn RSB_r_T1_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010111100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod RSB_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RSB_r_T1";
    #[inline]
    pub const fn RSB_r_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010111100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod RSBS_r_T1_RRX {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011110100000000000000110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RSBS_r_T1_RRX";
    #[inline]
    pub const fn RSBS_r_T1_RRX(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010111101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b0011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod RSBS_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101011110100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RSBS_r_T1";
    #[inline]
    pub const fn RSBS_r_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm2: ::aarchmrs_types::BitValue<2>,
        stype: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010111101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm2.into_inner() << 6u32
                | stype.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
