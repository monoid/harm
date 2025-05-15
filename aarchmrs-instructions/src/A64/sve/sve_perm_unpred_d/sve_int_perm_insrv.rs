/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod insr_z_v_ {
    #[inline]
    pub fn insr_z_v_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Vm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b110100001110u32 << 10u32
                | u32::from(Vm.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
