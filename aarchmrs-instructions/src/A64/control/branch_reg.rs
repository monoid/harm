/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod BR_64_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BR_64_branch_reg";
    #[inline]
    pub const fn BR_64_branch_reg(
        Rn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011000011111000000u32 << 10u32 | Rn.into_inner() << 5u32 | 0b00000u32 << 0u32,
        )
    }
}
pub mod BRAAZ_64_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110000111110000100000011111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BRAAZ_64_branch_reg";
    #[inline]
    pub const fn BRAAZ_64_branch_reg(
        Rn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011000011111000010u32 << 10u32 | Rn.into_inner() << 5u32 | 0b11111u32 << 0u32,
        )
    }
}
pub mod BRABZ_64_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110000111110000110000011111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BRABZ_64_branch_reg";
    #[inline]
    pub const fn BRABZ_64_branch_reg(
        Rn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011000011111000011u32 << 10u32 | Rn.into_inner() << 5u32 | 0b11111u32 << 0u32,
        )
    }
}
pub mod BLR_64_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110001111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BLR_64_branch_reg";
    #[inline]
    pub const fn BLR_64_branch_reg(
        Rn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011000111111000000u32 << 10u32 | Rn.into_inner() << 5u32 | 0b00000u32 << 0u32,
        )
    }
}
pub mod BLRAAZ_64_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110001111110000100000011111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BLRAAZ_64_branch_reg";
    #[inline]
    pub const fn BLRAAZ_64_branch_reg(
        Rn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011000111111000010u32 << 10u32 | Rn.into_inner() << 5u32 | 0b11111u32 << 0u32,
        )
    }
}
pub mod BLRABZ_64_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110001111110000110000011111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BLRABZ_64_branch_reg";
    #[inline]
    pub const fn BLRABZ_64_branch_reg(
        Rn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011000111111000011u32 << 10u32 | Rn.into_inner() << 5u32 | 0b11111u32 << 0u32,
        )
    }
}
pub mod RET_64R_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110010111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RET_64R_branch_reg";
    #[inline]
    pub const fn RET_64R_branch_reg(
        Rn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011001011111000000u32 << 10u32 | Rn.into_inner() << 5u32 | 0b00000u32 << 0u32,
        )
    }
}
pub mod RETAASPPCR_64M_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110010111110000101111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RETAASPPCR_64M_branch_reg";
    #[inline]
    pub const fn RETAASPPCR_64M_branch_reg(
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110101100101111100001011111u32 << 5u32 | Rm.into_inner() << 0u32,
        )
    }
}
pub mod RETAA_64E_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110010111110000101111111111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RETAA_64E_branch_reg";
    #[inline]
    pub const fn RETAA_64E_branch_reg() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010110010111110000101111111111u32 << 0u32)
    }
}
pub mod RETABSPPCR_64M_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110010111110000111111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RETABSPPCR_64M_branch_reg";
    #[inline]
    pub const fn RETABSPPCR_64M_branch_reg(
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110101100101111100001111111u32 << 5u32 | Rm.into_inner() << 0u32,
        )
    }
}
pub mod RETAB_64E_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110010111110000111111111111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RETAB_64E_branch_reg";
    #[inline]
    pub const fn RETAB_64E_branch_reg() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010110010111110000111111111111u32 << 0u32)
    }
}
pub mod ERET_64E_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110100111110000001111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ERET_64E_branch_reg";
    #[inline]
    pub const fn ERET_64E_branch_reg() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010110100111110000001111100000u32 << 0u32)
    }
}
pub mod ERETAA_64E_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110100111110000101111111111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ERETAA_64E_branch_reg";
    #[inline]
    pub const fn ERETAA_64E_branch_reg() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010110100111110000101111111111u32 << 0u32)
    }
}
pub mod ERETAB_64E_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110100111110000111111111111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ERETAB_64E_branch_reg";
    #[inline]
    pub const fn ERETAB_64E_branch_reg() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010110100111110000111111111111u32 << 0u32)
    }
}
pub mod TEXIT_te_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111101111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110111111110000001111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TEXIT_te_branch_reg";
    #[inline]
    pub const fn TEXIT_te_branch_reg(
        op1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110101101111111100000u32 << 11u32
                | op1.into_inner() << 10u32
                | 0b1111100000u32 << 0u32,
        )
    }
}
pub mod DRPS_64E_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110101111110000001111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "DRPS_64E_branch_reg";
    #[inline]
    pub const fn DRPS_64E_branch_reg() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010110101111110000001111100000u32 << 0u32)
    }
}
pub mod BRAA_64P_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010111000111110000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BRAA_64P_branch_reg";
    #[inline]
    pub const fn BRAA_64P_branch_reg(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011100011111000010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod BRAB_64P_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010111000111110000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BRAB_64P_branch_reg";
    #[inline]
    pub const fn BRAB_64P_branch_reg(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011100011111000011u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod BLRAA_64P_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010111001111110000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BLRAA_64P_branch_reg";
    #[inline]
    pub const fn BLRAA_64P_branch_reg(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011100111111000010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod BLRAB_64P_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010111001111110000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BLRAB_64P_branch_reg";
    #[inline]
    pub const fn BLRAB_64P_branch_reg(
        Rn: ::aarchmrs_types::BitValue<5>,
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011100111111000011u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rm.into_inner() << 0u32,
        )
    }
}
