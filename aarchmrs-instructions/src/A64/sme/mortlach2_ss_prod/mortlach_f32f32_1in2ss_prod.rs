/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ftmopa_za_zzzi_s2x1 {
    #[inline]
    pub fn ftmopa_za_zzzi_s2x1(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        K: impl Into<::aarchmrs_types::BitValue<1>>,
        Zk: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10000000010u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(K.into()) << 12u32
                | u32::from(Zk.into()) << 10u32
                | u32::from(Zn.into()) << 6u32
                | u32::from(i2.into()) << 4u32
                | 0b00u32 << 2u32
                | u32::from(ZAda.into()) << 0u32,
        )
    }
}
