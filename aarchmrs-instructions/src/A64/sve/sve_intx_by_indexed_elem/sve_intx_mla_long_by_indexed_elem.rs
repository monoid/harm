/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlalb_z_zzzi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100101000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlalb_z_zzzi_s";
    #[inline]
    pub const fn smlalb_z_zzzi_s(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        i3l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b1000u32 << 12u32
                | i3l.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod smlalb_z_zzzi_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlalb_z_zzzi_d";
    #[inline]
    pub const fn smlalb_z_zzzi_d(
        i2h: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        i2l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | i2h.into_inner() << 20u32
                | Zm.into_inner() << 16u32
                | 0b1000u32 << 12u32
                | i2l.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod smlslb_z_zzzi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100101000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlslb_z_zzzi_s";
    #[inline]
    pub const fn smlslb_z_zzzi_s(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        i3l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b1010u32 << 12u32
                | i3l.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod smlslb_z_zzzi_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100111000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlslb_z_zzzi_d";
    #[inline]
    pub const fn smlslb_z_zzzi_d(
        i2h: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        i2l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | i2h.into_inner() << 20u32
                | Zm.into_inner() << 16u32
                | 0b1010u32 << 12u32
                | i2l.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod smlalt_z_zzzi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100101000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlalt_z_zzzi_s";
    #[inline]
    pub const fn smlalt_z_zzzi_s(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        i3l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b1000u32 << 12u32
                | i3l.into_inner() << 11u32
                | 0b1u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod smlalt_z_zzzi_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100111000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlalt_z_zzzi_d";
    #[inline]
    pub const fn smlalt_z_zzzi_d(
        i2h: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        i2l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | i2h.into_inner() << 20u32
                | Zm.into_inner() << 16u32
                | 0b1000u32 << 12u32
                | i2l.into_inner() << 11u32
                | 0b1u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod smlslt_z_zzzi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100101000001010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlslt_z_zzzi_s";
    #[inline]
    pub const fn smlslt_z_zzzi_s(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        i3l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b1010u32 << 12u32
                | i3l.into_inner() << 11u32
                | 0b1u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod smlslt_z_zzzi_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100111000001010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlslt_z_zzzi_d";
    #[inline]
    pub const fn smlslt_z_zzzi_d(
        i2h: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        i2l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | i2h.into_inner() << 20u32
                | Zm.into_inner() << 16u32
                | 0b1010u32 << 12u32
                | i2l.into_inner() << 11u32
                | 0b1u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod umlalb_z_zzzi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100101000001001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlalb_z_zzzi_s";
    #[inline]
    pub const fn umlalb_z_zzzi_s(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        i3l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b1001u32 << 12u32
                | i3l.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod umlalb_z_zzzi_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100111000001001000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlalb_z_zzzi_d";
    #[inline]
    pub const fn umlalb_z_zzzi_d(
        i2h: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        i2l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | i2h.into_inner() << 20u32
                | Zm.into_inner() << 16u32
                | 0b1001u32 << 12u32
                | i2l.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod umlslb_z_zzzi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100101000001011000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlslb_z_zzzi_s";
    #[inline]
    pub const fn umlslb_z_zzzi_s(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        i3l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b1011u32 << 12u32
                | i3l.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod umlslb_z_zzzi_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100111000001011000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlslb_z_zzzi_d";
    #[inline]
    pub const fn umlslb_z_zzzi_d(
        i2h: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        i2l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | i2h.into_inner() << 20u32
                | Zm.into_inner() << 16u32
                | 0b1011u32 << 12u32
                | i2l.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod umlalt_z_zzzi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100101000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlalt_z_zzzi_s";
    #[inline]
    pub const fn umlalt_z_zzzi_s(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        i3l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b1001u32 << 12u32
                | i3l.into_inner() << 11u32
                | 0b1u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod umlalt_z_zzzi_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100111000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlalt_z_zzzi_d";
    #[inline]
    pub const fn umlalt_z_zzzi_d(
        i2h: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        i2l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | i2h.into_inner() << 20u32
                | Zm.into_inner() << 16u32
                | 0b1001u32 << 12u32
                | i2l.into_inner() << 11u32
                | 0b1u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod umlslt_z_zzzi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100101000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlslt_z_zzzi_s";
    #[inline]
    pub const fn umlslt_z_zzzi_s(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        i3l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b1011u32 << 12u32
                | i3l.into_inner() << 11u32
                | 0b1u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod umlslt_z_zzzi_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100111000001011010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlslt_z_zzzi_d";
    #[inline]
    pub const fn umlslt_z_zzzi_d(
        i2h: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        i2l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | i2h.into_inner() << 20u32
                | Zm.into_inner() << 16u32
                | 0b1011u32 << 12u32
                | i2l.into_inner() << 11u32
                | 0b1u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
