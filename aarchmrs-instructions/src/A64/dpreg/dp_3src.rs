/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MADD_32A_dp_3src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011011000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MADD_32A_dp_3src";
    #[inline]
    pub const fn MADD_32A_dp_3src(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011011000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MSUB_32A_dp_3src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011011000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MSUB_32A_dp_3src";
    #[inline]
    pub const fn MSUB_32A_dp_3src(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011011000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MADD_64A_dp_3src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011011000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MADD_64A_dp_3src";
    #[inline]
    pub const fn MADD_64A_dp_3src(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011011000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MSUB_64A_dp_3src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011011000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MSUB_64A_dp_3src";
    #[inline]
    pub const fn MSUB_64A_dp_3src(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011011000u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SMADDL_64WA_dp_3src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011011001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMADDL_64WA_dp_3src";
    #[inline]
    pub const fn SMADDL_64WA_dp_3src(
        U: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011011u32 << 24u32
                | U.into_inner() << 23u32
                | 0b01u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SMSUBL_64WA_dp_3src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011011001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMSUBL_64WA_dp_3src";
    #[inline]
    pub const fn SMSUBL_64WA_dp_3src(
        U: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011011u32 << 24u32
                | U.into_inner() << 23u32
                | 0b01u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SMULH_64_dp_3src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011011010000000111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMULH_64_dp_3src";
    #[inline]
    pub const fn SMULH_64_dp_3src(
        U: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011011u32 << 24u32
                | U.into_inner() << 23u32
                | 0b10u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b011111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MADDPT_64A_dp_3src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011011011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MADDPT_64A_dp_3src";
    #[inline]
    pub const fn MADDPT_64A_dp_3src(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011011011u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MSUBPT_64A_dp_3src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011011011000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MSUBPT_64A_dp_3src";
    #[inline]
    pub const fn MSUBPT_64A_dp_3src(
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011011011u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UMADDL_64WA_dp_3src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011011001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UMADDL_64WA_dp_3src";
    #[inline]
    pub const fn UMADDL_64WA_dp_3src(
        U: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011011u32 << 24u32
                | U.into_inner() << 23u32
                | 0b01u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UMSUBL_64WA_dp_3src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011011001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UMSUBL_64WA_dp_3src";
    #[inline]
    pub const fn UMSUBL_64WA_dp_3src(
        U: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        Ra: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011011u32 << 24u32
                | U.into_inner() << 23u32
                | 0b01u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Ra.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UMULH_64_dp_3src {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111011000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011011010000000111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000111110000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UMULH_64_dp_3src";
    #[inline]
    pub const fn UMULH_64_dp_3src(
        U: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011011u32 << 24u32
                | U.into_inner() << 23u32
                | 0b10u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b011111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
