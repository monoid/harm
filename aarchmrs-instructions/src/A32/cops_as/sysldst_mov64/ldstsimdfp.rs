/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod VSTMDB_A2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101001000000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSTMDB_A2";
    #[inline]
    pub const fn VSTMDB_A2(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VSTM_A2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111100100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100000000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSTM_A2";
    #[inline]
    pub const fn VSTM_A2(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11001u32 << 23u32
                | D.into_inner() << 22u32
                | W.into_inner() << 21u32
                | 0b0u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VSTMDB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101100000000111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101001000000000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSTMDB_A1";
    #[inline]
    pub const fn VSTMDB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<7>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod VSTM_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111100100000000111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100000000000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSTM_A1";
    #[inline]
    pub const fn VSTM_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<7>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11001u32 << 23u32
                | D.into_inner() << 22u32
                | W.into_inner() << 21u32
                | 0b0u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod FSTMDBX_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101100000000111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101001000000000101100000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FSTMDBX_A1";
    #[inline]
    pub const fn FSTMDBX_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<7>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b10u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod FSTMIAX_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111100100000000111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100000000000101100000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FSTMIAX_A1";
    #[inline]
    pub const fn FSTMIAX_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<7>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11001u32 << 23u32
                | D.into_inner() << 22u32
                | W.into_inner() << 21u32
                | 0b0u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod VLDMDB_A2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101001100000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDMDB_A2";
    #[inline]
    pub const fn VLDMDB_A2(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VLDM_A2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111100100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100100000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDM_A2";
    #[inline]
    pub const fn VLDM_A2(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11001u32 << 23u32
                | D.into_inner() << 22u32
                | W.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VLDMDB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101100000000111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101001100000000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDMDB_A1";
    #[inline]
    pub const fn VLDMDB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<7>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod VLDM_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111100100000000111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100100000000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDM_A1";
    #[inline]
    pub const fn VLDM_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<7>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11001u32 << 23u32
                | D.into_inner() << 22u32
                | W.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod FLDMDBX_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111101100000000111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101001100000000101100000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FLDMDBX_A1";
    #[inline]
    pub const fn FLDMDBX_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<7>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11010u32 << 23u32
                | D.into_inner() << 22u32
                | 0b11u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod FLDMIAX_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111100100000000111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100100000000101100000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FLDMIAX_A1";
    #[inline]
    pub const fn FLDMIAX_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        D: ::aarchmrs_types::BitValue<1>,
        W: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<7>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b11001u32 << 23u32
                | D.into_inner() << 22u32
                | W.into_inner() << 21u32
                | 0b1u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
pub mod VSTR_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111001100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101000000000000100100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSTR_A1_H";
    #[inline]
    pub const fn VSTR_A1_H(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b1101u32 << 24u32
                | U.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1001u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VSTR_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111001100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101000000000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSTR_A1_S";
    #[inline]
    pub const fn VSTR_A1_S(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b1101u32 << 24u32
                | U.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VSTR_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111001100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101000000000000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VSTR_A1_D";
    #[inline]
    pub const fn VSTR_A1_D(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b1101u32 << 24u32
                | U.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b00u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VLDR_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111001100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101000100000000100100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDR_A1_H";
    #[inline]
    pub const fn VLDR_A1_H(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b1101u32 << 24u32
                | U.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b01u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1001u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VLDR_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111001100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101000100000000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDR_A1_S";
    #[inline]
    pub const fn VLDR_A1_S(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b1101u32 << 24u32
                | U.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b01u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VLDR_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111001100000000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101000100000000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDR_A1_D";
    #[inline]
    pub const fn VLDR_A1_D(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b1101u32 << 24u32
                | U.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b01u32 << 20u32
                | Rn.into_inner() << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VLDR_l_A1_H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111001111110000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101000111110000100100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDR_l_A1_H";
    #[inline]
    pub const fn VLDR_l_A1_H(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b1101u32 << 24u32
                | U.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b011111u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b1001u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VLDR_l_A1_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111001111110000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101000111110000101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDR_l_A1_S";
    #[inline]
    pub const fn VLDR_l_A1_S(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b1101u32 << 24u32
                | U.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b011111u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b1010u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod VLDR_l_A1_D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111001111110000111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001101000111110000101100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "VLDR_l_A1_D";
    #[inline]
    pub const fn VLDR_l_A1_D(
        cond: ::aarchmrs_types::BitValue<4>,
        U: ::aarchmrs_types::BitValue<1>,
        D: ::aarchmrs_types::BitValue<1>,
        Vd: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b1101u32 << 24u32
                | U.into_inner() << 23u32
                | D.into_inner() << 22u32
                | 0b011111u32 << 16u32
                | Vd.into_inner() << 12u32
                | 0b1011u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
