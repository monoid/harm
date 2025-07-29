/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod VRINTA_vfp_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101110000000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTA_vfp_A1_H";
    #[inline]
    pub const fn VRINTA_vfp_A1_H(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111000u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTA_vfp_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101110000000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTA_vfp_A1_S";
    #[inline]
    pub const fn VRINTA_vfp_A1_S(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111000u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTA_vfp_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101110000000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTA_vfp_A1_D";
    #[inline]
    pub const fn VRINTA_vfp_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111000u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTN_vfp_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101110010000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTN_vfp_A1_H";
    #[inline]
    pub const fn VRINTN_vfp_A1_H(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111001u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTN_vfp_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101110010000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTN_vfp_A1_S";
    #[inline]
    pub const fn VRINTN_vfp_A1_S(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111001u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTN_vfp_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101110010000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTN_vfp_A1_D";
    #[inline]
    pub const fn VRINTN_vfp_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111001u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTP_vfp_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101110100000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTP_vfp_A1_H";
    #[inline]
    pub const fn VRINTP_vfp_A1_H(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111010u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTP_vfp_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101110100000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTP_vfp_A1_S";
    #[inline]
    pub const fn VRINTP_vfp_A1_S(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111010u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTP_vfp_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101110100000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTP_vfp_A1_D";
    #[inline]
    pub const fn VRINTP_vfp_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111010u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTM_vfp_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101110110000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTM_vfp_A1_H";
    #[inline]
    pub const fn VRINTM_vfp_A1_H(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111011u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTM_vfp_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101110110000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTM_vfp_A1_S";
    #[inline]
    pub const fn VRINTM_vfp_A1_S(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111011u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTM_vfp_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101110110000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTM_vfp_A1_D";
    #[inline]
    pub const fn VRINTM_vfp_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111011u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTA_vfp_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101111000000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTA_vfp_A1_H";
    #[inline]
    pub const fn VCVTA_vfp_A1_H(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111100u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b1001u32 << 8u32
                | op.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTA_vfp_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101111000000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTA_vfp_A1_S";
    #[inline]
    pub const fn VCVTA_vfp_A1_S(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111100u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | op.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTA_vfp_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101111000000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTA_vfp_A1_D";
    #[inline]
    pub const fn VCVTA_vfp_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111100u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | op.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTN_vfp_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101111010000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTN_vfp_A1_H";
    #[inline]
    pub const fn VCVTN_vfp_A1_H(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111101u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b1001u32 << 8u32
                | op.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTN_vfp_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101111010000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTN_vfp_A1_S";
    #[inline]
    pub const fn VCVTN_vfp_A1_S(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111101u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | op.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTN_vfp_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101111010000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTN_vfp_A1_D";
    #[inline]
    pub const fn VCVTN_vfp_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111101u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | op.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTP_vfp_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101111100000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTP_vfp_A1_H";
    #[inline]
    pub const fn VCVTP_vfp_A1_H(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111110u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b1001u32 << 8u32
                | op.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTP_vfp_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101111100000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTP_vfp_A1_S";
    #[inline]
    pub const fn VCVTP_vfp_A1_S(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111110u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | op.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTP_vfp_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101111100000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTP_vfp_A1_D";
    #[inline]
    pub const fn VCVTP_vfp_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111110u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | op.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTM_vfp_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101111110000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTM_vfp_A1_H";
    #[inline]
    pub const fn VCVTM_vfp_A1_H(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111111u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b1001u32 << 8u32
                | op.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTM_vfp_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101111110000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTM_vfp_A1_S";
    #[inline]
    pub const fn VCVTM_vfp_A1_S(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111111u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | op.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTM_vfp_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110101111110000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTM_vfp_A1_D";
    #[inline]
    pub const fn VCVTM_vfp_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111111u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | op.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
