/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ST1_asisdlso_B1_1b {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST1_asisdlso_B1_1b";
    #[inline]
    pub const fn ST1_asisdlso_B1_1b(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110100000000000u32 << 13u32
                | S.into_inner() << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST3_asisdlso_B3_3b {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101000000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST3_asisdlso_B3_3b";
    #[inline]
    pub const fn ST3_asisdlso_B3_3b(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110100000000001u32 << 13u32
                | S.into_inner() << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST1_asisdlso_H1_1h {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101000000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST1_asisdlso_H1_1h";
    #[inline]
    pub const fn ST1_asisdlso_H1_1h(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110100000000010u32 << 13u32
                | S.into_inner() << 12u32
                | size.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST3_asisdlso_H3_3h {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101000000000110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST3_asisdlso_H3_3h";
    #[inline]
    pub const fn ST3_asisdlso_H3_3h(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110100000000011u32 << 13u32
                | S.into_inner() << 12u32
                | size.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST1_asisdlso_S1_1s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST1_asisdlso_S1_1s";
    #[inline]
    pub const fn ST1_asisdlso_S1_1s(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110100000000100u32 << 13u32
                | S.into_inner() << 12u32
                | 0b00u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST1_asisdlso_D1_1d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101000000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST1_asisdlso_D1_1d";
    #[inline]
    pub const fn ST1_asisdlso_D1_1d(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110100000000100001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST3_asisdlso_S3_3s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101000000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST3_asisdlso_S3_3s";
    #[inline]
    pub const fn ST3_asisdlso_S3_3s(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110100000000101u32 << 13u32
                | S.into_inner() << 12u32
                | 0b00u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST3_asisdlso_D3_3d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101000000001010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST3_asisdlso_D3_3d";
    #[inline]
    pub const fn ST3_asisdlso_D3_3d(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110100000000101001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STL1_asisdlso_D1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101000000011000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STL1_asisdlso_D1";
    #[inline]
    pub const fn STL1_asisdlso_D1(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110100000001100001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST2_asisdlso_B2_2b {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST2_asisdlso_B2_2b";
    #[inline]
    pub const fn ST2_asisdlso_B2_2b(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110100100000000u32 << 13u32
                | S.into_inner() << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST4_asisdlso_B4_4b {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101001000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST4_asisdlso_B4_4b";
    #[inline]
    pub const fn ST4_asisdlso_B4_4b(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110100100000001u32 << 13u32
                | S.into_inner() << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST2_asisdlso_H2_2h {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101001000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST2_asisdlso_H2_2h";
    #[inline]
    pub const fn ST2_asisdlso_H2_2h(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110100100000010u32 << 13u32
                | S.into_inner() << 12u32
                | size.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST4_asisdlso_H4_4h {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101001000000110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST4_asisdlso_H4_4h";
    #[inline]
    pub const fn ST4_asisdlso_H4_4h(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110100100000011u32 << 13u32
                | S.into_inner() << 12u32
                | size.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST2_asisdlso_S2_2s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST2_asisdlso_S2_2s";
    #[inline]
    pub const fn ST2_asisdlso_S2_2s(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110100100000100u32 << 13u32
                | S.into_inner() << 12u32
                | 0b00u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST2_asisdlso_D2_2d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101001000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST2_asisdlso_D2_2d";
    #[inline]
    pub const fn ST2_asisdlso_D2_2d(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110100100000100001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST4_asisdlso_S4_4s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101001000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST4_asisdlso_S4_4s";
    #[inline]
    pub const fn ST4_asisdlso_S4_4s(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110100100000101u32 << 13u32
                | S.into_inner() << 12u32
                | 0b00u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST4_asisdlso_D4_4d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101001000001010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST4_asisdlso_D4_4d";
    #[inline]
    pub const fn ST4_asisdlso_D4_4d(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110100100000101001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD1_asisdlso_B1_1b {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD1_asisdlso_B1_1b";
    #[inline]
    pub const fn LD1_asisdlso_B1_1b(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110101000000000u32 << 13u32
                | S.into_inner() << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD3_asisdlso_B3_3b {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101010000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD3_asisdlso_B3_3b";
    #[inline]
    pub const fn LD3_asisdlso_B3_3b(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110101000000001u32 << 13u32
                | S.into_inner() << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD1_asisdlso_H1_1h {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101010000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD1_asisdlso_H1_1h";
    #[inline]
    pub const fn LD1_asisdlso_H1_1h(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110101000000010u32 << 13u32
                | S.into_inner() << 12u32
                | size.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD3_asisdlso_H3_3h {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101010000000110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD3_asisdlso_H3_3h";
    #[inline]
    pub const fn LD3_asisdlso_H3_3h(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110101000000011u32 << 13u32
                | S.into_inner() << 12u32
                | size.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD1_asisdlso_S1_1s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD1_asisdlso_S1_1s";
    #[inline]
    pub const fn LD1_asisdlso_S1_1s(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110101000000100u32 << 13u32
                | S.into_inner() << 12u32
                | 0b00u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD1_asisdlso_D1_1d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101010000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD1_asisdlso_D1_1d";
    #[inline]
    pub const fn LD1_asisdlso_D1_1d(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110101000000100001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD3_asisdlso_S3_3s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101010000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD3_asisdlso_S3_3s";
    #[inline]
    pub const fn LD3_asisdlso_S3_3s(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110101000000101u32 << 13u32
                | S.into_inner() << 12u32
                | 0b00u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD3_asisdlso_D3_3d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101010000001010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD3_asisdlso_D3_3d";
    #[inline]
    pub const fn LD3_asisdlso_D3_3d(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110101000000101001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD1R_asisdlso_R1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101010000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD1R_asisdlso_R1";
    #[inline]
    pub const fn LD1R_asisdlso_R1(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001101010000001100u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD3R_asisdlso_R3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101010000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD3R_asisdlso_R3";
    #[inline]
    pub const fn LD3R_asisdlso_R3(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001101010000001110u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDAP1_asisdlso_D1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101010000011000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDAP1_asisdlso_D1";
    #[inline]
    pub const fn LDAP1_asisdlso_D1(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110101000001100001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD2_asisdlso_B2_2b {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD2_asisdlso_B2_2b";
    #[inline]
    pub const fn LD2_asisdlso_B2_2b(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110101100000000u32 << 13u32
                | S.into_inner() << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD4_asisdlso_B4_4b {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101011000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD4_asisdlso_B4_4b";
    #[inline]
    pub const fn LD4_asisdlso_B4_4b(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110101100000001u32 << 13u32
                | S.into_inner() << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD2_asisdlso_H2_2h {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101011000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD2_asisdlso_H2_2h";
    #[inline]
    pub const fn LD2_asisdlso_H2_2h(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110101100000010u32 << 13u32
                | S.into_inner() << 12u32
                | size.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD4_asisdlso_H4_4h {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101011000000110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD4_asisdlso_H4_4h";
    #[inline]
    pub const fn LD4_asisdlso_H4_4h(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110101100000011u32 << 13u32
                | S.into_inner() << 12u32
                | size.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD2_asisdlso_S2_2s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101011000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD2_asisdlso_S2_2s";
    #[inline]
    pub const fn LD2_asisdlso_S2_2s(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110101100000100u32 << 13u32
                | S.into_inner() << 12u32
                | 0b00u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD2_asisdlso_D2_2d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101011000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD2_asisdlso_D2_2d";
    #[inline]
    pub const fn LD2_asisdlso_D2_2d(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110101100000100001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD4_asisdlso_S4_4s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101011000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD4_asisdlso_S4_4s";
    #[inline]
    pub const fn LD4_asisdlso_S4_4s(
        Q: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110101100000101u32 << 13u32
                | S.into_inner() << 12u32
                | 0b00u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD4_asisdlso_D4_4d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101011000001010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD4_asisdlso_D4_4d";
    #[inline]
    pub const fn LD4_asisdlso_D4_4d(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00110101100000101001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD2R_asisdlso_R2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101011000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD2R_asisdlso_R2";
    #[inline]
    pub const fn LD2R_asisdlso_R2(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001101011000001100u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD4R_asisdlso_R4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101011000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD4R_asisdlso_R4";
    #[inline]
    pub const fn LD4R_asisdlso_R4(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001101011000001110u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
