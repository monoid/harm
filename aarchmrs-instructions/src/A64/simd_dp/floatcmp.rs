/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FCMP_S_floatcmp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMP_S_floatcmp";
    #[inline]
    pub const fn FCMP_S_floatcmp(
        Rm: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        opc: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b001000u32 << 10u32
                | Rn.into_inner() << 5u32
                | opc.into_inner() << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMP_SZ_floatcmp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMP_SZ_floatcmp";
    #[inline]
    pub const fn FCMP_SZ_floatcmp(
        Rn: ::aarchmrs_types::BitValue<5>,
        opc: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000100000001000u32 << 10u32
                | Rn.into_inner() << 5u32
                | opc.into_inner() << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMPE_S_floatcmp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMPE_S_floatcmp";
    #[inline]
    pub const fn FCMPE_S_floatcmp(
        Rm: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        opc: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b001000u32 << 10u32
                | Rn.into_inner() << 5u32
                | opc.into_inner() << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMPE_SZ_floatcmp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMPE_SZ_floatcmp";
    #[inline]
    pub const fn FCMPE_SZ_floatcmp(
        Rn: ::aarchmrs_types::BitValue<5>,
        opc: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000100000001000u32 << 10u32
                | Rn.into_inner() << 5u32
                | opc.into_inner() << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMP_D_floatcmp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMP_D_floatcmp";
    #[inline]
    pub const fn FCMP_D_floatcmp(
        Rm: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        opc: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b001000u32 << 10u32
                | Rn.into_inner() << 5u32
                | opc.into_inner() << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMP_DZ_floatcmp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMP_DZ_floatcmp";
    #[inline]
    pub const fn FCMP_DZ_floatcmp(
        Rn: ::aarchmrs_types::BitValue<5>,
        opc: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001100000001000u32 << 10u32
                | Rn.into_inner() << 5u32
                | opc.into_inner() << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMPE_D_floatcmp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMPE_D_floatcmp";
    #[inline]
    pub const fn FCMPE_D_floatcmp(
        Rm: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        opc: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b001000u32 << 10u32
                | Rn.into_inner() << 5u32
                | opc.into_inner() << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMPE_DZ_floatcmp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMPE_DZ_floatcmp";
    #[inline]
    pub const fn FCMPE_DZ_floatcmp(
        Rn: ::aarchmrs_types::BitValue<5>,
        opc: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001100000001000u32 << 10u32
                | Rn.into_inner() << 5u32
                | opc.into_inner() << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMP_H_floatcmp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMP_H_floatcmp";
    #[inline]
    pub const fn FCMP_H_floatcmp(
        Rm: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        opc: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b001000u32 << 10u32
                | Rn.into_inner() << 5u32
                | opc.into_inner() << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMP_HZ_floatcmp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMP_HZ_floatcmp";
    #[inline]
    pub const fn FCMP_HZ_floatcmp(
        Rn: ::aarchmrs_types::BitValue<5>,
        opc: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011100000001000u32 << 10u32
                | Rn.into_inner() << 5u32
                | opc.into_inner() << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMPE_H_floatcmp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMPE_H_floatcmp";
    #[inline]
    pub const fn FCMPE_H_floatcmp(
        Rm: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        opc: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b001000u32 << 10u32
                | Rn.into_inner() << 5u32
                | opc.into_inner() << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMPE_HZ_floatcmp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMPE_HZ_floatcmp";
    #[inline]
    pub const fn FCMPE_HZ_floatcmp(
        Rn: ::aarchmrs_types::BitValue<5>,
        opc: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011100000001000u32 << 10u32
                | Rn.into_inner() << 5u32
                | opc.into_inner() << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
