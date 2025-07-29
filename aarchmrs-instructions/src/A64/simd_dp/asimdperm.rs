/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod UZP1_asimdperm_only {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001000001011110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110000000000001100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UZP1_asimdperm_only";
    #[inline]
    pub const fn UZP1_asimdperm_only(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | op.into_inner() << 14u32
                | 0b0110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod TRN1_asimdperm_only {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001000001011110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110000000000010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TRN1_asimdperm_only";
    #[inline]
    pub const fn TRN1_asimdperm_only(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | op.into_inner() << 14u32
                | 0b1010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ZIP1_asimdperm_only {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001000001011110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110000000000011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ZIP1_asimdperm_only";
    #[inline]
    pub const fn ZIP1_asimdperm_only(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | op.into_inner() << 14u32
                | 0b1110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UZP2_asimdperm_only {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001000001011110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110000000000001100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UZP2_asimdperm_only";
    #[inline]
    pub const fn UZP2_asimdperm_only(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | op.into_inner() << 14u32
                | 0b0110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod TRN2_asimdperm_only {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001000001011110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110000000000010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TRN2_asimdperm_only";
    #[inline]
    pub const fn TRN2_asimdperm_only(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | op.into_inner() << 14u32
                | 0b1010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ZIP2_asimdperm_only {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001000001011110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110000000000011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ZIP2_asimdperm_only";
    #[inline]
    pub const fn ZIP2_asimdperm_only(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rm: ::aarchmrs_types::BitValue<5>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b0u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | op.into_inner() << 14u32
                | 0b1110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
