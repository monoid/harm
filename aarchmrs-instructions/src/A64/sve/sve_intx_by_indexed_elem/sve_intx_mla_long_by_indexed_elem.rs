/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlalb_z_zzzi_s {
    #[inline]
    pub fn smlalb_z_zzzi_s(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | u32::from(i3h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(S.into()) << 13u32
                | u32::from(U.into()) << 12u32
                | u32::from(i3l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod smlalb_z_zzzi_d {
    #[inline]
    pub fn smlalb_z_zzzi_d(
        i2h: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        i2l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | u32::from(i2h.into()) << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(S.into()) << 13u32
                | u32::from(U.into()) << 12u32
                | u32::from(i2l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod smlslb_z_zzzi_s {
    #[inline]
    pub fn smlslb_z_zzzi_s(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | u32::from(i3h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(S.into()) << 13u32
                | u32::from(U.into()) << 12u32
                | u32::from(i3l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod smlslb_z_zzzi_d {
    #[inline]
    pub fn smlslb_z_zzzi_d(
        i2h: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        i2l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | u32::from(i2h.into()) << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(S.into()) << 13u32
                | u32::from(U.into()) << 12u32
                | u32::from(i2l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod smlalt_z_zzzi_s {
    #[inline]
    pub fn smlalt_z_zzzi_s(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | u32::from(i3h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(S.into()) << 13u32
                | u32::from(U.into()) << 12u32
                | u32::from(i3l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod smlalt_z_zzzi_d {
    #[inline]
    pub fn smlalt_z_zzzi_d(
        i2h: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        i2l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | u32::from(i2h.into()) << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(S.into()) << 13u32
                | u32::from(U.into()) << 12u32
                | u32::from(i2l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod smlslt_z_zzzi_s {
    #[inline]
    pub fn smlslt_z_zzzi_s(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | u32::from(i3h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(S.into()) << 13u32
                | u32::from(U.into()) << 12u32
                | u32::from(i3l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod smlslt_z_zzzi_d {
    #[inline]
    pub fn smlslt_z_zzzi_d(
        i2h: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        i2l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | u32::from(i2h.into()) << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(S.into()) << 13u32
                | u32::from(U.into()) << 12u32
                | u32::from(i2l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod umlalb_z_zzzi_s {
    #[inline]
    pub fn umlalb_z_zzzi_s(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | u32::from(i3h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(S.into()) << 13u32
                | u32::from(U.into()) << 12u32
                | u32::from(i3l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod umlalb_z_zzzi_d {
    #[inline]
    pub fn umlalb_z_zzzi_d(
        i2h: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        i2l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | u32::from(i2h.into()) << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(S.into()) << 13u32
                | u32::from(U.into()) << 12u32
                | u32::from(i2l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod umlslb_z_zzzi_s {
    #[inline]
    pub fn umlslb_z_zzzi_s(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | u32::from(i3h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(S.into()) << 13u32
                | u32::from(U.into()) << 12u32
                | u32::from(i3l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod umlslb_z_zzzi_d {
    #[inline]
    pub fn umlslb_z_zzzi_d(
        i2h: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        i2l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | u32::from(i2h.into()) << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(S.into()) << 13u32
                | u32::from(U.into()) << 12u32
                | u32::from(i2l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod umlalt_z_zzzi_s {
    #[inline]
    pub fn umlalt_z_zzzi_s(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | u32::from(i3h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(S.into()) << 13u32
                | u32::from(U.into()) << 12u32
                | u32::from(i3l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod umlalt_z_zzzi_d {
    #[inline]
    pub fn umlalt_z_zzzi_d(
        i2h: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        i2l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | u32::from(i2h.into()) << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(S.into()) << 13u32
                | u32::from(U.into()) << 12u32
                | u32::from(i2l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod umlslt_z_zzzi_s {
    #[inline]
    pub fn umlslt_z_zzzi_s(
        i3h: impl Into<::aarchmrs_types::BitValue<2>>,
        Zm: impl Into<::aarchmrs_types::BitValue<3>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        i3l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | u32::from(i3h.into()) << 19u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(S.into()) << 13u32
                | u32::from(U.into()) << 12u32
                | u32::from(i3l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
pub mod umlslt_z_zzzi_d {
    #[inline]
    pub fn umlslt_z_zzzi_d(
        i2h: impl Into<::aarchmrs_types::BitValue<1>>,
        Zm: impl Into<::aarchmrs_types::BitValue<4>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        i2l: impl Into<::aarchmrs_types::BitValue<1>>,
        T: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zda: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | u32::from(i2h.into()) << 20u32
                | u32::from(Zm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(S.into()) << 13u32
                | u32::from(U.into()) << 12u32
                | u32::from(i2l.into()) << 11u32
                | u32::from(T.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zda.into()) << 0u32,
        )
    }
}
