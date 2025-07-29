/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod add_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "add_z_zi_";
    #[inline]
    pub const fn add_z_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        sh: ::aarchmrs_types::BitValue<1>,
        imm8: ::aarchmrs_types::BitValue<8>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10000011u32 << 14u32
                | sh.into_inner() << 13u32
                | imm8.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod sub_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000011100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sub_z_zi_";
    #[inline]
    pub const fn sub_z_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        sh: ::aarchmrs_types::BitValue<1>,
        imm8: ::aarchmrs_types::BitValue<8>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10000111u32 << 14u32
                | sh.into_inner() << 13u32
                | imm8.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod subr_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "subr_z_zi_";
    #[inline]
    pub const fn subr_z_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        sh: ::aarchmrs_types::BitValue<1>,
        imm8: ::aarchmrs_types::BitValue<8>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10001111u32 << 14u32
                | sh.into_inner() << 13u32
                | imm8.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod sqadd_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001001001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqadd_z_zi_";
    #[inline]
    pub const fn sqadd_z_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        U: ::aarchmrs_types::BitValue<1>,
        sh: ::aarchmrs_types::BitValue<1>,
        imm8: ::aarchmrs_types::BitValue<8>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10010u32 << 17u32
                | U.into_inner() << 16u32
                | 0b11u32 << 14u32
                | sh.into_inner() << 13u32
                | imm8.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod sqsub_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001001101100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqsub_z_zi_";
    #[inline]
    pub const fn sqsub_z_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        U: ::aarchmrs_types::BitValue<1>,
        sh: ::aarchmrs_types::BitValue<1>,
        imm8: ::aarchmrs_types::BitValue<8>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10011u32 << 17u32
                | U.into_inner() << 16u32
                | 0b11u32 << 14u32
                | sh.into_inner() << 13u32
                | imm8.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod uqadd_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001001001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqadd_z_zi_";
    #[inline]
    pub const fn uqadd_z_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        U: ::aarchmrs_types::BitValue<1>,
        sh: ::aarchmrs_types::BitValue<1>,
        imm8: ::aarchmrs_types::BitValue<8>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10010u32 << 17u32
                | U.into_inner() << 16u32
                | 0b11u32 << 14u32
                | sh.into_inner() << 13u32
                | imm8.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod uqsub_z_zi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001001101100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqsub_z_zi_";
    #[inline]
    pub const fn uqsub_z_zi_(
        size: ::aarchmrs_types::BitValue<2>,
        U: ::aarchmrs_types::BitValue<1>,
        sh: ::aarchmrs_types::BitValue<1>,
        imm8: ::aarchmrs_types::BitValue<8>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10011u32 << 17u32
                | U.into_inner() << 16u32
                | 0b11u32 << 14u32
                | sh.into_inner() << 13u32
                | imm8.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
