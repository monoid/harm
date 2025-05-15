/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod splice_z_p_zz_des {
    #[inline]
    pub fn splice_z_p_zz_des(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pv: impl Into<::aarchmrs_types::BitValue<3>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b101100100u32 << 13u32
                | u32::from(Pv.into()) << 10u32
                | u32::from(Zm.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
