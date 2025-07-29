/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod dpint_2r_unpred_0;
pub mod dpint_2r_unpred_1;
pub mod QADD_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100000001111000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "QADD_T1";
    #[inline]
    pub const fn QADD_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b1000u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod QDADD_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100000001111000010010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "QDADD_T1";
    #[inline]
    pub const fn QDADD_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b1001u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod QSUB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100000001111000010100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "QSUB_T1";
    #[inline]
    pub const fn QSUB_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b1010u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod QDSUB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100000001111000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "QDSUB_T1";
    #[inline]
    pub const fn QDSUB_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101000u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b1011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod REV_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100100001111000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "REV_T2";
    #[inline]
    pub const fn REV_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b1000u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod REV16_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100100001111000010010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "REV16_T2";
    #[inline]
    pub const fn REV16_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b1001u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod RBIT_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100100001111000010100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RBIT_T1";
    #[inline]
    pub const fn RBIT_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b1010u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod REVSH_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010100100001111000010110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "REVSH_T2";
    #[inline]
    pub const fn REVSH_T2(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b1011u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod SEL_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010101000001111000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SEL_T1";
    #[inline]
    pub const fn SEL_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101010u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b1000u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod CLZ_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010101100001111000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CLZ_T1";
    #[inline]
    pub const fn CLZ_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b1000u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod CRC32B_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010110000001111000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CRC32B_T1";
    #[inline]
    pub const fn CRC32B_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b1000u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod CRC32H_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010110000001111000010010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CRC32H_T1";
    #[inline]
    pub const fn CRC32H_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b1001u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod CRC32W_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010110000001111000010100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CRC32W_T1";
    #[inline]
    pub const fn CRC32W_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101100u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b1010u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod CRC32CB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010110100001111000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CRC32CB_T1";
    #[inline]
    pub const fn CRC32CB_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b1000u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod CRC32CH_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010110100001111000010010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CRC32CH_T1";
    #[inline]
    pub const fn CRC32CH_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b1001u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod CRC32CW_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111010110100001111000010100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CRC32CW_T1";
    #[inline]
    pub const fn CRC32CW_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110101101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b1010u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
