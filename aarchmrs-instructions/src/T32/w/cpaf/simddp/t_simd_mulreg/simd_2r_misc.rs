/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod VREV64_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VREV64_T1_D";
    #[inline]
    pub const fn VREV64_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b000000u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VREV64_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000000001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VREV64_T1_Q";
    #[inline]
    pub const fn VREV64_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b000001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VREV32_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VREV32_T1_D";
    #[inline]
    pub const fn VREV32_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b000010u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VREV32_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000000011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VREV32_T1_Q";
    #[inline]
    pub const fn VREV32_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b000011u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VREV16_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000000100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VREV16_T1_D";
    #[inline]
    pub const fn VREV16_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b000100u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VREV16_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000000101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VREV16_T1_Q";
    #[inline]
    pub const fn VREV16_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b000101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VPADDL_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VPADDL_T1_D";
    #[inline]
    pub const fn VPADDL_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0010u32 << 8u32
                | op.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VPADDL_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000001001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VPADDL_T1_Q";
    #[inline]
    pub const fn VPADDL_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0010u32 << 8u32
                | op.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod AESE_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000001100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AESE_T1";
    #[inline]
    pub const fn AESE_T1(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b001100u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod AESD_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000001101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AESD_T1";
    #[inline]
    pub const fn AESD_T1(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b001101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod AESMC_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000001110000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AESMC_T1";
    #[inline]
    pub const fn AESMC_T1(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b001110u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod AESIMC_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000001111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AESIMC_T1";
    #[inline]
    pub const fn AESIMC_T1(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b001111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCLS_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCLS_T1_D";
    #[inline]
    pub const fn VCLS_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010000u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCLS_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000010001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCLS_T1_Q";
    #[inline]
    pub const fn VCLS_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VSWP_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSWP_T1_D";
    #[inline]
    pub const fn VSWP_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110010u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b000000u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VSWP_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000000001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSWP_T1_Q";
    #[inline]
    pub const fn VSWP_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110010u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b000001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCLZ_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000010010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCLZ_T1_D";
    #[inline]
    pub const fn VCLZ_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010010u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCLZ_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000010011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCLZ_T1_Q";
    #[inline]
    pub const fn VCLZ_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010011u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCNT_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000010100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCNT_T1_D";
    #[inline]
    pub const fn VCNT_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010100u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCNT_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000010101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCNT_T1_Q";
    #[inline]
    pub const fn VCNT_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMVN_r_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000010110000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMVN_r_T1_D";
    #[inline]
    pub const fn VMVN_r_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010110u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMVN_r_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000010111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMVN_r_T1_Q";
    #[inline]
    pub const fn VMVN_r_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VPADAL_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000011000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VPADAL_T1_D";
    #[inline]
    pub const fn VPADAL_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0110u32 << 8u32
                | op.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VPADAL_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000011001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VPADAL_T1_Q";
    #[inline]
    pub const fn VPADAL_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0110u32 << 8u32
                | op.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQABS_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000011100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQABS_T1_D";
    #[inline]
    pub const fn VQABS_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b011100u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQABS_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000011101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQABS_T1_Q";
    #[inline]
    pub const fn VQABS_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b011101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQNEG_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000011110000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQNEG_T1_D";
    #[inline]
    pub const fn VQNEG_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b011110u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQNEG_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100000000011111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQNEG_T1_Q";
    #[inline]
    pub const fn VQNEG_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b00u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b011111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCGT_i_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000101111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCGT_i_T1_D";
    #[inline]
    pub const fn VCGT_i_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b01u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | F.into_inner() << 10u32
                | 0b0000u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCGT_i_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000101111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100010000000001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCGT_i_T1_Q";
    #[inline]
    pub const fn VCGT_i_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b01u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | F.into_inner() << 10u32
                | 0b0001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCGE_i_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000101111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100010000000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCGE_i_T1_D";
    #[inline]
    pub const fn VCGE_i_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b01u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | F.into_inner() << 10u32
                | 0b0010u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCGE_i_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000101111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100010000000011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCGE_i_T1_Q";
    #[inline]
    pub const fn VCGE_i_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b01u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | F.into_inner() << 10u32
                | 0b0011u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCEQ_i_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000101111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100010000000100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCEQ_i_T1_D";
    #[inline]
    pub const fn VCEQ_i_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b01u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | F.into_inner() << 10u32
                | 0b0100u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCEQ_i_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000101111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100010000000101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCEQ_i_T1_Q";
    #[inline]
    pub const fn VCEQ_i_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b01u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | F.into_inner() << 10u32
                | 0b0101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCLE_i_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000101111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100010000000110000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCLE_i_T1_D";
    #[inline]
    pub const fn VCLE_i_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b01u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | F.into_inner() << 10u32
                | 0b0110u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCLE_i_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000101111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100010000000111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCLE_i_T1_Q";
    #[inline]
    pub const fn VCLE_i_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b01u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | F.into_inner() << 10u32
                | 0b0111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCLT_i_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000101111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100010000001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCLT_i_T1_D";
    #[inline]
    pub const fn VCLT_i_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b01u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | F.into_inner() << 10u32
                | 0b1000u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCLT_i_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000101111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100010000001001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCLT_i_T1_Q";
    #[inline]
    pub const fn VCLT_i_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b01u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | F.into_inner() << 10u32
                | 0b1001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VABS_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000101111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100010000001100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VABS_T1_D";
    #[inline]
    pub const fn VABS_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b01u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | F.into_inner() << 10u32
                | 0b1100u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VABS_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000101111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100010000001101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VABS_T1_Q";
    #[inline]
    pub const fn VABS_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b01u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | F.into_inner() << 10u32
                | 0b1101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VNEG_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000101111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100010000001110000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VNEG_T1_D";
    #[inline]
    pub const fn VNEG_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b01u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | F.into_inner() << 10u32
                | 0b1110u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VNEG_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000101111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100010000001111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VNEG_T1_Q";
    #[inline]
    pub const fn VNEG_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b01u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0u32 << 11u32
                | F.into_inner() << 10u32
                | 0b1111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod SHA1H_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100010000001011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHA1H_T1";
    #[inline]
    pub const fn SHA1H_T1(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b01u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b001011u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_bfs_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101101100000011001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_bfs_T1";
    #[inline]
    pub const fn VCVT_bfs_T1(
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110110u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b011001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VTRN_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VTRN_T1_D";
    #[inline]
    pub const fn VTRN_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b000010u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VTRN_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000000011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VTRN_T1_Q";
    #[inline]
    pub const fn VTRN_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b000011u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VUZP_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000000100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VUZP_T1_D";
    #[inline]
    pub const fn VUZP_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b000100u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VUZP_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000000101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VUZP_T1_Q";
    #[inline]
    pub const fn VUZP_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b000101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VZIP_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000000110000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VZIP_T1_D";
    #[inline]
    pub const fn VZIP_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b000110u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VZIP_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000000111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VZIP_T1_Q";
    #[inline]
    pub const fn VZIP_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b000111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMOVN_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOVN_T1";
    #[inline]
    pub const fn VMOVN_T1(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b001000u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQMOVN_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111110010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000001010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQMOVN_T1";
    #[inline]
    pub const fn VQMOVN_T1(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b00101u32 << 7u32
                | op.into_inner() << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQMOVUN_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000001001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQMOVUN_T1";
    #[inline]
    pub const fn VQMOVUN_T1(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b001001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VSHLL_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000001100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSHLL_T2";
    #[inline]
    pub const fn VSHLL_T2(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b001100u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod SHA1SU1_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000001110000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHA1SU1_T1";
    #[inline]
    pub const fn SHA1SU1_T1(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b001110u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod SHA256SU0_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000001111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHA256SU0_T1";
    #[inline]
    pub const fn SHA256SU0_T1(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b001111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTN_asimd_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTN_asimd_T1_D";
    #[inline]
    pub const fn VRINTN_asimd_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010000u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTN_asimd_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000010001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTN_asimd_T1_Q";
    #[inline]
    pub const fn VRINTN_asimd_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTX_asimd_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000010010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTX_asimd_T1_D";
    #[inline]
    pub const fn VRINTX_asimd_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010010u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTX_asimd_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000010011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTX_asimd_T1_Q";
    #[inline]
    pub const fn VRINTX_asimd_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010011u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTA_asimd_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000010100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTA_asimd_T1_D";
    #[inline]
    pub const fn VRINTA_asimd_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010100u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTA_asimd_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000010101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTA_asimd_T1_Q";
    #[inline]
    pub const fn VRINTA_asimd_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTZ_asimd_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000010110000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTZ_asimd_T1_D";
    #[inline]
    pub const fn VRINTZ_asimd_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010110u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTZ_asimd_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000010111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTZ_asimd_T1_Q";
    #[inline]
    pub const fn VRINTZ_asimd_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_sh_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000011100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_sh_T1";
    #[inline]
    pub const fn VCVT_sh_T1(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b011100u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_hs_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000011000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_hs_T1";
    #[inline]
    pub const fn VCVT_hs_T1(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b011000u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTM_asimd_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000011010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTM_asimd_T1_D";
    #[inline]
    pub const fn VRINTM_asimd_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b011010u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTM_asimd_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000011011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTM_asimd_T1_Q";
    #[inline]
    pub const fn VRINTM_asimd_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b011011u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTP_asimd_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000011110000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTP_asimd_T1_D";
    #[inline]
    pub const fn VRINTP_asimd_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b011110u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTP_asimd_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100100000011111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTP_asimd_T1_Q";
    #[inline]
    pub const fn VRINTP_asimd_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b10u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b011111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTA_asimd_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTA_asimd_T1_D";
    #[inline]
    pub const fn VCVTA_asimd_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b11u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0000u32 << 8u32
                | op.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTA_asimd_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100110000000001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTA_asimd_T1_Q";
    #[inline]
    pub const fn VCVTA_asimd_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b11u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0000u32 << 8u32
                | op.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTN_asimd_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100110000000100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTN_asimd_T1_D";
    #[inline]
    pub const fn VCVTN_asimd_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b11u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | op.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTN_asimd_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100110000000101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTN_asimd_T1_Q";
    #[inline]
    pub const fn VCVTN_asimd_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b11u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | op.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTP_asimd_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100110000001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTP_asimd_T1_D";
    #[inline]
    pub const fn VCVTP_asimd_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b11u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0010u32 << 8u32
                | op.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTP_asimd_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100110000001001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTP_asimd_T1_Q";
    #[inline]
    pub const fn VCVTP_asimd_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b11u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0010u32 << 8u32
                | op.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTM_asimd_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100110000001100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTM_asimd_T1_D";
    #[inline]
    pub const fn VCVTM_asimd_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b11u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0011u32 << 8u32
                | op.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTM_asimd_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100110000001101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTM_asimd_T1_Q";
    #[inline]
    pub const fn VCVTM_asimd_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b11u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b0011u32 << 8u32
                | op.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRECPE_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111011010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100110000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRECPE_T1_D";
    #[inline]
    pub const fn VRECPE_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b11u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010u32 << 9u32
                | F.into_inner() << 8u32
                | 0b00u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRECPE_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111011010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100110000010001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRECPE_T1_Q";
    #[inline]
    pub const fn VRECPE_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b11u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010u32 << 9u32
                | F.into_inner() << 8u32
                | 0b01u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRSQRTE_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111011010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100110000010010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRSQRTE_T1_D";
    #[inline]
    pub const fn VRSQRTE_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b11u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010u32 << 9u32
                | F.into_inner() << 8u32
                | 0b10u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRSQRTE_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111011010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100110000010011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRSQRTE_T1_Q";
    #[inline]
    pub const fn VRSQRTE_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        F: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b11u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b010u32 << 9u32
                | F.into_inner() << 8u32
                | 0b11u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_is_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111001010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100110000011000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_is_T1_D";
    #[inline]
    pub const fn VCVT_is_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<2>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b11u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b011u32 << 9u32
                | op.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_is_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100110000111001010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111111101100110000011001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_is_T1_Q";
    #[inline]
    pub const fn VCVT_is_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<2>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111111u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | size.into_inner() << 18u32
                | 0b11u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b011u32 << 9u32
                | op.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
