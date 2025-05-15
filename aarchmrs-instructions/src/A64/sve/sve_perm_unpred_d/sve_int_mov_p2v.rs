/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod pmov_z_pi_b {
    #[inline]
    pub fn pmov_z_pi_b(
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101001010110011100u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod pmov_z_pi_h {
    #[inline]
    pub fn pmov_z_pi_h(
        i1: impl Into<::aarchmrs_types::BitValue<1>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101001011u32 << 18u32
                | u32::from(i1.into()) << 17u32
                | 0b10011100u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod pmov_z_pi_s {
    #[inline]
    pub fn pmov_z_pi_s(
        i2: impl Into<::aarchmrs_types::BitValue<2>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0000010101101u32 << 19u32
                | u32::from(i2.into()) << 17u32
                | 0b10011100u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod pmov_z_pi_d {
    #[inline]
    pub fn pmov_z_pi_d(
        i3h: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<2>>,
        Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001011u32 << 23u32
                | u32::from(i3h.into()) << 22u32
                | 0b101u32 << 19u32
                | u32::from(i3l.into()) << 17u32
                | 0b10011100u32 << 9u32
                | u32::from(Pn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
