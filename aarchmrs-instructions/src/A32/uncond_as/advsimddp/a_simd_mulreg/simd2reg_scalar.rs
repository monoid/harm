/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod VMLA_s_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111001010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000000001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMLA_s_A1_D";
    #[inline]
    pub const fn VMLA_s_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100101u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b000u32 << 9u32
                | F.into_inner() << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMLA_s_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111001010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011100000000000000001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMLA_s_A1_Q";
    #[inline]
    pub const fn VMLA_s_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b000u32 << 9u32
                | F.into_inner() << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQDMLAL_A2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000001101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQDMLAL_A2";
    #[inline]
    pub const fn VQDMLAL_A2(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100101u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0011u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMLAL_s_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000001001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMLAL_s_A1";
    #[inline]
    pub const fn VMLAL_s_A1(
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | U.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0010u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQDMLSL_A2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000011101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQDMLSL_A2";
    #[inline]
    pub const fn VQDMLSL_A2(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100101u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0111u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMLS_s_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111001010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000010001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMLS_s_A1_D";
    #[inline]
    pub const fn VMLS_s_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100101u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b010u32 << 9u32
                | F.into_inner() << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMLS_s_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111001010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011100000000000010001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMLS_s_A1_Q";
    #[inline]
    pub const fn VMLS_s_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b010u32 << 9u32
                | F.into_inner() << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQDMULL_A2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQDMULL_A2";
    #[inline]
    pub const fn VQDMULL_A2(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100101u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMLSL_s_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000011001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMLSL_s_A1";
    #[inline]
    pub const fn VMLSL_s_A1(
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | U.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0110u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMUL_s_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111001010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000100001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMUL_s_A1_D";
    #[inline]
    pub const fn VMUL_s_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100101u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b100u32 << 9u32
                | F.into_inner() << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMUL_s_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111001010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011100000000000100001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMUL_s_A1_Q";
    #[inline]
    pub const fn VMUL_s_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b100u32 << 9u32
                | F.into_inner() << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMULL_s_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMULL_s_A1";
    #[inline]
    pub const fn VMULL_s_A1(
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111001u32 << 25u32
                | U.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQDMULH_A2_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000110001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQDMULH_A2_D";
    #[inline]
    pub const fn VQDMULH_A2_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100101u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1100u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQDMULH_A2_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011100000000000110001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQDMULH_A2_Q";
    #[inline]
    pub const fn VQDMULH_A2_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1100u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQRDMULH_A2_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000110101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQRDMULH_A2_D";
    #[inline]
    pub const fn VQRDMULH_A2_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100101u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1101u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQRDMULH_A2_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011100000000000110101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQRDMULH_A2_Q";
    #[inline]
    pub const fn VQRDMULH_A2_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1101u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQRDMLAH_A2_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000111001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQRDMLAH_A2_D";
    #[inline]
    pub const fn VQRDMLAH_A2_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100101u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1110u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQRDMLAH_A2_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011100000000000111001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQRDMLAH_A2_Q";
    #[inline]
    pub const fn VQRDMLAH_A2_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1110u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQRDMLSH_A2_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010100000000000111101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQRDMLSH_A2_D";
    #[inline]
    pub const fn VQRDMLSH_A2_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100101u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQRDMLSH_A2_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011100000000000111101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQRDMLSH_A2_Q";
    #[inline]
    pub const fn VQRDMLSH_A2_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
