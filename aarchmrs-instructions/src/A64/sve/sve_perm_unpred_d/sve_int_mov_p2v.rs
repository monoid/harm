/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod pmov_z_pi_b {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101001010110011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "pmov_z_pi_b";
    #[inline]
    pub const fn pmov_z_pi_b(
        Pn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101001010110011100u32 << 9u32
                | Pn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod pmov_z_pi_h {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111011111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101001011010011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "pmov_z_pi_h";
    #[inline]
    pub const fn pmov_z_pi_h(
        i1: ::aarchmrs_types::BitValue<1>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101001011u32 << 18u32
                | i1.into_inner() << 17u32
                | 0b10011100u32 << 9u32
                | Pn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod pmov_z_pi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111110011111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101011010010011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "pmov_z_pi_s";
    #[inline]
    pub const fn pmov_z_pi_s(
        i2: ::aarchmrs_types::BitValue<2>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0000010101101u32 << 19u32
                | i2.into_inner() << 17u32
                | 0b10011100u32 << 9u32
                | Pn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod pmov_z_pi_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101110011111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101101010010011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "pmov_z_pi_d";
    #[inline]
    pub const fn pmov_z_pi_d(
        i3h: ::aarchmrs_types::BitValue<1>,
        i3l: ::aarchmrs_types::BitValue<2>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001011u32 << 23u32
                | i3h.into_inner() << 22u32
                | 0b101u32 << 19u32
                | i3l.into_inner() << 17u32
                | 0b10011100u32 << 9u32
                | Pn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
