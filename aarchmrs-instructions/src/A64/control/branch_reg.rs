/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod BR_64_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BR_64_branch_reg";
    #[inline]
    pub const fn BR_64_branch_reg(
        Z: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<2>,
        A: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | Z.into_inner() << 24u32
                | 0b0u32 << 23u32
                | op.into_inner() << 21u32
                | 0b111110000u32 << 12u32
                | A.into_inner() << 11u32
                | M.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod BRAAZ_64_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BRAAZ_64_branch_reg";
    #[inline]
    pub const fn BRAAZ_64_branch_reg(
        Z: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<2>,
        A: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | Z.into_inner() << 24u32
                | 0b0u32 << 23u32
                | op.into_inner() << 21u32
                | 0b111110000u32 << 12u32
                | A.into_inner() << 11u32
                | M.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod BRABZ_64_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BRABZ_64_branch_reg";
    #[inline]
    pub const fn BRABZ_64_branch_reg(
        Z: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<2>,
        A: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | Z.into_inner() << 24u32
                | 0b0u32 << 23u32
                | op.into_inner() << 21u32
                | 0b111110000u32 << 12u32
                | A.into_inner() << 11u32
                | M.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod BLR_64_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BLR_64_branch_reg";
    #[inline]
    pub const fn BLR_64_branch_reg(
        Z: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<2>,
        A: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | Z.into_inner() << 24u32
                | 0b0u32 << 23u32
                | op.into_inner() << 21u32
                | 0b111110000u32 << 12u32
                | A.into_inner() << 11u32
                | M.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod BLRAAZ_64_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BLRAAZ_64_branch_reg";
    #[inline]
    pub const fn BLRAAZ_64_branch_reg(
        Z: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<2>,
        A: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | Z.into_inner() << 24u32
                | 0b0u32 << 23u32
                | op.into_inner() << 21u32
                | 0b111110000u32 << 12u32
                | A.into_inner() << 11u32
                | M.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod BLRABZ_64_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BLRABZ_64_branch_reg";
    #[inline]
    pub const fn BLRABZ_64_branch_reg(
        Z: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<2>,
        A: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | Z.into_inner() << 24u32
                | 0b0u32 << 23u32
                | op.into_inner() << 21u32
                | 0b111110000u32 << 12u32
                | A.into_inner() << 11u32
                | M.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod RET_64R_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RET_64R_branch_reg";
    #[inline]
    pub const fn RET_64R_branch_reg(
        Z: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<2>,
        A: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | Z.into_inner() << 24u32
                | 0b0u32 << 23u32
                | op.into_inner() << 21u32
                | 0b111110000u32 << 12u32
                | A.into_inner() << 11u32
                | M.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod RETAASPPCR_64M_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111101111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110010111110000101111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RETAASPPCR_64M_branch_reg";
    #[inline]
    pub const fn RETAASPPCR_64M_branch_reg(
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110101100101111100001u32 << 11u32
                | M.into_inner() << 10u32
                | 0b11111u32 << 5u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod RETAA_64E_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100111111111001111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110000111110000001111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RETAA_64E_branch_reg";
    #[inline]
    pub const fn RETAA_64E_branch_reg(
        Z: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<2>,
        A: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | Z.into_inner() << 24u32
                | 0b0u32 << 23u32
                | op.into_inner() << 21u32
                | 0b111110000u32 << 12u32
                | A.into_inner() << 11u32
                | M.into_inner() << 10u32
                | 0b11111u32 << 5u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod RETABSPPCR_64M_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111101111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110010111110000101111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RETABSPPCR_64M_branch_reg";
    #[inline]
    pub const fn RETABSPPCR_64M_branch_reg(
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110101100101111100001u32 << 11u32
                | M.into_inner() << 10u32
                | 0b11111u32 << 5u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod RETAB_64E_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100111111111001111100000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110000111110000001111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RETAB_64E_branch_reg";
    #[inline]
    pub const fn RETAB_64E_branch_reg(
        Z: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<2>,
        A: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | Z.into_inner() << 24u32
                | 0b0u32 << 23u32
                | op.into_inner() << 21u32
                | 0b111110000u32 << 12u32
                | A.into_inner() << 11u32
                | M.into_inner() << 10u32
                | 0b11111u32 << 5u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod ERET_64E_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111001111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110100111110000001111100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ERET_64E_branch_reg";
    #[inline]
    pub const fn ERET_64E_branch_reg(
        A: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010110100111110000u32 << 12u32
                | A.into_inner() << 11u32
                | M.into_inner() << 10u32
                | 0b1111100000u32 << 0u32,
        )
    }
}
pub mod ERETAA_64E_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111001111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110100111110000001111111111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ERETAA_64E_branch_reg";
    #[inline]
    pub const fn ERETAA_64E_branch_reg(
        A: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010110100111110000u32 << 12u32
                | A.into_inner() << 11u32
                | M.into_inner() << 10u32
                | 0b1111111111u32 << 0u32,
        )
    }
}
pub mod ERETAB_64E_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111001111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110100111110000001111111111u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ERETAB_64E_branch_reg";
    #[inline]
    pub const fn ERETAB_64E_branch_reg(
        A: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010110100111110000u32 << 12u32
                | A.into_inner() << 11u32
                | M.into_inner() << 10u32
                | 0b1111111111u32 << 0u32,
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
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BRAA_64P_branch_reg";
    #[inline]
    pub const fn BRAA_64P_branch_reg(
        Z: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<2>,
        A: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | Z.into_inner() << 24u32
                | 0b0u32 << 23u32
                | op.into_inner() << 21u32
                | 0b111110000u32 << 12u32
                | A.into_inner() << 11u32
                | M.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod BRAB_64P_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BRAB_64P_branch_reg";
    #[inline]
    pub const fn BRAB_64P_branch_reg(
        Z: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<2>,
        A: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | Z.into_inner() << 24u32
                | 0b0u32 << 23u32
                | op.into_inner() << 21u32
                | 0b111110000u32 << 12u32
                | A.into_inner() << 11u32
                | M.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod BLRAA_64P_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BLRAA_64P_branch_reg";
    #[inline]
    pub const fn BLRAA_64P_branch_reg(
        Z: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<2>,
        A: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | Z.into_inner() << 24u32
                | 0b0u32 << 23u32
                | op.into_inner() << 21u32
                | 0b111110000u32 << 12u32
                | A.into_inner() << 11u32
                | M.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod BLRAB_64P_branch_reg {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BLRAB_64P_branch_reg";
    #[inline]
    pub const fn BLRAB_64P_branch_reg(
        Z: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<2>,
        A: ::aarchmrs_types::BitValue<1>,
        M: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rm: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | Z.into_inner() << 24u32
                | 0b0u32 << 23u32
                | op.into_inner() << 21u32
                | 0b111110000u32 << 12u32
                | A.into_inner() << 11u32
                | M.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rm.into_inner() << 0u32,
        )
    }
}
