/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod VCMLA_s_T1_DH {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110000000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCMLA_s_T1_DH";
    #[inline]
    pub const fn VCMLA_s_T1_DH(
        D: ::aarchmrs_types::BitValue<1>,
        rot: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111100u32 << 23u32
                | D.into_inner() << 22u32
                | rot.into_inner() << 20u32
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
pub mod VCMLA_s_T1_DS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110100000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCMLA_s_T1_DS";
    #[inline]
    pub const fn VCMLA_s_T1_DS(
        D: ::aarchmrs_types::BitValue<1>,
        rot: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | rot.into_inner() << 20u32
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
pub mod VCMLA_s_T1_QH {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110000000000000100001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCMLA_s_T1_QH";
    #[inline]
    pub const fn VCMLA_s_T1_QH(
        D: ::aarchmrs_types::BitValue<1>,
        rot: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111100u32 << 23u32
                | D.into_inner() << 22u32
                | rot.into_inner() << 20u32
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
pub mod VCMLA_s_T1_QS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110100000000000100001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCMLA_s_T1_QS";
    #[inline]
    pub const fn VCMLA_s_T1_QS(
        D: ::aarchmrs_types::BitValue<1>,
        rot: ::aarchmrs_types::BitValue<2>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111101u32 << 23u32
                | D.into_inner() << 22u32
                | rot.into_inner() << 20u32
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
pub mod VFMAL_s_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110000000000000100000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VFMAL_s_T1_D";
    #[inline]
    pub const fn VFMAL_s_T1_D(
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
                | 0b1000u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VFMAL_s_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110000000000000100001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VFMAL_s_T1_Q";
    #[inline]
    pub const fn VFMAL_s_T1_Q(
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
                | 0b1000u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VFMSL_s_T1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110000100000000100000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VFMSL_s_T1_D";
    #[inline]
    pub const fn VFMSL_s_T1_D(
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
                | 0b1000u32 << 8u32
                | N.into_inner() << 7u32
                | 0b0u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VFMSL_s_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110000100000000100001010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VFMSL_s_T1_Q";
    #[inline]
    pub const fn VFMSL_s_T1_Q(
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
                | 0b1000u32 << 8u32
                | N.into_inner() << 7u32
                | 0b1u32 << 6u32
                | M.into_inner() << 5u32
                | 0b1u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VFMA_bfs_T1_Q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101100000000111100010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111110001100000000100000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VFMA_bfs_T1_Q";
    #[inline]
    pub const fn VFMA_bfs_T1_Q(
        D: ::aarchmrs_types::BitValue<1>,
        Vn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        N: ::aarchmrs_types::BitValue<1>,
        Q: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111111100u32 << 23u32
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
