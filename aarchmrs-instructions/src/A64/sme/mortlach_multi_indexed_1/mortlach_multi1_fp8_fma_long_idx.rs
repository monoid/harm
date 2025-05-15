/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlal_za_z8z8i_1 {
    #[inline]
    pub fn fmlal_za_z8z8i_1(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        i4A: impl Into<::aarchmrs_types::BitValue<1>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i4B: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        i4C: impl Into<::aarchmrs_types::BitValue<1>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011100u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | u32::from(i4A.into()) << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b0u32 << 12u32
                | u32::from(i4B.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(i4C.into()) << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
