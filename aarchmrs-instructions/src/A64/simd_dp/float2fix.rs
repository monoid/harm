/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SCVTF_S32_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110000000100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_S32_float2fix";
    #[inline]
    pub const fn SCVTF_S32_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000000010u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_S32_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110000000110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_S32_float2fix";
    #[inline]
    pub const fn UCVTF_S32_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000000011u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_32S_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110000110000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_32S_float2fix";
    #[inline]
    pub const fn FCVTZS_32S_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000011000u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_32S_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110000110010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_32S_float2fix";
    #[inline]
    pub const fn FCVTZU_32S_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000011001u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SCVTF_D32_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110010000100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_D32_float2fix";
    #[inline]
    pub const fn SCVTF_D32_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001000010u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_D32_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110010000110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_D32_float2fix";
    #[inline]
    pub const fn UCVTF_D32_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001000011u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_32D_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110010110000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_32D_float2fix";
    #[inline]
    pub const fn FCVTZS_32D_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001011000u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_32D_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110010110010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_32D_float2fix";
    #[inline]
    pub const fn FCVTZU_32D_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001011001u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SCVTF_H32_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110110000100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_H32_float2fix";
    #[inline]
    pub const fn SCVTF_H32_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011000010u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_H32_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110110000110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_H32_float2fix";
    #[inline]
    pub const fn UCVTF_H32_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011000011u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_32H_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110110110000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_32H_float2fix";
    #[inline]
    pub const fn FCVTZS_32H_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011011000u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_32H_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00011110110110010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_32H_float2fix";
    #[inline]
    pub const fn FCVTZU_32H_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011011001u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SCVTF_S64_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110000000100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_S64_float2fix";
    #[inline]
    pub const fn SCVTF_S64_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000000010u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_S64_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110000000110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_S64_float2fix";
    #[inline]
    pub const fn UCVTF_S64_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000000011u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_64S_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110000110000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_64S_float2fix";
    #[inline]
    pub const fn FCVTZS_64S_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000011000u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_64S_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110000110010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_64S_float2fix";
    #[inline]
    pub const fn FCVTZU_64S_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111000011001u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SCVTF_D64_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110010000100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_D64_float2fix";
    #[inline]
    pub const fn SCVTF_D64_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111001000010u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_D64_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110010000110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_D64_float2fix";
    #[inline]
    pub const fn UCVTF_D64_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111001000011u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_64D_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110010110000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_64D_float2fix";
    #[inline]
    pub const fn FCVTZS_64D_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111001011000u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_64D_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110010110010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_64D_float2fix";
    #[inline]
    pub const fn FCVTZU_64D_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111001011001u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SCVTF_H64_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110110000100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_H64_float2fix";
    #[inline]
    pub const fn SCVTF_H64_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011000010u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_H64_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110110000110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_H64_float2fix";
    #[inline]
    pub const fn UCVTF_H64_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011000011u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_64H_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110110110000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_64H_float2fix";
    #[inline]
    pub const fn FCVTZS_64H_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011011000u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_64H_float2fix {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10011110110110010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_64H_float2fix";
    #[inline]
    pub const fn FCVTZU_64H_float2fix(
        scale: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001111011011001u32 << 16u32
                | scale.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
