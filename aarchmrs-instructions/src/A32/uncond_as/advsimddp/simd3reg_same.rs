/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod VFMA_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000110000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VFMA_A1_D";
    #[inline]
    pub const fn VFMA_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1100u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VFMA_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000110001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VFMA_A1_Q";
    #[inline]
    pub const fn VFMA_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1100u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VADD_f_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000110100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VADD_f_A1_D";
    #[inline]
    pub const fn VADD_f_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1101u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VADD_f_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000110101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VADD_f_A1_Q";
    #[inline]
    pub const fn VADD_f_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
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
pub mod VMLA_f_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000110100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMLA_f_A1_D";
    #[inline]
    pub const fn VMLA_f_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1101u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMLA_f_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000110101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMLA_f_A1_Q";
    #[inline]
    pub const fn VMLA_f_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1101u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCEQ_r_A2_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCEQ_r_A2_D";
    #[inline]
    pub const fn VCEQ_r_A2_D(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1110u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCEQ_r_A2_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000111001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCEQ_r_A2_Q";
    #[inline]
    pub const fn VCEQ_r_A2_Q(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
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
pub mod VMAX_f_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMAX_f_A1_D";
    #[inline]
    pub const fn VMAX_f_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMAX_f_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000111101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMAX_f_A1_Q";
    #[inline]
    pub const fn VMAX_f_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
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
pub mod VRECPS_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000111100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRECPS_A1_D";
    #[inline]
    pub const fn VRECPS_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRECPS_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRECPS_A1_Q";
    #[inline]
    pub const fn VRECPS_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VHADD_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VHADD_A1_D";
    #[inline]
    pub const fn VHADD_A1_D(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0000u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VHADD_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000000001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VHADD_A1_Q";
    #[inline]
    pub const fn VHADD_A1_Q(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0000u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VAND_r_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000000100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VAND_r_A1_D";
    #[inline]
    pub const fn VAND_r_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VAND_r_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000000101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VAND_r_A1_Q";
    #[inline]
    pub const fn VAND_r_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQADD_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQADD_A1_D";
    #[inline]
    pub const fn VQADD_A1_D(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0000u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQADD_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000000001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQADD_A1_Q";
    #[inline]
    pub const fn VQADD_A1_Q(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0000u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRHADD_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000000100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRHADD_A1_D";
    #[inline]
    pub const fn VRHADD_A1_D(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRHADD_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000000101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRHADD_A1_Q";
    #[inline]
    pub const fn VRHADD_A1_Q(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod SHA1C_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHA1C_A1";
    #[inline]
    pub const fn SHA1C_A1(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        Q: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1100u32 << 8u32
                | N.into_inner() << 7u32
                | Q.into_inner() << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VHSUB_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VHSUB_A1_D";
    #[inline]
    pub const fn VHSUB_A1_D(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0010u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VHSUB_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000001001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VHSUB_A1_Q";
    #[inline]
    pub const fn VHSUB_A1_Q(
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
                | 0b0u32 << 23u32
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
pub mod VBIC_r_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000100000000000100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VBIC_r_A1_D";
    #[inline]
    pub const fn VBIC_r_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b01u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VBIC_r_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000100000000000101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VBIC_r_A1_Q";
    #[inline]
    pub const fn VBIC_r_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b01u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQSUB_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000001000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQSUB_A1_D";
    #[inline]
    pub const fn VQSUB_A1_D(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0010u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQSUB_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000001001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQSUB_A1_Q";
    #[inline]
    pub const fn VQSUB_A1_Q(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0010u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCGT_r_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000001100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCGT_r_A1_D";
    #[inline]
    pub const fn VCGT_r_A1_D(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0011u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCGT_r_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000001101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCGT_r_A1_Q";
    #[inline]
    pub const fn VCGT_r_A1_Q(
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
                | 0b0u32 << 23u32
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
pub mod VCGE_r_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000001100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCGE_r_A1_D";
    #[inline]
    pub const fn VCGE_r_A1_D(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0011u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCGE_r_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000001101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCGE_r_A1_Q";
    #[inline]
    pub const fn VCGE_r_A1_Q(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0011u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod SHA1P_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000100000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHA1P_A1";
    #[inline]
    pub const fn SHA1P_A1(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        Q: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b01u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1100u32 << 8u32
                | N.into_inner() << 7u32
                | Q.into_inner() << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VFMS_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010001000000000110000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VFMS_A1_D";
    #[inline]
    pub const fn VFMS_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1100u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VFMS_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010001000000000110001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VFMS_A1_Q";
    #[inline]
    pub const fn VFMS_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1100u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VSUB_f_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010001000000000110100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSUB_f_A1_D";
    #[inline]
    pub const fn VSUB_f_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1101u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VSUB_f_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010001000000000110101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSUB_f_A1_Q";
    #[inline]
    pub const fn VSUB_f_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | sz.into_inner() << 20u32
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
pub mod VMLS_f_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010001000000000110100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMLS_f_A1_D";
    #[inline]
    pub const fn VMLS_f_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1101u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMLS_f_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010001000000000110101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMLS_f_A1_Q";
    #[inline]
    pub const fn VMLS_f_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1101u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMIN_f_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010001000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMIN_f_A1_D";
    #[inline]
    pub const fn VMIN_f_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMIN_f_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010001000000000111101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMIN_f_A1_Q";
    #[inline]
    pub const fn VMIN_f_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | sz.into_inner() << 20u32
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
pub mod VRSQRTS_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010001000000000111100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRSQRTS_A1_D";
    #[inline]
    pub const fn VRSQRTS_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRSQRTS_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010001000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRSQRTS_A1_Q";
    #[inline]
    pub const fn VRSQRTS_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VSHL_r_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSHL_r_A1_D";
    #[inline]
    pub const fn VSHL_r_A1_D(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0100u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VSHL_r_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000010001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSHL_r_A1_Q";
    #[inline]
    pub const fn VSHL_r_A1_Q(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0100u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VADD_i_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VADD_i_A1_D";
    #[inline]
    pub const fn VADD_i_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1000u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VADD_i_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000100001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VADD_i_A1_Q";
    #[inline]
    pub const fn VADD_i_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1000u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VORR_r_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010001000000000000100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VORR_r_A1_D";
    #[inline]
    pub const fn VORR_r_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VORR_r_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010001000000000000101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VORR_r_A1_Q";
    #[inline]
    pub const fn VORR_r_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VTST_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000100000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VTST_A1_D";
    #[inline]
    pub const fn VTST_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1000u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VTST_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000100001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VTST_A1_Q";
    #[inline]
    pub const fn VTST_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1000u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQSHL_r_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000010000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQSHL_r_A1_D";
    #[inline]
    pub const fn VQSHL_r_A1_D(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0100u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQSHL_r_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000010001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQSHL_r_A1_Q";
    #[inline]
    pub const fn VQSHL_r_A1_Q(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0100u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMLA_i_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000100100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMLA_i_A1_D";
    #[inline]
    pub const fn VMLA_i_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMLA_i_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMLA_i_A1_Q";
    #[inline]
    pub const fn VMLA_i_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRSHL_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000010100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRSHL_A1_D";
    #[inline]
    pub const fn VRSHL_A1_D(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0101u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRSHL_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000010101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRSHL_A1_Q";
    #[inline]
    pub const fn VRSHL_A1_Q(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0101u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQRSHL_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000010100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQRSHL_A1_D";
    #[inline]
    pub const fn VQRSHL_A1_D(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0101u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQRSHL_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000010101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQRSHL_A1_Q";
    #[inline]
    pub const fn VQRSHL_A1_Q(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0101u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQDMULH_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQDMULH_A1_D";
    #[inline]
    pub const fn VQDMULH_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQDMULH_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQDMULH_A1_Q";
    #[inline]
    pub const fn VQDMULH_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
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
pub mod SHA1M_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010001000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHA1M_A1";
    #[inline]
    pub const fn SHA1M_A1(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        Q: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1100u32 << 8u32
                | N.into_inner() << 7u32
                | Q.into_inner() << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VPADD_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111100010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000101100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VPADD_i_A1";
    #[inline]
    pub const fn VPADD_i_A1(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        Q: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | N.into_inner() << 7u32
                | Q.into_inner() << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMAX_i_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000011000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMAX_i_A1_D";
    #[inline]
    pub const fn VMAX_i_A1_D(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0110u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMAX_i_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000011001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMAX_i_A1_Q";
    #[inline]
    pub const fn VMAX_i_A1_Q(
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
                | 0b0u32 << 23u32
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
pub mod VORN_r_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010001100000000000100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VORN_r_A1_D";
    #[inline]
    pub const fn VORN_r_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VORN_r_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010001100000000000101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VORN_r_A1_Q";
    #[inline]
    pub const fn VORN_r_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMIN_i_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000011000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMIN_i_A1_D";
    #[inline]
    pub const fn VMIN_i_A1_D(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0110u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMIN_i_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000011001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMIN_i_A1_Q";
    #[inline]
    pub const fn VMIN_i_A1_Q(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0110u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VABD_i_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000011100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VABD_i_A1_D";
    #[inline]
    pub const fn VABD_i_A1_D(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0111u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VABD_i_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000011101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VABD_i_A1_Q";
    #[inline]
    pub const fn VABD_i_A1_Q(
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
                | 0b0u32 << 23u32
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
pub mod VABA_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000011100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VABA_A1_D";
    #[inline]
    pub const fn VABA_A1_D(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0111u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VABA_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000011101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VABA_A1_Q";
    #[inline]
    pub const fn VABA_A1_Q(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0111u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod SHA1SU0_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010001100000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHA1SU0_A1";
    #[inline]
    pub const fn SHA1SU0_A1(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        Q: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1100u32 << 8u32
                | N.into_inner() << 7u32
                | Q.into_inner() << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VPADD_f_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111100010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000110100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VPADD_f_A1";
    #[inline]
    pub const fn VPADD_f_A1(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        Q: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1101u32 << 8u32
                | N.into_inner() << 7u32
                | Q.into_inner() << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMUL_f_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000110100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMUL_f_A1_D";
    #[inline]
    pub const fn VMUL_f_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1101u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMUL_f_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000110101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMUL_f_A1_Q";
    #[inline]
    pub const fn VMUL_f_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1101u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCGE_r_A2_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCGE_r_A2_D";
    #[inline]
    pub const fn VCGE_r_A2_D(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1110u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCGE_r_A2_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000111001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCGE_r_A2_Q";
    #[inline]
    pub const fn VCGE_r_A2_Q(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
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
pub mod VACGE_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000111000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VACGE_A1_D";
    #[inline]
    pub const fn VACGE_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1110u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VACGE_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000111001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VACGE_A1_Q";
    #[inline]
    pub const fn VACGE_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1110u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VPMAX_f_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VPMAX_f_A1";
    #[inline]
    pub const fn VPMAX_f_A1(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMAXNM_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000111100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMAXNM_A1_D";
    #[inline]
    pub const fn VMAXNM_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMAXNM_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMAXNM_A1_Q";
    #[inline]
    pub const fn VMAXNM_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VEOR_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000000100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VEOR_A1_D";
    #[inline]
    pub const fn VEOR_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VEOR_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000000101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VEOR_A1_Q";
    #[inline]
    pub const fn VEOR_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMUL_i_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000100100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMUL_i_A1_D";
    #[inline]
    pub const fn VMUL_i_A1_D(
        op: ::aarchmrs_types::BitValue<1>,
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
                | op.into_inner() << 24u32
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMUL_i_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000100101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMUL_i_A1_Q";
    #[inline]
    pub const fn VMUL_i_A1_Q(
        op: ::aarchmrs_types::BitValue<1>,
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
                | op.into_inner() << 24u32
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod SHA256H_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHA256H_A1";
    #[inline]
    pub const fn SHA256H_A1(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        Q: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1100u32 << 8u32
                | N.into_inner() << 7u32
                | Q.into_inner() << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VPMAX_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VPMAX_i_A1";
    #[inline]
    pub const fn VPMAX_i_A1(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VBSL_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000100000000000100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VBSL_A1_D";
    #[inline]
    pub const fn VBSL_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b01u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VBSL_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000100000000000101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VBSL_A1_Q";
    #[inline]
    pub const fn VBSL_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b01u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VPMIN_i_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110010000000000000101000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VPMIN_i_A1";
    #[inline]
    pub const fn VPMIN_i_A1(
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
                | 0b0u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod SHA256H2_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000100000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHA256H2_A1";
    #[inline]
    pub const fn SHA256H2_A1(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        Q: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b01u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1100u32 << 8u32
                | N.into_inner() << 7u32
                | Q.into_inner() << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VABD_f_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011001000000000110100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VABD_f_A1_D";
    #[inline]
    pub const fn VABD_f_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1101u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VABD_f_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011001000000000110101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VABD_f_A1_Q";
    #[inline]
    pub const fn VABD_f_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | sz.into_inner() << 20u32
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
pub mod VCGT_r_A2_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011001000000000111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCGT_r_A2_D";
    #[inline]
    pub const fn VCGT_r_A2_D(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1110u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCGT_r_A2_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011001000000000111001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCGT_r_A2_Q";
    #[inline]
    pub const fn VCGT_r_A2_Q(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | sz.into_inner() << 20u32
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
pub mod VACGT_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011001000000000111000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VACGT_A1_D";
    #[inline]
    pub const fn VACGT_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1110u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VACGT_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011001000000000111001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VACGT_A1_Q";
    #[inline]
    pub const fn VACGT_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1110u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VPMIN_f_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011001000000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VPMIN_f_A1";
    #[inline]
    pub const fn VPMIN_f_A1(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMINNM_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011001000000000111100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMINNM_A1_D";
    #[inline]
    pub const fn VMINNM_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMINNM_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011001000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMINNM_A1_Q";
    #[inline]
    pub const fn VMINNM_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | sz.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1111u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VSUB_i_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSUB_i_A1_D";
    #[inline]
    pub const fn VSUB_i_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1000u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VSUB_i_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000100001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSUB_i_A1_Q";
    #[inline]
    pub const fn VSUB_i_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1000u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VBIT_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011001000000000000100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VBIT_A1_D";
    #[inline]
    pub const fn VBIT_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VBIT_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011001000000000000101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VBIT_A1_Q";
    #[inline]
    pub const fn VBIT_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCEQ_r_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000100000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCEQ_r_A1_D";
    #[inline]
    pub const fn VCEQ_r_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1000u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCEQ_r_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000100001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCEQ_r_A1_Q";
    #[inline]
    pub const fn VCEQ_r_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1000u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMLS_i_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000100100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMLS_i_A1_D";
    #[inline]
    pub const fn VMLS_i_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMLS_i_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMLS_i_A1_Q";
    #[inline]
    pub const fn VMLS_i_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQRDMULH_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQRDMULH_A1_D";
    #[inline]
    pub const fn VQRDMULH_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQRDMULH_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQRDMULH_A1_Q";
    #[inline]
    pub const fn VQRDMULH_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
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
pub mod SHA256SU1_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011001000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHA256SU1_A1";
    #[inline]
    pub const fn SHA256SU1_A1(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        Q: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1100u32 << 8u32
                | N.into_inner() << 7u32
                | Q.into_inner() << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQRDMLAH_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000101100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQRDMLAH_A1_D";
    #[inline]
    pub const fn VQRDMLAH_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQRDMLAH_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000101101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQRDMLAH_A1_Q";
    #[inline]
    pub const fn VQRDMLAH_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VBIF_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011001100000000000100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VBIF_A1_D";
    #[inline]
    pub const fn VBIF_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VBIF_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011001100000000000101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VBIF_A1_Q";
    #[inline]
    pub const fn VBIF_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b0001u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQRDMLSH_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000110000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQRDMLSH_A1_D";
    #[inline]
    pub const fn VQRDMLSH_A1_D(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1100u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VQRDMLSH_A1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011000000000000110001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VQRDMLSH_A1_Q";
    #[inline]
    pub const fn VQRDMLSH_A1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100110u32 << 23u32
                | D.into_inner() << 22u32
                | size.into_inner() << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1100u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
