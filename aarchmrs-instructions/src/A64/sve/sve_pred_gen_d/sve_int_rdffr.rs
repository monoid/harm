/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod rdffr_p_p_f_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000110001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "rdffr_p_p_f_";
    #[inline]
    pub const fn rdffr_p_p_f_(
        Pg: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101000110001111000u32 << 9u32
                | Pg.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod rdffrs_p_p_f_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101010110001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "rdffrs_p_p_f_";
    #[inline]
    pub const fn rdffrs_p_p_f_(
        Pg: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101010110001111000u32 << 9u32
                | Pg.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
