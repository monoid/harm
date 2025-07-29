/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod VABS_A2_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100000000100111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VABS_A2_H";
    #[inline]
    pub const fn VABS_A2_H(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110000u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VABS_A2_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100000000101011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VABS_A2_S";
    #[inline]
    pub const fn VABS_A2_S(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110000u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101011u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VABS_A2_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100000000101111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VABS_A2_D";
    #[inline]
    pub const fn VABS_A2_D(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110000u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMOV_r_A2_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100000000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_r_A2_S";
    #[inline]
    pub const fn VMOV_r_A2_S(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110000u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VMOV_r_A2_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100000000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VMOV_r_A2_D";
    #[inline]
    pub const fn VMOV_r_A2_D(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110000u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VNEG_A2_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100010000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VNEG_A2_H";
    #[inline]
    pub const fn VNEG_A2_H(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110001u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VNEG_A2_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100010000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VNEG_A2_S";
    #[inline]
    pub const fn VNEG_A2_S(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110001u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VNEG_A2_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100010000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VNEG_A2_D";
    #[inline]
    pub const fn VNEG_A2_D(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110001u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VSQRT_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100010000100111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSQRT_A1_H";
    #[inline]
    pub const fn VSQRT_A1_H(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110001u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VSQRT_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100010000101011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSQRT_A1_S";
    #[inline]
    pub const fn VSQRT_A1_S(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110001u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101011u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VSQRT_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100010000101111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSQRT_A1_D";
    #[inline]
    pub const fn VSQRT_A1_D(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110001u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTB_A1_SH {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100100000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTB_A1_SH";
    #[inline]
    pub const fn VCVTB_A1_SH(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110010u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTB_A1_DH {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100100000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTB_A1_DH";
    #[inline]
    pub const fn VCVTB_A1_DH(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110010u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTB_A1_HS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100110000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTB_A1_HS";
    #[inline]
    pub const fn VCVTB_A1_HS(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110011u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTB_A1_HD {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100110000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTB_A1_HD";
    #[inline]
    pub const fn VCVTB_A1_HD(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110011u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTT_A1_SH {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100100000101011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTT_A1_SH";
    #[inline]
    pub const fn VCVTT_A1_SH(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110010u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101011u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTT_A1_DH {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100100000101111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTT_A1_DH";
    #[inline]
    pub const fn VCVTT_A1_DH(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110010u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTT_A1_HS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100110000101011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTT_A1_HS";
    #[inline]
    pub const fn VCVTT_A1_HS(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110011u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101011u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTT_A1_HD {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100110000101111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTT_A1_HD";
    #[inline]
    pub const fn VCVTT_A1_HD(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110011u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTB_A1_bfs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100110000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTB_A1_bfs";
    #[inline]
    pub const fn VCVTB_A1_bfs(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110011u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTT_A1_bfs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101100110000100111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTT_A1_bfs";
    #[inline]
    pub const fn VCVTT_A1_bfs(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110011u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCMP_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101000000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCMP_A1_H";
    #[inline]
    pub const fn VCMP_A1_H(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110100u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCMP_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101000000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCMP_A1_S";
    #[inline]
    pub const fn VCMP_A1_S(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110100u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCMP_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101000000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCMP_A1_D";
    #[inline]
    pub const fn VCMP_A1_D(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110100u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCMPE_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101000000100111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCMPE_A1_H";
    #[inline]
    pub const fn VCMPE_A1_H(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110100u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCMPE_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101000000101011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCMPE_A1_S";
    #[inline]
    pub const fn VCMPE_A1_S(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110100u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101011u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCMPE_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101000000101111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCMPE_A1_D";
    #[inline]
    pub const fn VCMPE_A1_D(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110100u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCMP_A2_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101010000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000101111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCMP_A2_H";
    #[inline]
    pub const fn VCMP_A2_H(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110101u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100101000000u32 << 0u32,
        )
    }
}
pub mod VCMP_A2_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101010000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000101111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCMP_A2_S";
    #[inline]
    pub const fn VCMP_A2_S(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110101u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101001000000u32 << 0u32,
        )
    }
}
pub mod VCMP_A2_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101010000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000101111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCMP_A2_D";
    #[inline]
    pub const fn VCMP_A2_D(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110101u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101101000000u32 << 0u32,
        )
    }
}
pub mod VCMPE_A2_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101010000100111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000101111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCMPE_A2_H";
    #[inline]
    pub const fn VCMPE_A2_H(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110101u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100111000000u32 << 0u32,
        )
    }
}
pub mod VCMPE_A2_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101010000101011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000101111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCMPE_A2_S";
    #[inline]
    pub const fn VCMPE_A2_S(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110101u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101011000000u32 << 0u32,
        )
    }
}
pub mod VCMPE_A2_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101010000101111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000101111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCMPE_A2_D";
    #[inline]
    pub const fn VCMPE_A2_D(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110101u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101111000000u32 << 0u32,
        )
    }
}
pub mod VRINTR_vfp_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101100000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTR_vfp_A1_H";
    #[inline]
    pub const fn VRINTR_vfp_A1_H(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110110u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTR_vfp_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101100000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTR_vfp_A1_S";
    #[inline]
    pub const fn VRINTR_vfp_A1_S(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110110u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTR_vfp_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101100000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTR_vfp_A1_D";
    #[inline]
    pub const fn VRINTR_vfp_A1_D(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110110u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTZ_vfp_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101100000100111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTZ_vfp_A1_H";
    #[inline]
    pub const fn VRINTZ_vfp_A1_H(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110110u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTZ_vfp_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101100000101011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTZ_vfp_A1_S";
    #[inline]
    pub const fn VRINTZ_vfp_A1_S(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110110u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101011u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTZ_vfp_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101100000101111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTZ_vfp_A1_D";
    #[inline]
    pub const fn VRINTZ_vfp_A1_D(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110110u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTX_vfp_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101110000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTX_vfp_A1_H";
    #[inline]
    pub const fn VRINTX_vfp_A1_H(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110111u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTX_vfp_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101110000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTX_vfp_A1_S";
    #[inline]
    pub const fn VRINTX_vfp_A1_S(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110111u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VRINTX_vfp_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101110000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VRINTX_vfp_A1_D";
    #[inline]
    pub const fn VRINTX_vfp_A1_D(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110111u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_ds_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101110000101011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_ds_A1";
    #[inline]
    pub const fn VCVT_ds_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110111u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101011u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_sd_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101101110000101111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_sd_A1";
    #[inline]
    pub const fn VCVT_sd_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b110111u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_vi_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101110000000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_vi_A1_H";
    #[inline]
    pub const fn VCVT_vi_A1_H(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111000u32 << 16u32
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
pub mod VCVT_vi_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101110000000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_vi_A1_S";
    #[inline]
    pub const fn VCVT_vi_A1_S(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111000u32 << 16u32
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
pub mod VCVT_vi_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101110000000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_vi_A1_D";
    #[inline]
    pub const fn VCVT_vi_A1_D(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        op: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111000u32 << 16u32
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
pub mod VJCVT_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101110010000101111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VJCVT_A1";
    #[inline]
    pub const fn VJCVT_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111001u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_toxv_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111100000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101110100000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_toxv_A1_H";
    #[inline]
    pub const fn VCVT_toxv_A1_H(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        U: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        sx: ::aarchmrs_types::BitValue<1>,
        i: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11101u32 << 17u32
                | U.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1001u32 << 8u32
                | sx.into_inner() << 7u32
                | 0b1u32 << 6u32
                | i.into_inner() << 5u32
                | 0b0u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_xv_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111100000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101111100000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_xv_A1_H";
    #[inline]
    pub const fn VCVT_xv_A1_H(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        U: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        sx: ::aarchmrs_types::BitValue<1>,
        i: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11111u32 << 17u32
                | U.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1001u32 << 8u32
                | sx.into_inner() << 7u32
                | 0b1u32 << 6u32
                | i.into_inner() << 5u32
                | 0b0u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_toxv_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111100000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101110100000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_toxv_A1_S";
    #[inline]
    pub const fn VCVT_toxv_A1_S(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        U: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        sx: ::aarchmrs_types::BitValue<1>,
        i: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11101u32 << 17u32
                | U.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | sx.into_inner() << 7u32
                | 0b1u32 << 6u32
                | i.into_inner() << 5u32
                | 0b0u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_xv_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111100000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101111100000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_xv_A1_S";
    #[inline]
    pub const fn VCVT_xv_A1_S(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        U: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        sx: ::aarchmrs_types::BitValue<1>,
        i: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11111u32 << 17u32
                | U.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | sx.into_inner() << 7u32
                | 0b1u32 << 6u32
                | i.into_inner() << 5u32
                | 0b0u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_toxv_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111100000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101110100000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_toxv_A1_D";
    #[inline]
    pub const fn VCVT_toxv_A1_D(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        U: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        sx: ::aarchmrs_types::BitValue<1>,
        i: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11101u32 << 17u32
                | U.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | sx.into_inner() << 7u32
                | 0b1u32 << 6u32
                | i.into_inner() << 5u32
                | 0b0u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_xv_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111100000111101010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101111100000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_xv_A1_D";
    #[inline]
    pub const fn VCVT_xv_A1_D(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        U: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        sx: ::aarchmrs_types::BitValue<1>,
        i: ::aarchmrs_types::BitValue<1>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11111u32 << 17u32
                | U.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | sx.into_inner() << 7u32
                | 0b1u32 << 6u32
                | i.into_inner() << 5u32
                | 0b0u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
pub mod VCVTR_uiv_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101111000000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTR_uiv_A1_H";
    #[inline]
    pub const fn VCVTR_uiv_A1_H(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111100u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTR_siv_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101111010000100101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTR_siv_A1_H";
    #[inline]
    pub const fn VCVTR_siv_A1_H(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111101u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTR_uiv_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101111000000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTR_uiv_A1_S";
    #[inline]
    pub const fn VCVTR_uiv_A1_S(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111100u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTR_siv_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101111010000101001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTR_siv_A1_S";
    #[inline]
    pub const fn VCVTR_siv_A1_S(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111101u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101001u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTR_uiv_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101111000000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTR_uiv_A1_D";
    #[inline]
    pub const fn VCVTR_uiv_A1_D(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111100u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVTR_siv_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101111010000101101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVTR_siv_A1_D";
    #[inline]
    pub const fn VCVTR_siv_A1_D(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111101u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101101u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_uiv_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101111000000100111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_uiv_A1_H";
    #[inline]
    pub const fn VCVT_uiv_A1_H(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111100u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_siv_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101111010000100111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_siv_A1_H";
    #[inline]
    pub const fn VCVT_siv_A1_H(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111101u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b100111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_uiv_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101111000000101011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_uiv_A1_S";
    #[inline]
    pub const fn VCVT_uiv_A1_S(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111100u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101011u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_siv_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101111010000101011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_siv_A1_S";
    #[inline]
    pub const fn VCVT_siv_A1_S(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111101u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101011u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_uiv_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101111000000101111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_uiv_A1_D";
    #[inline]
    pub const fn VCVT_uiv_A1_D(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111100u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
pub mod VCVT_siv_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101111110000111111010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101111010000101111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VCVT_siv_A1_D";
    #[inline]
    pub const fn VCVT_siv_A1_D(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
        Vm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11101u32 << 23u32
                | D.into_inner() << 22u32
                | 0b111101u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b101111u32 << 6u32
                | M.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Vm.into_inner() << 0u32,
        )
    }
}
