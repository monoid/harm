/* Copyright (c) 2010-2026 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CFLTGT_32_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111001000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110110000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CFLTGT_32_imm";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_WIDTH: u32 = 5u32;
    #[inline]
    pub const fn CFLTGT_32_imm(
        imm9h: ::aarchmrs_types::BitValue<5>,
        imm2: ::aarchmrs_types::BitValue<2>,
        imm9l: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110110000u32 << 21u32
                | imm9h.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm2.into_inner() << 10u32
                | 0b0u32 << 9u32
                | imm9l.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CFLTLT_32_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111001000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110110001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CFLTLT_32_imm";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_WIDTH: u32 = 5u32;
    #[inline]
    pub const fn CFLTLT_32_imm(
        imm9h: ::aarchmrs_types::BitValue<5>,
        imm2: ::aarchmrs_types::BitValue<2>,
        imm9l: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110110001u32 << 21u32
                | imm9h.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm2.into_inner() << 10u32
                | 0b0u32 << 9u32
                | imm9l.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CFLTHI_32_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111001000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110110010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CFLTHI_32_imm";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_WIDTH: u32 = 5u32;
    #[inline]
    pub const fn CFLTHI_32_imm(
        imm9h: ::aarchmrs_types::BitValue<5>,
        imm2: ::aarchmrs_types::BitValue<2>,
        imm9l: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110110010u32 << 21u32
                | imm9h.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm2.into_inner() << 10u32
                | 0b0u32 << 9u32
                | imm9l.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CFLTLO_32_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111001000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110110011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CFLTLO_32_imm";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_WIDTH: u32 = 5u32;
    #[inline]
    pub const fn CFLTLO_32_imm(
        imm9h: ::aarchmrs_types::BitValue<5>,
        imm2: ::aarchmrs_types::BitValue<2>,
        imm9l: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110110011u32 << 21u32
                | imm9h.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm2.into_inner() << 10u32
                | 0b0u32 << 9u32
                | imm9l.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CFLTEQ_32_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111001000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110110100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CFLTEQ_32_imm";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_WIDTH: u32 = 5u32;
    #[inline]
    pub const fn CFLTEQ_32_imm(
        imm9h: ::aarchmrs_types::BitValue<5>,
        imm2: ::aarchmrs_types::BitValue<2>,
        imm9l: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110110100u32 << 21u32
                | imm9h.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm2.into_inner() << 10u32
                | 0b0u32 << 9u32
                | imm9l.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CFLTNE_32_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111001000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01110110101000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CFLTNE_32_imm";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_WIDTH: u32 = 5u32;
    #[inline]
    pub const fn CFLTNE_32_imm(
        imm9h: ::aarchmrs_types::BitValue<5>,
        imm2: ::aarchmrs_types::BitValue<2>,
        imm9l: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01110110101u32 << 21u32
                | imm9h.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm2.into_inner() << 10u32
                | 0b0u32 << 9u32
                | imm9l.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CFLTGT_64_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111001000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110110000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CFLTGT_64_imm";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_WIDTH: u32 = 5u32;
    #[inline]
    pub const fn CFLTGT_64_imm(
        imm9h: ::aarchmrs_types::BitValue<5>,
        imm2: ::aarchmrs_types::BitValue<2>,
        imm9l: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110110000u32 << 21u32
                | imm9h.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm2.into_inner() << 10u32
                | 0b0u32 << 9u32
                | imm9l.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CFLTLT_64_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111001000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110110001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CFLTLT_64_imm";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_WIDTH: u32 = 5u32;
    #[inline]
    pub const fn CFLTLT_64_imm(
        imm9h: ::aarchmrs_types::BitValue<5>,
        imm2: ::aarchmrs_types::BitValue<2>,
        imm9l: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110110001u32 << 21u32
                | imm9h.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm2.into_inner() << 10u32
                | 0b0u32 << 9u32
                | imm9l.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CFLTHI_64_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111001000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110110010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CFLTHI_64_imm";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_WIDTH: u32 = 5u32;
    #[inline]
    pub const fn CFLTHI_64_imm(
        imm9h: ::aarchmrs_types::BitValue<5>,
        imm2: ::aarchmrs_types::BitValue<2>,
        imm9l: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110110010u32 << 21u32
                | imm9h.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm2.into_inner() << 10u32
                | 0b0u32 << 9u32
                | imm9l.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CFLTLO_64_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111001000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110110011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CFLTLO_64_imm";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_WIDTH: u32 = 5u32;
    #[inline]
    pub const fn CFLTLO_64_imm(
        imm9h: ::aarchmrs_types::BitValue<5>,
        imm2: ::aarchmrs_types::BitValue<2>,
        imm9l: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110110011u32 << 21u32
                | imm9h.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm2.into_inner() << 10u32
                | 0b0u32 << 9u32
                | imm9l.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CFLTEQ_64_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111001000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110110100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CFLTEQ_64_imm";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_WIDTH: u32 = 5u32;
    #[inline]
    pub const fn CFLTEQ_64_imm(
        imm9h: ::aarchmrs_types::BitValue<5>,
        imm2: ::aarchmrs_types::BitValue<2>,
        imm9l: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110110100u32 << 21u32
                | imm9h.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm2.into_inner() << 10u32
                | 0b0u32 << 9u32
                | imm9l.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod CFLTNE_64_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111001000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110110101000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CFLTNE_64_imm";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rt_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9l_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm2_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_imm9h_WIDTH: u32 = 5u32;
    #[inline]
    pub const fn CFLTNE_64_imm(
        imm9h: ::aarchmrs_types::BitValue<5>,
        imm2: ::aarchmrs_types::BitValue<2>,
        imm9l: ::aarchmrs_types::BitValue<4>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110110101u32 << 21u32
                | imm9h.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | imm2.into_inner() << 10u32
                | 0b0u32 << 9u32
                | imm9l.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
