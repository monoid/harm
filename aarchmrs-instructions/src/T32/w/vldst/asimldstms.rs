/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod VST4_m_T1_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111000001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000000000001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST4_m_T1_nowb";
    #[inline]
    pub const fn VST4_m_T1_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        itype: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b000u32 << 9u32
                | itype.into_inner() << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1111u32 << 0u32,
        )
    }
}
pub mod VST4_m_T1_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111000001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000000000001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST4_m_T1_posti";
    #[inline]
    pub const fn VST4_m_T1_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        itype: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b000u32 << 9u32
                | itype.into_inner() << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1101u32 << 0u32,
        )
    }
}
pub mod VST4_m_T1_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST4_m_T1_postr";
    #[inline]
    pub const fn VST4_m_T1_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        itype: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b000u32 << 9u32
                | itype.into_inner() << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod VST1_m_T4_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000001000001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST1_m_T4_nowb";
    #[inline]
    pub const fn VST1_m_T4_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0010u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1111u32 << 0u32,
        )
    }
}
pub mod VST1_m_T4_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000001000001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST1_m_T4_posti";
    #[inline]
    pub const fn VST1_m_T4_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0010u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1101u32 << 0u32,
        )
    }
}
pub mod VST1_m_T4_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST1_m_T4_postr";
    #[inline]
    pub const fn VST1_m_T4_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0010u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod VST2_m_T2_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000001100001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST2_m_T2_nowb";
    #[inline]
    pub const fn VST2_m_T2_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0011u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1111u32 << 0u32,
        )
    }
}
pub mod VST2_m_T2_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000001100001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST2_m_T2_posti";
    #[inline]
    pub const fn VST2_m_T2_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0011u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1101u32 << 0u32,
        )
    }
}
pub mod VST2_m_T2_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000001100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST2_m_T2_postr";
    #[inline]
    pub const fn VST2_m_T2_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0011u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod VST3_m_T1_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111000001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000010000001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST3_m_T1_nowb";
    #[inline]
    pub const fn VST3_m_T1_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        itype: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b010u32 << 9u32
                | itype.into_inner() << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1111u32 << 0u32,
        )
    }
}
pub mod VST3_m_T1_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111000001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000010000001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST3_m_T1_posti";
    #[inline]
    pub const fn VST3_m_T1_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        itype: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b010u32 << 9u32
                | itype.into_inner() << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1101u32 << 0u32,
        )
    }
}
pub mod VST3_m_T1_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST3_m_T1_postr";
    #[inline]
    pub const fn VST3_m_T1_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        itype: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b010u32 << 9u32
                | itype.into_inner() << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod VST1_m_T3_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000011000001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST1_m_T3_nowb";
    #[inline]
    pub const fn VST1_m_T3_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0110u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1111u32 << 0u32,
        )
    }
}
pub mod VST1_m_T3_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000011000001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST1_m_T3_posti";
    #[inline]
    pub const fn VST1_m_T3_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0110u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1101u32 << 0u32,
        )
    }
}
pub mod VST1_m_T3_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000011000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST1_m_T3_postr";
    #[inline]
    pub const fn VST1_m_T3_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0110u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod VST1_m_T1_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000011100001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST1_m_T1_nowb";
    #[inline]
    pub const fn VST1_m_T1_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0111u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1111u32 << 0u32,
        )
    }
}
pub mod VST1_m_T1_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000011100001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST1_m_T1_posti";
    #[inline]
    pub const fn VST1_m_T1_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0111u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1101u32 << 0u32,
        )
    }
}
pub mod VST1_m_T1_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000011100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST1_m_T1_postr";
    #[inline]
    pub const fn VST1_m_T1_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0111u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod VST2_m_T1_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111000001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000100000001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST2_m_T1_nowb";
    #[inline]
    pub const fn VST2_m_T1_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        itype: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b100u32 << 9u32
                | itype.into_inner() << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1111u32 << 0u32,
        )
    }
}
pub mod VST2_m_T1_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111000001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000100000001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST2_m_T1_posti";
    #[inline]
    pub const fn VST2_m_T1_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        itype: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b100u32 << 9u32
                | itype.into_inner() << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1101u32 << 0u32,
        )
    }
}
pub mod VST2_m_T1_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST2_m_T1_postr";
    #[inline]
    pub const fn VST2_m_T1_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        itype: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b100u32 << 9u32
                | itype.into_inner() << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod VST1_m_T2_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000101000001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST1_m_T2_nowb";
    #[inline]
    pub const fn VST1_m_T2_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1111u32 << 0u32,
        )
    }
}
pub mod VST1_m_T2_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000101000001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST1_m_T2_posti";
    #[inline]
    pub const fn VST1_m_T2_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1101u32 << 0u32,
        )
    }
}
pub mod VST1_m_T2_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001000000000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VST1_m_T2_postr";
    #[inline]
    pub const fn VST1_m_T2_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod VLD4_m_T1_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111000001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000000000001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD4_m_T1_nowb";
    #[inline]
    pub const fn VLD4_m_T1_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        itype: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b000u32 << 9u32
                | itype.into_inner() << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1111u32 << 0u32,
        )
    }
}
pub mod VLD4_m_T1_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111000001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000000000001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD4_m_T1_posti";
    #[inline]
    pub const fn VLD4_m_T1_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        itype: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b000u32 << 9u32
                | itype.into_inner() << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1101u32 << 0u32,
        )
    }
}
pub mod VLD4_m_T1_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD4_m_T1_postr";
    #[inline]
    pub const fn VLD4_m_T1_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        itype: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b000u32 << 9u32
                | itype.into_inner() << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod VLD1_m_T4_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000001000001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD1_m_T4_nowb";
    #[inline]
    pub const fn VLD1_m_T4_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0010u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1111u32 << 0u32,
        )
    }
}
pub mod VLD1_m_T4_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000001000001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD1_m_T4_posti";
    #[inline]
    pub const fn VLD1_m_T4_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0010u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1101u32 << 0u32,
        )
    }
}
pub mod VLD1_m_T4_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD1_m_T4_postr";
    #[inline]
    pub const fn VLD1_m_T4_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0010u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod VLD2_m_T2_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000001100001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD2_m_T2_nowb";
    #[inline]
    pub const fn VLD2_m_T2_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0011u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1111u32 << 0u32,
        )
    }
}
pub mod VLD2_m_T2_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000001100001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD2_m_T2_posti";
    #[inline]
    pub const fn VLD2_m_T2_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0011u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1101u32 << 0u32,
        )
    }
}
pub mod VLD2_m_T2_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000001100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD2_m_T2_postr";
    #[inline]
    pub const fn VLD2_m_T2_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0011u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod VLD3_m_T1_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111000001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000010000001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD3_m_T1_nowb";
    #[inline]
    pub const fn VLD3_m_T1_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        itype: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b010u32 << 9u32
                | itype.into_inner() << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1111u32 << 0u32,
        )
    }
}
pub mod VLD3_m_T1_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111000001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000010000001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD3_m_T1_posti";
    #[inline]
    pub const fn VLD3_m_T1_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        itype: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b010u32 << 9u32
                | itype.into_inner() << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1101u32 << 0u32,
        )
    }
}
pub mod VLD3_m_T1_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD3_m_T1_postr";
    #[inline]
    pub const fn VLD3_m_T1_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        itype: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b010u32 << 9u32
                | itype.into_inner() << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod VLD1_m_T3_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000011000001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD1_m_T3_nowb";
    #[inline]
    pub const fn VLD1_m_T3_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0110u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1111u32 << 0u32,
        )
    }
}
pub mod VLD1_m_T3_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000011000001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD1_m_T3_posti";
    #[inline]
    pub const fn VLD1_m_T3_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0110u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1101u32 << 0u32,
        )
    }
}
pub mod VLD1_m_T3_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000011000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD1_m_T3_postr";
    #[inline]
    pub const fn VLD1_m_T3_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0110u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod VLD1_m_T1_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000011100001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD1_m_T1_nowb";
    #[inline]
    pub const fn VLD1_m_T1_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0111u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1111u32 << 0u32,
        )
    }
}
pub mod VLD1_m_T1_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000011100001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD1_m_T1_posti";
    #[inline]
    pub const fn VLD1_m_T1_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0111u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1101u32 << 0u32,
        )
    }
}
pub mod VLD1_m_T1_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000011100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD1_m_T1_postr";
    #[inline]
    pub const fn VLD1_m_T1_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0111u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod VLD2_m_T1_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111000001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000100000001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD2_m_T1_nowb";
    #[inline]
    pub const fn VLD2_m_T1_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        itype: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b100u32 << 9u32
                | itype.into_inner() << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1111u32 << 0u32,
        )
    }
}
pub mod VLD2_m_T1_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111000001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000100000001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD2_m_T1_posti";
    #[inline]
    pub const fn VLD2_m_T1_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        itype: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b100u32 << 9u32
                | itype.into_inner() << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1101u32 << 0u32,
        )
    }
}
pub mod VLD2_m_T1_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD2_m_T1_postr";
    #[inline]
    pub const fn VLD2_m_T1_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        itype: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b100u32 << 9u32
                | itype.into_inner() << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod VLD1_m_T2_nowb {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000101000001111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD1_m_T2_nowb";
    #[inline]
    pub const fn VLD1_m_T2_nowb(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1111u32 << 0u32,
        )
    }
}
pub mod VLD1_m_T2_posti {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100001111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000101000001101u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD1_m_T2_posti";
    #[inline]
    pub const fn VLD1_m_T2_posti(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | 0b1101u32 << 0u32,
        )
    }
}
pub mod VLD1_m_T2_postr {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111001001000000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLD1_m_T2_postr";
    #[inline]
    pub const fn VLD1_m_T2_postr(
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        size: ::aarchmrs_types::BitValue<2>,
        align: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | size.into_inner() << 6u32
                | align.into_inner() << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
