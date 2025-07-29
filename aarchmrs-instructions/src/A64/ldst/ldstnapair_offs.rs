/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STNP_32_ldstnapair_offs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STNP_32_ldstnapair_offs";
    #[inline]
    pub const fn STNP_32_ldstnapair_offs(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010100000u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDNP_32_ldstnapair_offs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101000010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDNP_32_ldstnapair_offs";
    #[inline]
    pub const fn LDNP_32_ldstnapair_offs(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010100001u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STNP_S_ldstnapair_offs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101100000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STNP_S_ldstnapair_offs";
    #[inline]
    pub const fn STNP_S_ldstnapair_offs(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010110000u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDNP_S_ldstnapair_offs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101100010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDNP_S_ldstnapair_offs";
    #[inline]
    pub const fn LDNP_S_ldstnapair_offs(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010110001u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STNP_D_ldstnapair_offs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01101100000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STNP_D_ldstnapair_offs";
    #[inline]
    pub const fn STNP_D_ldstnapair_offs(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110110000u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDNP_D_ldstnapair_offs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01101100010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDNP_D_ldstnapair_offs";
    #[inline]
    pub const fn LDNP_D_ldstnapair_offs(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110110001u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STNP_64_ldstnapair_offs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10101000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STNP_64_ldstnapair_offs";
    #[inline]
    pub const fn STNP_64_ldstnapair_offs(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010100000u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDNP_64_ldstnapair_offs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10101000010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDNP_64_ldstnapair_offs";
    #[inline]
    pub const fn LDNP_64_ldstnapair_offs(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010100001u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STNP_Q_ldstnapair_offs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10101100000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STNP_Q_ldstnapair_offs";
    #[inline]
    pub const fn STNP_Q_ldstnapair_offs(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010110000u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDNP_Q_ldstnapair_offs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10101100010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDNP_Q_ldstnapair_offs";
    #[inline]
    pub const fn LDNP_Q_ldstnapair_offs(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010110001u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STTNP_64_ldstnapair_offs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STTNP_64_ldstnapair_offs";
    #[inline]
    pub const fn STTNP_64_ldstnapair_offs(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110100000u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDTNP_64_ldstnapair_offs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDTNP_64_ldstnapair_offs";
    #[inline]
    pub const fn LDTNP_64_ldstnapair_offs(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110100001u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STTNP_Q_ldstnapair_offs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101100000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STTNP_Q_ldstnapair_offs";
    #[inline]
    pub const fn STTNP_Q_ldstnapair_offs(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110110000u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDTNP_Q_ldstnapair_offs {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101100010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDTNP_Q_ldstnapair_offs";
    #[inline]
    pub const fn LDTNP_Q_ldstnapair_offs(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110110001u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
