/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqrshrn_z_mz2_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101101100000010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqrshrn_z_mz2_";
    #[inline]
    pub const fn sqrshrn_z_mz2_(
        imm4: ::aarchmrs_types::BitValue<4>,
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001011011u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b001010u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqrshrn_z_mz2_b {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111110001111110000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101101010000010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqrshrn_z_mz2_b";
    #[inline]
    pub const fn sqrshrn_z_mz2_b(
        imm3: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100010110101u32 << 19u32
                | imm3.into_inner() << 16u32
                | 0b001010u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqrshrun_z_mz2_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101101100000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqrshrun_z_mz2_";
    #[inline]
    pub const fn sqrshrun_z_mz2_(
        imm4: ::aarchmrs_types::BitValue<4>,
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001011011u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b000010u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqrshrun_z_mz2_b {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111110001111110000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101101010000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqrshrun_z_mz2_b";
    #[inline]
    pub const fn sqrshrun_z_mz2_b(
        imm3: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100010110101u32 << 19u32
                | imm3.into_inner() << 16u32
                | 0b000010u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqshrn_z_mz2_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101101000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqshrn_z_mz2_";
    #[inline]
    pub const fn sqshrn_z_mz2_(
        tsize: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101101u32 << 21u32
                | tsize.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b000000u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqshrun_z_mz2_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101101000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqshrun_z_mz2_";
    #[inline]
    pub const fn sqshrun_z_mz2_(
        tsize: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101101u32 << 21u32
                | tsize.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b001000u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uqrshrn_z_mz2_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111110000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101101100000011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqrshrn_z_mz2_";
    #[inline]
    pub const fn uqrshrn_z_mz2_(
        imm4: ::aarchmrs_types::BitValue<4>,
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001011011u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b001110u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uqrshrn_z_mz2_b {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111110001111110000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101101010000011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqrshrn_z_mz2_b";
    #[inline]
    pub const fn uqrshrn_z_mz2_b(
        imm3: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100010110101u32 << 19u32
                | imm3.into_inner() << 16u32
                | 0b001110u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uqshrn_z_mz2_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101101000000001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqshrn_z_mz2_";
    #[inline]
    pub const fn uqshrn_z_mz2_(
        tsize: ::aarchmrs_types::BitValue<2>,
        imm3: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000101101u32 << 21u32
                | tsize.into_inner() << 19u32
                | imm3.into_inner() << 16u32
                | 0b000100u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
