/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cntp_r_pn_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111101000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001000001000001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cntp_r_pn_";
    #[inline]
    pub const fn cntp_r_pn_(
        size: ::aarchmrs_types::BitValue<2>,
        vl: ::aarchmrs_types::BitValue<1>,
        PNn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10000010000u32 << 11u32
                | vl.into_inner() << 10u32
                | 0b1u32 << 9u32
                | PNn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
