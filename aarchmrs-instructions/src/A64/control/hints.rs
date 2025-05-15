/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod HINT_HM_hints {
    #[inline]
    pub fn HINT_HM_hints(
        CRm: impl Into<::aarchmrs_types::BitValue<4>>,
        op2: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11010101000000110010u32 << 12u32
                | u32::from(CRm.into()) << 8u32
                | u32::from(op2.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod NOP_HI_hints {
    #[inline]
    pub fn NOP_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010000000011111u32 << 0u32)
    }
}
pub mod YIELD_HI_hints {
    #[inline]
    pub fn YIELD_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010000000111111u32 << 0u32)
    }
}
pub mod WFE_HI_hints {
    #[inline]
    pub fn WFE_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010000001011111u32 << 0u32)
    }
}
pub mod WFI_HI_hints {
    #[inline]
    pub fn WFI_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010000001111111u32 << 0u32)
    }
}
pub mod SEV_HI_hints {
    #[inline]
    pub fn SEV_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010000010011111u32 << 0u32)
    }
}
pub mod SEVL_HI_hints {
    #[inline]
    pub fn SEVL_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010000010111111u32 << 0u32)
    }
}
pub mod DGH_HI_hints {
    #[inline]
    pub fn DGH_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010000011011111u32 << 0u32)
    }
}
pub mod XPACLRI_HI_hints {
    #[inline]
    pub fn XPACLRI_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010000011111111u32 << 0u32)
    }
}
pub mod PACIA1716_HI_hints {
    #[inline]
    pub fn PACIA1716_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010000100011111u32 << 0u32)
    }
}
pub mod PACIB1716_HI_hints {
    #[inline]
    pub fn PACIB1716_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010000101011111u32 << 0u32)
    }
}
pub mod AUTIA1716_HI_hints {
    #[inline]
    pub fn AUTIA1716_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010000110011111u32 << 0u32)
    }
}
pub mod AUTIB1716_HI_hints {
    #[inline]
    pub fn AUTIB1716_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010000111011111u32 << 0u32)
    }
}
pub mod ESB_HI_hints {
    #[inline]
    pub fn ESB_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010001000011111u32 << 0u32)
    }
}
pub mod PSB_HC_hints {
    #[inline]
    pub fn PSB_HC_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010001000111111u32 << 0u32)
    }
}
pub mod TSB_HC_hints {
    #[inline]
    pub fn TSB_HC_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010001001011111u32 << 0u32)
    }
}
pub mod GCSB_HD_hints {
    #[inline]
    pub fn GCSB_HD_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010001001111111u32 << 0u32)
    }
}
pub mod CSDB_HI_hints {
    #[inline]
    pub fn CSDB_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010001010011111u32 << 0u32)
    }
}
pub mod CLRBHB_HI_hints {
    #[inline]
    pub fn CLRBHB_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010001011011111u32 << 0u32)
    }
}
pub mod PACIAZ_HI_hints {
    #[inline]
    pub fn PACIAZ_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010001100011111u32 << 0u32)
    }
}
pub mod PACIASP_HI_hints {
    #[inline]
    pub fn PACIASP_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010001100111111u32 << 0u32)
    }
}
pub mod PACIBZ_HI_hints {
    #[inline]
    pub fn PACIBZ_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010001101011111u32 << 0u32)
    }
}
pub mod PACIBSP_HI_hints {
    #[inline]
    pub fn PACIBSP_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010001101111111u32 << 0u32)
    }
}
pub mod AUTIAZ_HI_hints {
    #[inline]
    pub fn AUTIAZ_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010001110011111u32 << 0u32)
    }
}
pub mod AUTIASP_HI_hints {
    #[inline]
    pub fn AUTIASP_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010001110111111u32 << 0u32)
    }
}
pub mod AUTIBZ_HI_hints {
    #[inline]
    pub fn AUTIBZ_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010001111011111u32 << 0u32)
    }
}
pub mod AUTIBSP_HI_hints {
    #[inline]
    pub fn AUTIBSP_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010001111111111u32 << 0u32)
    }
}
pub mod BTI_HB_hints {
    #[inline]
    pub fn BTI_HB_hints(
        op2: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110101010000001100100100u32 << 8u32
                | u32::from(op2.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
pub mod PACM_HI_hints {
    #[inline]
    pub fn PACM_HI_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010010011111111u32 << 0u32)
    }
}
pub mod CHKFEAT_HF_hints {
    #[inline]
    pub fn CHKFEAT_HF_hints() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11010101000000110010010100011111u32 << 0u32)
    }
}
pub mod STSHH_HI_hints {
    #[inline]
    pub fn STSHH_HI_hints(
        op2: impl Into<::aarchmrs_types::BitValue<3>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110101010000001100100110u32 << 8u32
                | u32::from(op2.into()) << 5u32
                | 0b11111u32 << 0u32,
        )
    }
}
