/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CLREX_BN_barriers {
    #[inline]
    pub fn CLREX_BN_barriers(
        CRm: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010101000000110011u32 << 12u32
                | u32::from(CRm.into()) << 8u32
                | 0b01011111u32 << 0u32,
        )
    }
}
pub mod DSB_BO_barriers {
    #[inline]
    pub fn DSB_BO_barriers(
        CRm: impl Into<::aarchmrs_types::BitValue<4>>,
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010101000000110011u32 << 12u32
                | u32::from(CRm.into()) << 8u32
                | 0b1u32 << 7u32
                | u32::from(opc.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod DMB_BO_barriers {
    #[inline]
    pub fn DMB_BO_barriers(
        CRm: impl Into<::aarchmrs_types::BitValue<4>>,
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010101000000110011u32 << 12u32
                | u32::from(CRm.into()) << 8u32
                | 0b1u32 << 7u32
                | u32::from(opc.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod ISB_BI_barriers {
    #[inline]
    pub fn ISB_BI_barriers(
        CRm: impl Into<::aarchmrs_types::BitValue<4>>,
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010101000000110011u32 << 12u32
                | u32::from(CRm.into()) << 8u32
                | 0b1u32 << 7u32
                | u32::from(opc.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod SB_only_barriers {
    #[inline]
    pub fn SB_only_barriers(
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101010100000011001100001u32 << 7u32
                | u32::from(opc.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod DSB_BOn_barriers {
    #[inline]
    pub fn DSB_BOn_barriers(
        imm2: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010101000000110011u32 << 12u32
                | u32::from(imm2.into()) << 10u32
                | 0b1000111111u32 << 0u32,
        )
    }
}
pub mod TCOMMIT_only_barriers {
    #[inline]
    pub fn TCOMMIT_only_barriers() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110011000001111111u32 << 0u32)
    }
}
