/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FCVTNS_32S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNS_32S_float2int";
    #[inline]
    pub const fn FCVTNS_32S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000100000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNU_32S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001000010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNU_32S_float2int";
    #[inline]
    pub const fn FCVTNU_32S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000100001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SCVTF_S32_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001000100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_S32_float2int";
    #[inline]
    pub const fn SCVTF_S32_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000100010000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_S32_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001000110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_S32_float2int";
    #[inline]
    pub const fn UCVTF_S32_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000100011000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAS_32S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001001000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAS_32S_float2int";
    #[inline]
    pub const fn FCVTAS_32S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000100100000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAU_32S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001001010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAU_32S_float2int";
    #[inline]
    pub const fn FCVTAU_32S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000100101000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMOV_32S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001001100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMOV_32S_float2int";
    #[inline]
    pub const fn FMOV_32S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000100110000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMOV_S32_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001001110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMOV_S32_float2int";
    #[inline]
    pub const fn FMOV_S32_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000100111000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPS_32S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001010000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPS_32S_float2int";
    #[inline]
    pub const fn FCVTPS_32S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000101000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPU_32S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001010010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPU_32S_float2int";
    #[inline]
    pub const fn FCVTPU_32S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000101001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMS_32S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMS_32S_float2int";
    #[inline]
    pub const fn FCVTMS_32S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000110000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMU_32S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001100010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMU_32S_float2int";
    #[inline]
    pub const fn FCVTMU_32S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000110001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_32S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001110000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_32S_float2int";
    #[inline]
    pub const fn FCVTZS_32S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000111000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_32S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110001110010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_32S_float2int";
    #[inline]
    pub const fn FCVTZU_32S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000111001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNS_32D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNS_32D_float2int";
    #[inline]
    pub const fn FCVTNS_32D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001100000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNU_32D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011000010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNU_32D_float2int";
    #[inline]
    pub const fn FCVTNU_32D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001100001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SCVTF_D32_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011000100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_D32_float2int";
    #[inline]
    pub const fn SCVTF_D32_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001100010000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_D32_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011000110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_D32_float2int";
    #[inline]
    pub const fn UCVTF_D32_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001100011000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAS_32D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011001000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAS_32D_float2int";
    #[inline]
    pub const fn FCVTAS_32D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001100100000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAU_32D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011001010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAU_32D_float2int";
    #[inline]
    pub const fn FCVTAU_32D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001100101000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPS_32D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011010000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPS_32D_float2int";
    #[inline]
    pub const fn FCVTPS_32D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001101000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPU_32D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011010010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPU_32D_float2int";
    #[inline]
    pub const fn FCVTPU_32D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001101001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMS_32D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMS_32D_float2int";
    #[inline]
    pub const fn FCVTMS_32D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001110000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMU_32D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011100010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMU_32D_float2int";
    #[inline]
    pub const fn FCVTMU_32D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001110001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_32D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011110000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_32D_float2int";
    #[inline]
    pub const fn FCVTZS_32D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001111000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_32D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011110010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_32D_float2int";
    #[inline]
    pub const fn FCVTZU_32D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001111001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FJCVTZS_32D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011111100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FJCVTZS_32D_float2int";
    #[inline]
    pub const fn FJCVTZS_32D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001111110000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNS_32H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNS_32H_float2int";
    #[inline]
    pub const fn FCVTNS_32H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011100000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNU_32H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111000010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNU_32H_float2int";
    #[inline]
    pub const fn FCVTNU_32H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011100001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SCVTF_H32_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111000100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_H32_float2int";
    #[inline]
    pub const fn SCVTF_H32_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011100010000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_H32_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111000110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_H32_float2int";
    #[inline]
    pub const fn UCVTF_H32_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011100011000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAS_32H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111001000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAS_32H_float2int";
    #[inline]
    pub const fn FCVTAS_32H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011100100000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAU_32H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111001010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAU_32H_float2int";
    #[inline]
    pub const fn FCVTAU_32H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011100101000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMOV_32H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111001100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMOV_32H_float2int";
    #[inline]
    pub const fn FMOV_32H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011100110000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMOV_H32_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111001110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMOV_H32_float2int";
    #[inline]
    pub const fn FMOV_H32_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011100111000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPS_32H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111010000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPS_32H_float2int";
    #[inline]
    pub const fn FCVTPS_32H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011101000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPU_32H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111010010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPU_32H_float2int";
    #[inline]
    pub const fn FCVTPU_32H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011101001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMS_32H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMS_32H_float2int";
    #[inline]
    pub const fn FCVTMS_32H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011110000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMU_32H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111100010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMU_32H_float2int";
    #[inline]
    pub const fn FCVTMU_32H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011110001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_32H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111110000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_32H_float2int";
    #[inline]
    pub const fn FCVTZS_32H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011111000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_32H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111110010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_32H_float2int";
    #[inline]
    pub const fn FCVTZU_32H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011111001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNS_64S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNS_64S_float2int";
    #[inline]
    pub const fn FCVTNS_64S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000100000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNU_64S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001000010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNU_64S_float2int";
    #[inline]
    pub const fn FCVTNU_64S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000100001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SCVTF_S64_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001000100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_S64_float2int";
    #[inline]
    pub const fn SCVTF_S64_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000100010000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_S64_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001000110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_S64_float2int";
    #[inline]
    pub const fn UCVTF_S64_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000100011000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAS_64S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001001000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAS_64S_float2int";
    #[inline]
    pub const fn FCVTAS_64S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000100100000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAU_64S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001001010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAU_64S_float2int";
    #[inline]
    pub const fn FCVTAU_64S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000100101000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPS_64S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001010000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPS_64S_float2int";
    #[inline]
    pub const fn FCVTPS_64S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000101000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPU_64S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001010010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPU_64S_float2int";
    #[inline]
    pub const fn FCVTPU_64S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000101001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMS_64S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMS_64S_float2int";
    #[inline]
    pub const fn FCVTMS_64S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000110000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMU_64S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001100010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMU_64S_float2int";
    #[inline]
    pub const fn FCVTMU_64S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000110001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_64S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001110000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_64S_float2int";
    #[inline]
    pub const fn FCVTZS_64S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000111000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_64S_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001110010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_64S_float2int";
    #[inline]
    pub const fn FCVTZU_64S_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000111001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNS_64D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110011000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNS_64D_float2int";
    #[inline]
    pub const fn FCVTNS_64D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111001100000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNU_64D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110011000010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNU_64D_float2int";
    #[inline]
    pub const fn FCVTNU_64D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111001100001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SCVTF_D64_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110011000100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_D64_float2int";
    #[inline]
    pub const fn SCVTF_D64_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111001100010000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_D64_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110011000110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_D64_float2int";
    #[inline]
    pub const fn UCVTF_D64_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111001100011000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAS_64D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110011001000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAS_64D_float2int";
    #[inline]
    pub const fn FCVTAS_64D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111001100100000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAU_64D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110011001010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAU_64D_float2int";
    #[inline]
    pub const fn FCVTAU_64D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111001100101000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMOV_64D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110011001100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMOV_64D_float2int";
    #[inline]
    pub const fn FMOV_64D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111001100110000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMOV_D64_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110011001110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMOV_D64_float2int";
    #[inline]
    pub const fn FMOV_D64_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111001100111000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPS_64D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110011010000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPS_64D_float2int";
    #[inline]
    pub const fn FCVTPS_64D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111001101000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPU_64D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110011010010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPU_64D_float2int";
    #[inline]
    pub const fn FCVTPU_64D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111001101001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMS_64D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110011100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMS_64D_float2int";
    #[inline]
    pub const fn FCVTMS_64D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111001110000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMU_64D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110011100010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMU_64D_float2int";
    #[inline]
    pub const fn FCVTMU_64D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111001110001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_64D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110011110000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_64D_float2int";
    #[inline]
    pub const fn FCVTZS_64D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111001111000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_64D_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110011110010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_64D_float2int";
    #[inline]
    pub const fn FCVTZU_64D_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111001111001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMOV_64VX_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110101011100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMOV_64VX_float2int";
    #[inline]
    pub const fn FMOV_64VX_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111010101110000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMOV_V64I_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110101011110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMOV_V64I_float2int";
    #[inline]
    pub const fn FMOV_V64I_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111010101111000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNS_64H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNS_64H_float2int";
    #[inline]
    pub const fn FCVTNS_64H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011100000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNU_64H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111000010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNU_64H_float2int";
    #[inline]
    pub const fn FCVTNU_64H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011100001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SCVTF_H64_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111000100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_H64_float2int";
    #[inline]
    pub const fn SCVTF_H64_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011100010000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_H64_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111000110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_H64_float2int";
    #[inline]
    pub const fn UCVTF_H64_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011100011000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAS_64H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111001000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAS_64H_float2int";
    #[inline]
    pub const fn FCVTAS_64H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011100100000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAU_64H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111001010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAU_64H_float2int";
    #[inline]
    pub const fn FCVTAU_64H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011100101000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMOV_64H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111001100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMOV_64H_float2int";
    #[inline]
    pub const fn FMOV_64H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011100110000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMOV_H64_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111001110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMOV_H64_float2int";
    #[inline]
    pub const fn FMOV_H64_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011100111000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPS_64H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111010000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPS_64H_float2int";
    #[inline]
    pub const fn FCVTPS_64H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011101000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPU_64H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111010010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPU_64H_float2int";
    #[inline]
    pub const fn FCVTPU_64H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011101001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMS_64H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMS_64H_float2int";
    #[inline]
    pub const fn FCVTMS_64H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011110000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMU_64H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111100010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMU_64H_float2int";
    #[inline]
    pub const fn FCVTMU_64H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011110001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_64H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111110000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_64H_float2int";
    #[inline]
    pub const fn FCVTZS_64H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011111000000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_64H_float2int {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111110010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_64H_float2int";
    #[inline]
    pub const fn FCVTZU_64H_float2int(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011111001000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNS_sisd_32D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011010100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNS_sisd_32D";
    #[inline]
    pub const fn FCVTNS_sisd_32D(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001101010000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAS_sisd_32D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011110100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAS_sisd_32D";
    #[inline]
    pub const fn FCVTAS_sisd_32D(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001111010000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPS_sisd_32D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011100100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPS_sisd_32D";
    #[inline]
    pub const fn FCVTPS_sisd_32D(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001110010000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMS_sisd_32D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011101000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMS_sisd_32D";
    #[inline]
    pub const fn FCVTMS_sisd_32D(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001110100000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_sisd_32D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011101100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_sisd_32D";
    #[inline]
    pub const fn FCVTZS_sisd_32D(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001110110000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SCVTF_sisd_32D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011111000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_sisd_32D";
    #[inline]
    pub const fn SCVTF_sisd_32D(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001111100000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNU_sisd_32D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011010110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNU_sisd_32D";
    #[inline]
    pub const fn FCVTNU_sisd_32D(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001101011000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAU_sisd_32D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011110110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAU_sisd_32D";
    #[inline]
    pub const fn FCVTAU_sisd_32D(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001111011000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPU_sisd_32D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011100110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPU_sisd_32D";
    #[inline]
    pub const fn FCVTPU_sisd_32D(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001110011000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMU_sisd_32D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011101010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMU_sisd_32D";
    #[inline]
    pub const fn FCVTMU_sisd_32D(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001110101000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_sisd_32D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011101110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_sisd_32D";
    #[inline]
    pub const fn FCVTZU_sisd_32D(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001110111000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_sisd_32D {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110011111010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_sisd_32D";
    #[inline]
    pub const fn UCVTF_sisd_32D(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001111101000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNS_sisd_32H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111010100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNS_sisd_32H";
    #[inline]
    pub const fn FCVTNS_sisd_32H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011101010000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAS_sisd_32H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111110100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAS_sisd_32H";
    #[inline]
    pub const fn FCVTAS_sisd_32H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011111010000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPS_sisd_32H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111100100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPS_sisd_32H";
    #[inline]
    pub const fn FCVTPS_sisd_32H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011110010000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMS_sisd_32H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111101000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMS_sisd_32H";
    #[inline]
    pub const fn FCVTMS_sisd_32H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011110100000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_sisd_32H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111101100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_sisd_32H";
    #[inline]
    pub const fn FCVTZS_sisd_32H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011110110000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SCVTF_sisd_32H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111111000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_sisd_32H";
    #[inline]
    pub const fn SCVTF_sisd_32H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011111100000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNU_sisd_32H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111010110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNU_sisd_32H";
    #[inline]
    pub const fn FCVTNU_sisd_32H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011101011000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAU_sisd_32H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111110110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAU_sisd_32H";
    #[inline]
    pub const fn FCVTAU_sisd_32H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011111011000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPU_sisd_32H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111100110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPU_sisd_32H";
    #[inline]
    pub const fn FCVTPU_sisd_32H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011110011000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMU_sisd_32H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111101010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMU_sisd_32H";
    #[inline]
    pub const fn FCVTMU_sisd_32H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011110101000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_sisd_32H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111101110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_sisd_32H";
    #[inline]
    pub const fn FCVTZU_sisd_32H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011110111000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_sisd_32H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110111111010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_sisd_32H";
    #[inline]
    pub const fn UCVTF_sisd_32H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011111101000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNS_sisd_64H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111010100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNS_sisd_64H";
    #[inline]
    pub const fn FCVTNS_sisd_64H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011101010000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAS_sisd_64H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111110100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAS_sisd_64H";
    #[inline]
    pub const fn FCVTAS_sisd_64H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011111010000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPS_sisd_64H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111100100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPS_sisd_64H";
    #[inline]
    pub const fn FCVTPS_sisd_64H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011110010000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMS_sisd_64H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111101000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMS_sisd_64H";
    #[inline]
    pub const fn FCVTMS_sisd_64H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011110100000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_sisd_64H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111101100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_sisd_64H";
    #[inline]
    pub const fn FCVTZS_sisd_64H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011110110000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SCVTF_sisd_64H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111111000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_sisd_64H";
    #[inline]
    pub const fn SCVTF_sisd_64H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011111100000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNU_sisd_64H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111010110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNU_sisd_64H";
    #[inline]
    pub const fn FCVTNU_sisd_64H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011101011000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAU_sisd_64H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111110110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAU_sisd_64H";
    #[inline]
    pub const fn FCVTAU_sisd_64H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011111011000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPU_sisd_64H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111100110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPU_sisd_64H";
    #[inline]
    pub const fn FCVTPU_sisd_64H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011110011000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMU_sisd_64H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111101010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMU_sisd_64H";
    #[inline]
    pub const fn FCVTMU_sisd_64H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011110101000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_sisd_64H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111101110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_sisd_64H";
    #[inline]
    pub const fn FCVTZU_sisd_64H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011110111000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_sisd_64H {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110111111010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_sisd_64H";
    #[inline]
    pub const fn UCVTF_sisd_64H(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011111101000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNS_sisd_64S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001010100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNS_sisd_64S";
    #[inline]
    pub const fn FCVTNS_sisd_64S(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000101010000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAS_sisd_64S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001110100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAS_sisd_64S";
    #[inline]
    pub const fn FCVTAS_sisd_64S(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000111010000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPS_sisd_64S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001100100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPS_sisd_64S";
    #[inline]
    pub const fn FCVTPS_sisd_64S(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000110010000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMS_sisd_64S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001101000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMS_sisd_64S";
    #[inline]
    pub const fn FCVTMS_sisd_64S(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000110100000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_sisd_64S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001101100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_sisd_64S";
    #[inline]
    pub const fn FCVTZS_sisd_64S(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000110110000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SCVTF_sisd_64S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001111000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_sisd_64S";
    #[inline]
    pub const fn SCVTF_sisd_64S(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000111100000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNU_sisd_64S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001010110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNU_sisd_64S";
    #[inline]
    pub const fn FCVTNU_sisd_64S(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000101011000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAU_sisd_64S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001110110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAU_sisd_64S";
    #[inline]
    pub const fn FCVTAU_sisd_64S(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000111011000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPU_sisd_64S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001100110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPU_sisd_64S";
    #[inline]
    pub const fn FCVTPU_sisd_64S(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000110011000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMU_sisd_64S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001101010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMU_sisd_64S";
    #[inline]
    pub const fn FCVTMU_sisd_64S(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000110101000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_sisd_64S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001101110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_sisd_64S";
    #[inline]
    pub const fn FCVTZU_sisd_64S(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000110111000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_sisd_64S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110001111010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_sisd_64S";
    #[inline]
    pub const fn UCVTF_sisd_64S(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000111101000000u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
