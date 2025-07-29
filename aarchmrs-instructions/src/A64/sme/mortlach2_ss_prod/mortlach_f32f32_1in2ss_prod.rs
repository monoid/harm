/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ftmopa_za_zzzi_s2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001110000000001100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ftmopa_za_zzzi_s2x1";
    #[inline]
    pub const fn ftmopa_za_zzzi_s2x1(
        Zm: ::aarchmrs_types::BitValue<5>,
        K: ::aarchmrs_types::BitValue<1>,
        Zk: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        i2: ::aarchmrs_types::BitValue<2>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000000010u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b000u32 << 13u32
                | K.into_inner() << 12u32
                | Zk.into_inner() << 10u32
                | Zn.into_inner() << 6u32
                | i2.into_inner() << 4u32
                | 0b00u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
