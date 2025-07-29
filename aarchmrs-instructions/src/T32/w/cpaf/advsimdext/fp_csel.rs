/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod VSELEQ_T1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110000000000000100100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSELEQ_T1_H";
    #[inline]
    pub const fn VSELEQ_T1_H(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
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
pub mod VSELEQ_T1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110000000000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSELEQ_T1_S";
    #[inline]
    pub const fn VSELEQ_T1_S(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
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
pub mod VSELEQ_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110000000000000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSELEQ_T1_D";
    #[inline]
    pub const fn VSELEQ_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
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
pub mod VSELGE_T1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110001000000000100100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSELGE_T1_H";
    #[inline]
    pub const fn VSELGE_T1_H(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
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
pub mod VSELGE_T1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110001000000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSELGE_T1_S";
    #[inline]
    pub const fn VSELGE_T1_S(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
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
pub mod VSELGE_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110001000000000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSELGE_T1_D";
    #[inline]
    pub const fn VSELGE_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
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
pub mod VSELGT_T1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110001100000000100100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSELGT_T1_H";
    #[inline]
    pub const fn VSELGT_T1_H(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
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
pub mod VSELGT_T1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110001100000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSELGT_T1_S";
    #[inline]
    pub const fn VSELGT_T1_S(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
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
pub mod VSELGT_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110001100000000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSELGT_T1_D";
    #[inline]
    pub const fn VSELGT_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
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
pub mod VSELVS_T1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110000100000000100100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSELVS_T1_H";
    #[inline]
    pub const fn VSELVS_T1_H(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b01u32 << 20u32
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
pub mod VSELVS_T1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110000100000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSELVS_T1_S";
    #[inline]
    pub const fn VSELVS_T1_S(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b01u32 << 20u32
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
pub mod VSELVS_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110000100000000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSELVS_T1_D";
    #[inline]
    pub const fn VSELVS_T1_D(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111100u32 << 23u32
                | D.into_inner() << 22u32
                | 0b01u32 << 20u32
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
