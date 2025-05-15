/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod BR_64_branch_reg {
    #[inline]
    pub fn BR_64_branch_reg(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        A: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | u32::from(Z.into()) << 24u32
                | 0b0u32 << 23u32
                | u32::from(op.into()) << 21u32
                | 0b111110000u32 << 12u32
                | u32::from(A.into()) << 11u32
                | u32::from(M.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rm.into()) << 0u32,
        )
    }
}
pub mod BRAAZ_64_branch_reg {
    #[inline]
    pub fn BRAAZ_64_branch_reg(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        A: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | u32::from(Z.into()) << 24u32
                | 0b0u32 << 23u32
                | u32::from(op.into()) << 21u32
                | 0b111110000u32 << 12u32
                | u32::from(A.into()) << 11u32
                | u32::from(M.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rm.into()) << 0u32,
        )
    }
}
pub mod BRABZ_64_branch_reg {
    #[inline]
    pub fn BRABZ_64_branch_reg(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        A: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | u32::from(Z.into()) << 24u32
                | 0b0u32 << 23u32
                | u32::from(op.into()) << 21u32
                | 0b111110000u32 << 12u32
                | u32::from(A.into()) << 11u32
                | u32::from(M.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rm.into()) << 0u32,
        )
    }
}
pub mod BLR_64_branch_reg {
    #[inline]
    pub fn BLR_64_branch_reg(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        A: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | u32::from(Z.into()) << 24u32
                | 0b0u32 << 23u32
                | u32::from(op.into()) << 21u32
                | 0b111110000u32 << 12u32
                | u32::from(A.into()) << 11u32
                | u32::from(M.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rm.into()) << 0u32,
        )
    }
}
pub mod BLRAAZ_64_branch_reg {
    #[inline]
    pub fn BLRAAZ_64_branch_reg(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        A: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | u32::from(Z.into()) << 24u32
                | 0b0u32 << 23u32
                | u32::from(op.into()) << 21u32
                | 0b111110000u32 << 12u32
                | u32::from(A.into()) << 11u32
                | u32::from(M.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rm.into()) << 0u32,
        )
    }
}
pub mod BLRABZ_64_branch_reg {
    #[inline]
    pub fn BLRABZ_64_branch_reg(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        A: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | u32::from(Z.into()) << 24u32
                | 0b0u32 << 23u32
                | u32::from(op.into()) << 21u32
                | 0b111110000u32 << 12u32
                | u32::from(A.into()) << 11u32
                | u32::from(M.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rm.into()) << 0u32,
        )
    }
}
pub mod RET_64R_branch_reg {
    #[inline]
    pub fn RET_64R_branch_reg(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        A: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | u32::from(Z.into()) << 24u32
                | 0b0u32 << 23u32
                | u32::from(op.into()) << 21u32
                | 0b111110000u32 << 12u32
                | u32::from(A.into()) << 11u32
                | u32::from(M.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rm.into()) << 0u32,
        )
    }
}
pub mod RETAASPPCR_64M_branch_reg {
    #[inline]
    pub fn RETAASPPCR_64M_branch_reg(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110101100101111100001u32 << 11u32
                | u32::from(M.into()) << 10u32
                | 0b11111u32 << 5u32
                | u32::from(Rm.into()) << 0u32,
        )
    }
}
pub mod RETAA_64E_branch_reg {
    #[inline]
    pub fn RETAA_64E_branch_reg(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        A: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | u32::from(Z.into()) << 24u32
                | 0b0u32 << 23u32
                | u32::from(op.into()) << 21u32
                | 0b111110000u32 << 12u32
                | u32::from(A.into()) << 11u32
                | u32::from(M.into()) << 10u32
                | 0b11111u32 << 5u32
                | u32::from(Rm.into()) << 0u32,
        )
    }
}
pub mod RETABSPPCR_64M_branch_reg {
    #[inline]
    pub fn RETABSPPCR_64M_branch_reg(
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110101100101111100001u32 << 11u32
                | u32::from(M.into()) << 10u32
                | 0b11111u32 << 5u32
                | u32::from(Rm.into()) << 0u32,
        )
    }
}
pub mod RETAB_64E_branch_reg {
    #[inline]
    pub fn RETAB_64E_branch_reg(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        A: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | u32::from(Z.into()) << 24u32
                | 0b0u32 << 23u32
                | u32::from(op.into()) << 21u32
                | 0b111110000u32 << 12u32
                | u32::from(A.into()) << 11u32
                | u32::from(M.into()) << 10u32
                | 0b11111u32 << 5u32
                | u32::from(Rm.into()) << 0u32,
        )
    }
}
pub mod ERET_64E_branch_reg {
    #[inline]
    pub fn ERET_64E_branch_reg(
        A: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010110100111110000u32 << 12u32
                | u32::from(A.into()) << 11u32
                | u32::from(M.into()) << 10u32
                | 0b1111100000u32 << 0u32,
        )
    }
}
pub mod ERETAA_64E_branch_reg {
    #[inline]
    pub fn ERETAA_64E_branch_reg(
        A: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010110100111110000u32 << 12u32
                | u32::from(A.into()) << 11u32
                | u32::from(M.into()) << 10u32
                | 0b1111111111u32 << 0u32,
        )
    }
}
pub mod ERETAB_64E_branch_reg {
    #[inline]
    pub fn ERETAB_64E_branch_reg(
        A: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010110100111110000u32 << 12u32
                | u32::from(A.into()) << 11u32
                | u32::from(M.into()) << 10u32
                | 0b1111111111u32 << 0u32,
        )
    }
}
pub mod DRPS_64E_branch_reg {
    #[inline]
    pub fn DRPS_64E_branch_reg() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010110101111110000001111100000u32 << 0u32)
    }
}
pub mod BRAA_64P_branch_reg {
    #[inline]
    pub fn BRAA_64P_branch_reg(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        A: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | u32::from(Z.into()) << 24u32
                | 0b0u32 << 23u32
                | u32::from(op.into()) << 21u32
                | 0b111110000u32 << 12u32
                | u32::from(A.into()) << 11u32
                | u32::from(M.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rm.into()) << 0u32,
        )
    }
}
pub mod BRAB_64P_branch_reg {
    #[inline]
    pub fn BRAB_64P_branch_reg(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        A: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | u32::from(Z.into()) << 24u32
                | 0b0u32 << 23u32
                | u32::from(op.into()) << 21u32
                | 0b111110000u32 << 12u32
                | u32::from(A.into()) << 11u32
                | u32::from(M.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rm.into()) << 0u32,
        )
    }
}
pub mod BLRAA_64P_branch_reg {
    #[inline]
    pub fn BLRAA_64P_branch_reg(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        A: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | u32::from(Z.into()) << 24u32
                | 0b0u32 << 23u32
                | u32::from(op.into()) << 21u32
                | 0b111110000u32 << 12u32
                | u32::from(A.into()) << 11u32
                | u32::from(M.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rm.into()) << 0u32,
        )
    }
}
pub mod BLRAB_64P_branch_reg {
    #[inline]
    pub fn BLRAB_64P_branch_reg(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        A: impl Into<::aarchmrs_types::BitValue<1>>,
        M: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101011u32 << 25u32
                | u32::from(Z.into()) << 24u32
                | 0b0u32 << 23u32
                | u32::from(op.into()) << 21u32
                | 0b111110000u32 << 12u32
                | u32::from(A.into()) << 11u32
                | u32::from(M.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rm.into()) << 0u32,
        )
    }
}
