/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cls_z_p_z_m {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000110001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cls_z_p_z_m";
    #[inline]
    pub const fn cls_z_p_z_m(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b011000101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod cls_z_p_z_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000010001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cls_z_p_z_z";
    #[inline]
    pub const fn cls_z_p_z_z(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b001000101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod clz_z_p_z_m {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000110011010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "clz_z_p_z_m";
    #[inline]
    pub const fn clz_z_p_z_m(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b011001101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod clz_z_p_z_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000010011010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "clz_z_p_z_z";
    #[inline]
    pub const fn clz_z_p_z_z(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b001001101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod cnt_z_p_z_m {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000110101010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cnt_z_p_z_m";
    #[inline]
    pub const fn cnt_z_p_z_m(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b011010101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod cnt_z_p_z_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000010101010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cnt_z_p_z_z";
    #[inline]
    pub const fn cnt_z_p_z_z(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b001010101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod cnot_z_p_z_m {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000110111010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cnot_z_p_z_m";
    #[inline]
    pub const fn cnot_z_p_z_m(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b011011101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod cnot_z_p_z_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000010111010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "cnot_z_p_z_z";
    #[inline]
    pub const fn cnot_z_p_z_z(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b001011101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fabs_z_p_z_m {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000111001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fabs_z_p_z_m";
    #[inline]
    pub const fn fabs_z_p_z_m(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b011100101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fabs_z_p_z_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000011001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fabs_z_p_z_z";
    #[inline]
    pub const fn fabs_z_p_z_z(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b001100101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fneg_z_p_z_m {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000111011010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fneg_z_p_z_m";
    #[inline]
    pub const fn fneg_z_p_z_m(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b011101101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod fneg_z_p_z_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000011011010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fneg_z_p_z_z";
    #[inline]
    pub const fn fneg_z_p_z_z(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b001101101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod not_z_p_z_m {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000111101010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "not_z_p_z_m";
    #[inline]
    pub const fn not_z_p_z_m(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b011110101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod not_z_p_z_z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000100000011101010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "not_z_p_z_z";
    #[inline]
    pub const fn not_z_p_z_z(
        size: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | size.into_inner() << 22u32
                | 0b001110101u32 << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
