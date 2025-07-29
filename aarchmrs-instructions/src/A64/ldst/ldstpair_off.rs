/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STP_32_ldstpair_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101001000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STP_32_ldstpair_off";
    #[inline]
    pub const fn STP_32_ldstpair_off(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010100100u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDP_32_ldstpair_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101001010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDP_32_ldstpair_off";
    #[inline]
    pub const fn LDP_32_ldstpair_off(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010100101u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STP_S_ldstpair_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101101000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STP_S_ldstpair_off";
    #[inline]
    pub const fn STP_S_ldstpair_off(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010110100u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDP_S_ldstpair_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101101010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDP_S_ldstpair_off";
    #[inline]
    pub const fn LDP_S_ldstpair_off(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010110101u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STGP_64_ldstpair_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01101001000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STGP_64_ldstpair_off";
    #[inline]
    pub const fn STGP_64_ldstpair_off(
        simm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110100100u32 << 22u32
                | simm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDPSW_64_ldstpair_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01101001010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDPSW_64_ldstpair_off";
    #[inline]
    pub const fn LDPSW_64_ldstpair_off(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110100101u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STP_D_ldstpair_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01101101000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STP_D_ldstpair_off";
    #[inline]
    pub const fn STP_D_ldstpair_off(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110110100u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDP_D_ldstpair_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01101101010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDP_D_ldstpair_off";
    #[inline]
    pub const fn LDP_D_ldstpair_off(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110110101u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STP_64_ldstpair_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10101001000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STP_64_ldstpair_off";
    #[inline]
    pub const fn STP_64_ldstpair_off(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010100100u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDP_64_ldstpair_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10101001010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDP_64_ldstpair_off";
    #[inline]
    pub const fn LDP_64_ldstpair_off(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010100101u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STP_Q_ldstpair_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10101101000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STP_Q_ldstpair_off";
    #[inline]
    pub const fn STP_Q_ldstpair_off(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010110100u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDP_Q_ldstpair_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10101101010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDP_Q_ldstpair_off";
    #[inline]
    pub const fn LDP_Q_ldstpair_off(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1010110101u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STTP_64_ldstpair_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101001000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STTP_64_ldstpair_off";
    #[inline]
    pub const fn STTP_64_ldstpair_off(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110100100u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDTP_64_ldstpair_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101001010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDTP_64_ldstpair_off";
    #[inline]
    pub const fn LDTP_64_ldstpair_off(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110100101u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STTP_Q_ldstpair_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STTP_Q_ldstpair_off";
    #[inline]
    pub const fn STTP_Q_ldstpair_off(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110110100u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDTP_Q_ldstpair_off {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101101010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDTP_Q_ldstpair_off";
    #[inline]
    pub const fn LDTP_Q_ldstpair_off(
        imm7: ::aarchmrs_types::BitValue<7>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110110101u32 << 22u32
                | imm7.into_inner() << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
