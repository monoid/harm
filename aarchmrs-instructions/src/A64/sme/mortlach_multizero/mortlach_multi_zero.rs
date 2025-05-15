/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod zero_za1_ri_2 {
    #[inline]
    pub fn zero_za1_ri_2(
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000000011000u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b0000000000u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod zero_za1_ri_4 {
    #[inline]
    pub fn zero_za1_ri_4(
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000000011100u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b0000000000u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod zero_za2_ri_1 {
    #[inline]
    pub fn zero_za2_ri_1(
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        off3: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000000011001u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b0000000000u32 << 3u32
                | u32::from(off3.into()) << 0u32,
        )
    }
}
pub mod zero_za2_ri_2 {
    #[inline]
    pub fn zero_za2_ri_2(
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000000011010u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b00000000000u32 << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
pub mod zero_za2_ri_4 {
    #[inline]
    pub fn zero_za2_ri_4(
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000000011011u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b00000000000u32 << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
pub mod zero_za4_ri_1 {
    #[inline]
    pub fn zero_za4_ri_1(
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        off2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000000011101u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b00000000000u32 << 2u32
                | u32::from(off2.into()) << 0u32,
        )
    }
}
pub mod zero_za4_ri_2 {
    #[inline]
    pub fn zero_za4_ri_2(
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000000011110u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b000000000000u32 << 1u32
                | u32::from(o1.into()) << 0u32,
        )
    }
}
pub mod zero_za4_ri_4 {
    #[inline]
    pub fn zero_za4_ri_4(
        Rv: impl Into<::aarchmrs_types::BitValue<2>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000000000011111u32 << 15u32
                | u32::from(Rv.into()) << 13u32
                | 0b000000000000u32 << 1u32
                | u32::from(o1.into()) << 0u32,
        )
    }
}
