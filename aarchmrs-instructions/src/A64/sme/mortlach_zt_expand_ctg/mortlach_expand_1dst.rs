/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod luti2_z_ztz_ {
    #[inline]
    pub fn luti2_z_ztz_(
        i4: impl Into<::aarchmrs_types::BitValue<4>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000110011u32 << 18u32
                | u32::from(i4.into()) << 14u32
                | u32::from(size.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod luti4_z_ztz_ {
    #[inline]
    pub fn luti4_z_ztz_(
        i3: impl Into<::aarchmrs_types::BitValue<3>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000001100101u32 << 17u32
                | u32::from(i3.into()) << 14u32
                | u32::from(size.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
