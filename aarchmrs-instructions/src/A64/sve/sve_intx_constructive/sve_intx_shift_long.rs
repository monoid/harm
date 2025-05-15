/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sshllb_z_zi_ {
    #[inline]
    pub fn sshllb_z_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<1>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | u32::from(tszh.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | u32::from(imm3.into()) << 16u32
                | 0b1010u32 << 12u32
                | u32::from(U.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod sshllt_z_zi_ {
    #[inline]
    pub fn sshllt_z_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<1>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | u32::from(tszh.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | u32::from(imm3.into()) << 16u32
                | 0b1010u32 << 12u32
                | u32::from(U.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod ushllb_z_zi_ {
    #[inline]
    pub fn ushllb_z_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<1>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | u32::from(tszh.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | u32::from(imm3.into()) << 16u32
                | 0b1010u32 << 12u32
                | u32::from(U.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod ushllt_z_zi_ {
    #[inline]
    pub fn ushllt_z_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<1>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | u32::from(tszh.into()) << 22u32
                | 0b0u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | u32::from(imm3.into()) << 16u32
                | 0b1010u32 << 12u32
                | u32::from(U.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
