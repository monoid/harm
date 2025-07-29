/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod flogb_z_p_z_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100000111101000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "flogb_z_p_z_z";
    #[inline]
    pub const fn flogb_z_p_z_z(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100000111101u32 << 15u32
                | size.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fcvtzs_z_p_z_s2wz {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100100111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtzs_z_p_z_s2wz";
    #[inline]
    pub const fn fcvtzs_z_p_z_s2wz(
        int_U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001001001111110u32 << 14u32
                | int_U.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fcvtzs_z_p_z_d2wz {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100110111101000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtzs_z_p_z_d2wz";
    #[inline]
    pub const fn fcvtzs_z_p_z_d2wz(
        int_U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001001101111010u32 << 14u32
                | int_U.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fcvtzs_z_p_z_s2xz {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100110111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtzs_z_p_z_s2xz";
    #[inline]
    pub const fn fcvtzs_z_p_z_s2xz(
        int_U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001001101111110u32 << 14u32
                | int_U.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fcvtzs_z_p_z_d2xz {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100110111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtzs_z_p_z_d2xz";
    #[inline]
    pub const fn fcvtzs_z_p_z_d2xz(
        int_U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001001101111111u32 << 14u32
                | int_U.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fcvtzs_z_p_z_fp162hz {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100010111101100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtzs_z_p_z_fp162hz";
    #[inline]
    pub const fn fcvtzs_z_p_z_fp162hz(
        int_U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001000101111011u32 << 14u32
                | int_U.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fcvtzs_z_p_z_fp162wz {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100010111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtzs_z_p_z_fp162wz";
    #[inline]
    pub const fn fcvtzs_z_p_z_fp162wz(
        int_U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001000101111110u32 << 14u32
                | int_U.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fcvtzs_z_p_z_fp162xz {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100010111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtzs_z_p_z_fp162xz";
    #[inline]
    pub const fn fcvtzs_z_p_z_fp162xz(
        int_U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001000101111111u32 << 14u32
                | int_U.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fcvtzu_z_p_z_s2wz {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100100111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtzu_z_p_z_s2wz";
    #[inline]
    pub const fn fcvtzu_z_p_z_s2wz(
        int_U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001001001111110u32 << 14u32
                | int_U.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fcvtzu_z_p_z_d2wz {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100110111101000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtzu_z_p_z_d2wz";
    #[inline]
    pub const fn fcvtzu_z_p_z_d2wz(
        int_U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001001101111010u32 << 14u32
                | int_U.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fcvtzu_z_p_z_s2xz {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100110111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtzu_z_p_z_s2xz";
    #[inline]
    pub const fn fcvtzu_z_p_z_s2xz(
        int_U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001001101111110u32 << 14u32
                | int_U.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fcvtzu_z_p_z_d2xz {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100110111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtzu_z_p_z_d2xz";
    #[inline]
    pub const fn fcvtzu_z_p_z_d2xz(
        int_U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001001101111111u32 << 14u32
                | int_U.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fcvtzu_z_p_z_fp162hz {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100010111101100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtzu_z_p_z_fp162hz";
    #[inline]
    pub const fn fcvtzu_z_p_z_fp162hz(
        int_U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001000101111011u32 << 14u32
                | int_U.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fcvtzu_z_p_z_fp162wz {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100010111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtzu_z_p_z_fp162wz";
    #[inline]
    pub const fn fcvtzu_z_p_z_fp162wz(
        int_U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001000101111110u32 << 14u32
                | int_U.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fcvtzu_z_p_z_fp162xz {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01100100010111111100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtzu_z_p_z_fp162xz";
    #[inline]
    pub const fn fcvtzu_z_p_z_fp162xz(
        int_U: ::aarchmrs_types::BitValue<1>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001000101111111u32 << 14u32
                | int_U.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
