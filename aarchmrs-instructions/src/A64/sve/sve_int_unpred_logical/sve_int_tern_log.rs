/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod eor3_z_zzz_ {
    #[inline]
    pub fn eor3_z_zzz_(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zk: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100001u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b001110u32 << 10u32
                | u32::from(Zk.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod bcax_z_zzz_ {
    #[inline]
    pub fn bcax_z_zzz_(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zk: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100011u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b001110u32 << 10u32
                | u32::from(Zk.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod bsl_z_zzz_ {
    #[inline]
    pub fn bsl_z_zzz_(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zk: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100001u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b001111u32 << 10u32
                | u32::from(Zk.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod bsl1n_z_zzz_ {
    #[inline]
    pub fn bsl1n_z_zzz_(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zk: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100011u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b001111u32 << 10u32
                | u32::from(Zk.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod bsl2n_z_zzz_ {
    #[inline]
    pub fn bsl2n_z_zzz_(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zk: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100101u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b001111u32 << 10u32
                | u32::from(Zk.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod nbsl_z_zzz_ {
    #[inline]
    pub fn nbsl_z_zzz_(
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zk: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100111u32 << 21u32
                | u32::from(Zm.into()) << 16u32
                | 0b001111u32 << 10u32
                | u32::from(Zk.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
