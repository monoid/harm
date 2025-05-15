/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod xar_z_zzi_ {
    #[inline]
    pub fn xar_z_zzi_(
        tszh: impl Into<::aarchmrs_types::BitValue<2>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Zm: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(tszh.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | u32::from(imm3.into()) << 16u32
                | 0b001101u32 << 10u32
                | u32::from(Zm.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
