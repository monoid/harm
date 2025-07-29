/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod VCADD_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100100000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCADD_T1_D";
    #[inline]
    pub const fn VCADD_T1_D(
        rot: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111110u32 << 25u32
                | rot.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | S.into_inner() << 20u32
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
pub mod VCADD_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110101000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100100000000000100001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCADD_T1_Q";
    #[inline]
    pub const fn VCADD_T1_Q(
        rot: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111110u32 << 25u32
                | rot.into_inner() << 24u32
                | 0b1u32 << 23u32
                | D.into_inner() << 22u32
                | 0b0u32 << 21u32
                | S.into_inner() << 20u32
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
pub mod VMMLA_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100000000000000110001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMMLA_T1_Q";
    #[inline]
    pub const fn VMMLA_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111000u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
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
pub mod VDOT_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100000000000000110100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VDOT_T1_D";
    #[inline]
    pub const fn VDOT_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111000u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
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
pub mod VDOT_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100000000000000110101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VDOT_T1_Q";
    #[inline]
    pub const fn VDOT_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111000u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
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
pub mod VFMAL_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100001000000000100000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VFMAL_T1_D";
    #[inline]
    pub const fn VFMAL_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111000u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
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
pub mod VFMAL_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100001000000000100001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VFMAL_T1_Q";
    #[inline]
    pub const fn VFMAL_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111000u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
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
pub mod VSMMLA_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100001000000000110001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSMMLA_T1_Q";
    #[inline]
    pub const fn VSMMLA_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111000u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
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
pub mod VUMMLA_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100001000000000110001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VUMMLA_T1_Q";
    #[inline]
    pub const fn VUMMLA_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111000u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
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
pub mod VSDOT_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100001000000000110100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSDOT_T1_D";
    #[inline]
    pub const fn VSDOT_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111000u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
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
pub mod VSDOT_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100001000000000110101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSDOT_T1_Q";
    #[inline]
    pub const fn VSDOT_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111000u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
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
pub mod VUDOT_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100001000000000110100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VUDOT_T1_D";
    #[inline]
    pub const fn VUDOT_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111000u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
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
pub mod VUDOT_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100001000000000110101010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VUDOT_T1_Q";
    #[inline]
    pub const fn VUDOT_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111000u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
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
pub mod VFMA_bf_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100001100000000100000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VFMA_bf_T1_Q";
    #[inline]
    pub const fn VFMA_bf_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        Q: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111000u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | Vn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1000u32 << 8u32
                | N.into_inner() << 7u32
                | Q.into_inner() << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VFMSL_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100101000000000100000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VFMSL_T1_D";
    #[inline]
    pub const fn VFMSL_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111001u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
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
pub mod VFMSL_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100101000000000100001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VFMSL_T1_Q";
    #[inline]
    pub const fn VFMSL_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111001u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
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
pub mod VUSMMLA_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100101000000000110001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VUSMMLA_T1_Q";
    #[inline]
    pub const fn VUSMMLA_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111001u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
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
pub mod VUSDOT_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100101000000000110100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VUSDOT_T1_D";
    #[inline]
    pub const fn VUSDOT_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111001u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
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
pub mod VUSDOT_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100101000000000110101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VUSDOT_T1_Q";
    #[inline]
    pub const fn VUSDOT_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111001u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
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
pub mod VCMLA_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110001000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100001000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCMLA_T1_D";
    #[inline]
    pub const fn VCMLA_T1_D(
        rot: ::aarchmrs_types::BitValue<2>,
        D: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111110u32 << 25u32
                | rot.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | S.into_inner() << 20u32
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
pub mod VCMLA_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110001000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111100001000000000100001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCMLA_T1_Q";
    #[inline]
    pub const fn VCMLA_T1_Q(
        rot: ::aarchmrs_types::BitValue<2>,
        D: ::aarchmrs_types::BitValue<1>,
        S: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111110u32 << 25u32
                | rot.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b1u32 << 21u32
                | S.into_inner() << 20u32
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
