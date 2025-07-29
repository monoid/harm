/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod AND_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AND_i_T1";
    #[inline]
    pub const fn AND_i_T1(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b000000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod ANDS_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110000000100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ANDS_i_T1";
    #[inline]
    pub const fn ANDS_i_T1(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b000001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod TST_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110000000100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TST_i_T1";
    #[inline]
    pub const fn TST_i_T1(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b000001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod BIC_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110000001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BIC_i_T1";
    #[inline]
    pub const fn BIC_i_T1(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b000010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod BICS_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110000001100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BICS_i_T1";
    #[inline]
    pub const fn BICS_i_T1(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b000011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod ORR_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110000010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORR_i_T1";
    #[inline]
    pub const fn ORR_i_T1(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b000100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod ORRS_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110000010100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORRS_i_T1";
    #[inline]
    pub const fn ORRS_i_T1(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b000101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod MOV_i_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110000010011110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOV_i_T2";
    #[inline]
    pub const fn MOV_i_T2(
        i: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b00010011110u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod MOVS_i_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110000010111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOVS_i_T2";
    #[inline]
    pub const fn MOVS_i_T2(
        i: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b00010111110u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod ORNS_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110000011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORNS_i_T1";
    #[inline]
    pub const fn ORNS_i_T1(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b000111u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod ORN_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110000011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORN_i_T1";
    #[inline]
    pub const fn ORN_i_T1(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b000110u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod MVN_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110000011011110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MVN_i_T1";
    #[inline]
    pub const fn MVN_i_T1(
        i: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b00011011110u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod MVNS_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110000011111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MVNS_i_T1";
    #[inline]
    pub const fn MVNS_i_T1(
        i: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b00011111110u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod EOR_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110000100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "EOR_i_T1";
    #[inline]
    pub const fn EOR_i_T1(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b001000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod EORS_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110000100100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "EORS_i_T1";
    #[inline]
    pub const fn EORS_i_T1(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b001001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod TEQ_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110000100100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TEQ_i_T1";
    #[inline]
    pub const fn TEQ_i_T1(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b001001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod ADD_i_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADD_i_T3";
    #[inline]
    pub const fn ADD_i_T3(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b010000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod ADDS_i_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001000100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADDS_i_T3";
    #[inline]
    pub const fn ADDS_i_T3(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b010001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod ADD_SP_i_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001000011010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADD_SP_i_T3";
    #[inline]
    pub const fn ADD_SP_i_T3(
        i: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b01000011010u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod ADDS_SP_i_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001000111010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADDS_SP_i_T3";
    #[inline]
    pub const fn ADDS_SP_i_T3(
        i: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b01000111010u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod CMN_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001000100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMN_i_T1";
    #[inline]
    pub const fn CMN_i_T1(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b010001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod ADC_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADC_i_T1";
    #[inline]
    pub const fn ADC_i_T1(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b010100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod ADCS_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001010100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADCS_i_T1";
    #[inline]
    pub const fn ADCS_i_T1(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b010101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod SBC_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SBC_i_T1";
    #[inline]
    pub const fn SBC_i_T1(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b010110u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod SBCS_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SBCS_i_T1";
    #[inline]
    pub const fn SBCS_i_T1(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b010111u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod SUB_i_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001101000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUB_i_T3";
    #[inline]
    pub const fn SUB_i_T3(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b011010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod SUBS_i_T3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001101100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUBS_i_T3";
    #[inline]
    pub const fn SUBS_i_T3(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b011011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod SUB_SP_i_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001101011010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUB_SP_i_T2";
    #[inline]
    pub const fn SUB_SP_i_T2(
        i: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b01101011010u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod SUBS_SP_i_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001101111010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUBS_SP_i_T2";
    #[inline]
    pub const fn SUBS_SP_i_T2(
        i: ::aarchmrs_types::BitValue<1>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b01101111010u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod CMP_i_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001101100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMP_i_T2";
    #[inline]
    pub const fn CMP_i_T2(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b011011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod RSB_i_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RSB_i_T2";
    #[inline]
    pub const fn RSB_i_T2(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b011100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod RSBS_i_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111011111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001110100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RSBS_i_T2";
    #[inline]
    pub const fn RSBS_i_T2(
        i: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110u32 << 27u32
                | i.into_inner() << 26u32
                | 0b011101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b0u32 << 15u32
                | imm3.into_inner() << 12u32
                | Rd.into_inner() << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
