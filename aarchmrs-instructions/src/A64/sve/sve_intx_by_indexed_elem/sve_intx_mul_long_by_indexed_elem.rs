/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smullb_z_zzi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100101000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smullb_z_zzi_s";
    #[inline]
    pub const fn smullb_z_zzi_s(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        i3l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b1100u32 << 12u32
                | i3l.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod smullb_z_zzi_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100111000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smullb_z_zzi_d";
    #[inline]
    pub const fn smullb_z_zzi_d(
        i2h: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        i2l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | i2h.into_inner() << 20u32
                | Zm.into_inner() << 16u32
                | 0b1100u32 << 12u32
                | i2l.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod smullt_z_zzi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100101000001100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smullt_z_zzi_s";
    #[inline]
    pub const fn smullt_z_zzi_s(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        i3l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b1100u32 << 12u32
                | i3l.into_inner() << 11u32
                | 0b1u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod smullt_z_zzi_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100111000001100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smullt_z_zzi_d";
    #[inline]
    pub const fn smullt_z_zzi_d(
        i2h: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        i2l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | i2h.into_inner() << 20u32
                | Zm.into_inner() << 16u32
                | 0b1100u32 << 12u32
                | i2l.into_inner() << 11u32
                | 0b1u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod umullb_z_zzi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100101000001101000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umullb_z_zzi_s";
    #[inline]
    pub const fn umullb_z_zzi_s(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        i3l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b1101u32 << 12u32
                | i3l.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod umullb_z_zzi_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100111000001101000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umullb_z_zzi_d";
    #[inline]
    pub const fn umullb_z_zzi_d(
        i2h: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        i2l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | i2h.into_inner() << 20u32
                | Zm.into_inner() << 16u32
                | 0b1101u32 << 12u32
                | i2l.into_inner() << 11u32
                | 0b0u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod umullt_z_zzi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100101000001101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umullt_z_zzi_s";
    #[inline]
    pub const fn umullt_z_zzi_s(
        i3h: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        i3l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | i3h.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b1101u32 << 12u32
                | i3l.into_inner() << 11u32
                | 0b1u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod umullt_z_zzi_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100111000001101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umullt_z_zzi_d";
    #[inline]
    pub const fn umullt_z_zzi_d(
        i2h: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        i2l: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | i2h.into_inner() << 20u32
                | Zm.into_inner() << 16u32
                | 0b1101u32 << 12u32
                | i2l.into_inner() << 11u32
                | 0b1u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
