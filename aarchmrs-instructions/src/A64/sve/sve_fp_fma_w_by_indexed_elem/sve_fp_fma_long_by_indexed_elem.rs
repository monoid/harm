/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlalb_z_zzzi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001101000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100101000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmlalb_z_zzzi_s";
    #[inline]
    pub const fn fmlalb_z_zzzi_s(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        i3l: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100101u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | op.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i3l.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod bfmlalb_z_zzzi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001101000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100111000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmlalb_z_zzzi_";
    #[inline]
    pub const fn bfmlalb_z_zzzi_(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        i3l: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100111u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | op.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i3l.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod fmlslb_z_zzzi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001101000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100101000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmlslb_z_zzzi_s";
    #[inline]
    pub const fn fmlslb_z_zzzi_s(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        i3l: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100101u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | op.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i3l.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod bfmlslb_z_zzzi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001101000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100111000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmlslb_z_zzzi_";
    #[inline]
    pub const fn bfmlslb_z_zzzi_(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        i3l: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100111u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | op.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i3l.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod fmlalt_z_zzzi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001101000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100101000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmlalt_z_zzzi_s";
    #[inline]
    pub const fn fmlalt_z_zzzi_s(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        i3l: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100101u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | op.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i3l.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod bfmlalt_z_zzzi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001101000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100111000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmlalt_z_zzzi_";
    #[inline]
    pub const fn bfmlalt_z_zzzi_(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        i3l: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100111u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | op.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i3l.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod fmlslt_z_zzzi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001101000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100101000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmlslt_z_zzzi_s";
    #[inline]
    pub const fn fmlslt_z_zzzi_s(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        i3l: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100101u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | op.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i3l.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod bfmlslt_z_zzzi_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001101000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100111000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmlslt_z_zzzi_";
    #[inline]
    pub const fn bfmlslt_z_zzzi_(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        i3l: ::aarchmrs_types::BitValue<1>,
        T: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100111u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b01u32 << 14u32
                | op.into_inner() << 13u32
                | 0b0u32 << 12u32
                | i3l.into_inner() << 11u32
                | T.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
