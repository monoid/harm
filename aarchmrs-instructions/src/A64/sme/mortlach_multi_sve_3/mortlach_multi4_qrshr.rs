/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqrshr_z_mz4_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111110001100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001101100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqrshr_z_mz4_";
    #[inline]
    pub const fn sqrshr_z_mz4_(
        tsize: ::aarchmrs_types::BitValue<2>,
        imm5: ::aarchmrs_types::BitValue<5>,
        Zn: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | tsize.into_inner() << 22u32
                | 0b1u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b110110u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b00u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqrshru_z_mz4_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111110001100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001101100001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqrshru_z_mz4_";
    #[inline]
    pub const fn sqrshru_z_mz4_(
        tsize: ::aarchmrs_types::BitValue<2>,
        imm5: ::aarchmrs_types::BitValue<5>,
        Zn: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | tsize.into_inner() << 22u32
                | 0b1u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b110110u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b10u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqrshrn_z_mz4_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111110001100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001101110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqrshrn_z_mz4_";
    #[inline]
    pub const fn sqrshrn_z_mz4_(
        tsize: ::aarchmrs_types::BitValue<2>,
        imm5: ::aarchmrs_types::BitValue<5>,
        Zn: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | tsize.into_inner() << 22u32
                | 0b1u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b110111u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b00u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqrshrun_z_mz4_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111110001100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001101110001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqrshrun_z_mz4_";
    #[inline]
    pub const fn sqrshrun_z_mz4_(
        tsize: ::aarchmrs_types::BitValue<2>,
        imm5: ::aarchmrs_types::BitValue<5>,
        Zn: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | tsize.into_inner() << 22u32
                | 0b1u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b110111u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b10u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uqrshr_z_mz4_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111110001100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001101100000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqrshr_z_mz4_";
    #[inline]
    pub const fn uqrshr_z_mz4_(
        tsize: ::aarchmrs_types::BitValue<2>,
        imm5: ::aarchmrs_types::BitValue<5>,
        Zn: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | tsize.into_inner() << 22u32
                | 0b1u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b110110u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b01u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uqrshrn_z_mz4_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111110001100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001101110000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqrshrn_z_mz4_";
    #[inline]
    pub const fn uqrshrn_z_mz4_(
        tsize: ::aarchmrs_types::BitValue<2>,
        imm5: ::aarchmrs_types::BitValue<5>,
        Zn: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | tsize.into_inner() << 22u32
                | 0b1u32 << 21u32
                | imm5.into_inner() << 16u32
                | 0b110111u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b01u32 << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
