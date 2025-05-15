/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlalb_z_zzzi_s {
    #[inline]
    pub fn fmlalb_z_zzzi_s(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100101u32 << 21u32
                | u32::from(i3h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 13u32
                | 0b0u32 << 12u32
                | u32::from(i3l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod bfmlalb_z_zzzi_ {
    #[inline]
    pub fn bfmlalb_z_zzzi_(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100111u32 << 21u32
                | u32::from(i3h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 13u32
                | 0b0u32 << 12u32
                | u32::from(i3l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod fmlslb_z_zzzi_s {
    #[inline]
    pub fn fmlslb_z_zzzi_s(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100101u32 << 21u32
                | u32::from(i3h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 13u32
                | 0b0u32 << 12u32
                | u32::from(i3l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod bfmlslb_z_zzzi_ {
    #[inline]
    pub fn bfmlslb_z_zzzi_(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100111u32 << 21u32
                | u32::from(i3h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 13u32
                | 0b0u32 << 12u32
                | u32::from(i3l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod fmlalt_z_zzzi_s {
    #[inline]
    pub fn fmlalt_z_zzzi_s(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100101u32 << 21u32
                | u32::from(i3h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 13u32
                | 0b0u32 << 12u32
                | u32::from(i3l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod bfmlalt_z_zzzi_ {
    #[inline]
    pub fn bfmlalt_z_zzzi_(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100111u32 << 21u32
                | u32::from(i3h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 13u32
                | 0b0u32 << 12u32
                | u32::from(i3l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod fmlslt_z_zzzi_s {
    #[inline]
    pub fn fmlslt_z_zzzi_s(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100101u32 << 21u32
                | u32::from(i3h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 13u32
                | 0b0u32 << 12u32
                | u32::from(i3l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod bfmlslt_z_zzzi_ {
    #[inline]
    pub fn bfmlslt_z_zzzi_(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100111u32 << 21u32
                | u32::from(i3h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 13u32
                | 0b0u32 << 12u32
                | u32::from(i3l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
