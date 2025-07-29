/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod VLD1_a_A1_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110100101000000000110000001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD1_a_A1_nowb";
    #[inline]
    pub const fn VLD1_a_A1_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        T: ::aarchmrs_types::BitValue<1>,
        a: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111101001u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1100u32 << 8u32
                | size.into_inner() << 6u32
                | T.into_inner() << 5u32
                | a.into_inner() << 4u32
                | 0b1111u32 << 0u32,
        )
    }
}
pub mod VLD1_a_A1_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110100101000000000110000001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD1_a_A1_posti";
    #[inline]
    pub const fn VLD1_a_A1_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        T: ::aarchmrs_types::BitValue<1>,
        a: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111101001u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1100u32 << 8u32
                | size.into_inner() << 6u32
                | T.into_inner() << 5u32
                | a.into_inner() << 4u32
                | 0b1101u32 << 0u32,
        )
    }
}
pub mod VLD1_a_A1_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110100101000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD1_a_A1_postr";
    #[inline]
    pub const fn VLD1_a_A1_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        T: ::aarchmrs_types::BitValue<1>,
        a: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111101001u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1100u32 << 8u32
                | size.into_inner() << 6u32
                | T.into_inner() << 5u32
                | a.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod VLD2_a_A1_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110100101000000000110100001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD2_a_A1_nowb";
    #[inline]
    pub const fn VLD2_a_A1_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        T: ::aarchmrs_types::BitValue<1>,
        a: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111101001u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1101u32 << 8u32
                | size.into_inner() << 6u32
                | T.into_inner() << 5u32
                | a.into_inner() << 4u32
                | 0b1111u32 << 0u32,
        )
    }
}
pub mod VLD2_a_A1_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110100101000000000110100001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD2_a_A1_posti";
    #[inline]
    pub const fn VLD2_a_A1_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        T: ::aarchmrs_types::BitValue<1>,
        a: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111101001u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1101u32 << 8u32
                | size.into_inner() << 6u32
                | T.into_inner() << 5u32
                | a.into_inner() << 4u32
                | 0b1101u32 << 0u32,
        )
    }
}
pub mod VLD2_a_A1_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110100101000000000110100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD2_a_A1_postr";
    #[inline]
    pub const fn VLD2_a_A1_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        T: ::aarchmrs_types::BitValue<1>,
        a: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111101001u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1101u32 << 8u32
                | size.into_inner() << 6u32
                | T.into_inner() << 5u32
                | a.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod VLD3_a_A1_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110100101000000000111000001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD3_a_A1_nowb";
    #[inline]
    pub const fn VLD3_a_A1_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        T: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111101001u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1110u32 << 8u32
                | size.into_inner() << 6u32
                | T.into_inner() << 5u32
                | 0b01111u32 << 0u32,
        )
    }
}
pub mod VLD3_a_A1_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110100101000000000111000001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD3_a_A1_posti";
    #[inline]
    pub const fn VLD3_a_A1_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        T: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111101001u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1110u32 << 8u32
                | size.into_inner() << 6u32
                | T.into_inner() << 5u32
                | 0b01101u32 << 0u32,
        )
    }
}
pub mod VLD3_a_A1_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110100101000000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD3_a_A1_postr";
    #[inline]
    pub const fn VLD3_a_A1_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        T: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111101001u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1110u32 << 8u32
                | size.into_inner() << 6u32
                | T.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod VLD4_a_A1_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110100101000000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD4_a_A1_nowb";
    #[inline]
    pub const fn VLD4_a_A1_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        T: ::aarchmrs_types::BitValue<1>,
        a: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111101001u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | size.into_inner() << 6u32
                | T.into_inner() << 5u32
                | a.into_inner() << 4u32
                | 0b1111u32 << 0u32,
        )
    }
}
pub mod VLD4_a_A1_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110100101000000000111100001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD4_a_A1_posti";
    #[inline]
    pub const fn VLD4_a_A1_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        T: ::aarchmrs_types::BitValue<1>,
        a: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111101001u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | size.into_inner() << 6u32
                | T.into_inner() << 5u32
                | a.into_inner() << 4u32
                | 0b1101u32 << 0u32,
        )
    }
}
pub mod VLD4_a_A1_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110100101000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD4_a_A1_postr";
    #[inline]
    pub const fn VLD4_a_A1_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        T: ::aarchmrs_types::BitValue<1>,
        a: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111101001u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | size.into_inner() << 6u32
                | T.into_inner() << 5u32
                | a.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
