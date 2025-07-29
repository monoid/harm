/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STR_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000101000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STR_r_T1";
    #[inline]
    pub const fn STR_r_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<3>,
        Rt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101000u32 << 9u32
                | Rm.into_inner() << 6u32
                | Rn.into_inner() << 3u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STRH_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000101001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRH_r_T1";
    #[inline]
    pub const fn STRH_r_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<3>,
        Rt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101001u32 << 9u32
                | Rm.into_inner() << 6u32
                | Rn.into_inner() << 3u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STRB_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STRB_r_T1";
    #[inline]
    pub const fn STRB_r_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<3>,
        Rt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101010u32 << 9u32
                | Rm.into_inner() << 6u32
                | Rn.into_inner() << 3u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDRSB_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000101011000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSB_r_T1";
    #[inline]
    pub const fn LDRSB_r_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<3>,
        Rt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101011u32 << 9u32
                | Rm.into_inner() << 6u32
                | Rn.into_inner() << 3u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDR_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000101100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDR_r_T1";
    #[inline]
    pub const fn LDR_r_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<3>,
        Rt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101100u32 << 9u32
                | Rm.into_inner() << 6u32
                | Rn.into_inner() << 3u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDRH_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000101101000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRH_r_T1";
    #[inline]
    pub const fn LDRH_r_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<3>,
        Rt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101101u32 << 9u32
                | Rm.into_inner() << 6u32
                | Rn.into_inner() << 3u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDRB_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000101110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRB_r_T1";
    #[inline]
    pub const fn LDRB_r_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<3>,
        Rt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101110u32 << 9u32
                | Rm.into_inner() << 6u32
                | Rn.into_inner() << 3u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDRSH_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000101111000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDRSH_r_T1";
    #[inline]
    pub const fn LDRSH_r_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<3>,
        Rt: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101111u32 << 9u32
                | Rm.into_inner() << 6u32
                | Rn.into_inner() << 3u32
                | Rt.into_inner() << 0u32,
        )
    }
}
