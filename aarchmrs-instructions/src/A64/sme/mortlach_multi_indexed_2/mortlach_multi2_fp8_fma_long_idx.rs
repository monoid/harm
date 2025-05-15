/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlal_za_z8z8i_2xi {
    #[inline]
    pub fn fmlal_za_z8z8i_2xi(
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        i4h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<4>>,
        i4l: impl Into<::aarchmrs_types::BitValue<2>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011001u32 << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b1u32 << 12u32
                | u32::from(i4h.into()) << 10u32
                | u32::from(Zn.into()) << 6u32
                | 0b11u32 << 4u32
                | u32::from(i4l.into()) << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
