/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod HINT_HM_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct HINT_HM_hints {
        pub CRm: ::aarchmrs_types::BitValue<4>,
        pub op2: ::aarchmrs_types::BitValue<3>,
    }
    impl HINT_HM_hints {
        #[inline]
        pub const fn new(
            CRm: ::aarchmrs_types::BitValue<4>,
            op2: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { CRm, op2 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010u32 << 12u32
                    | self.CRm.into_inner() << 8u32
                    | self.op2.into_inner() << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod NOP_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct NOP_HI_hints {}
    impl NOP_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010000000011111u32 << 0u32,
            )
        }
    }
}
pub mod YIELD_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct YIELD_HI_hints {}
    impl YIELD_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010000000111111u32 << 0u32,
            )
        }
    }
}
pub mod WFE_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct WFE_HI_hints {}
    impl WFE_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010000001011111u32 << 0u32,
            )
        }
    }
}
pub mod WFI_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct WFI_HI_hints {}
    impl WFI_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010000001111111u32 << 0u32,
            )
        }
    }
}
pub mod SEV_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SEV_HI_hints {}
    impl SEV_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010000010011111u32 << 0u32,
            )
        }
    }
}
pub mod SEVL_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SEVL_HI_hints {}
    impl SEVL_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010000010111111u32 << 0u32,
            )
        }
    }
}
pub mod DGH_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct DGH_HI_hints {}
    impl DGH_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010000011011111u32 << 0u32,
            )
        }
    }
}
pub mod XPACLRI_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct XPACLRI_HI_hints {}
    impl XPACLRI_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010000011111111u32 << 0u32,
            )
        }
    }
}
pub mod PACIA1716_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIA1716_HI_hints {}
    impl PACIA1716_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010000100011111u32 << 0u32,
            )
        }
    }
}
pub mod PACIB1716_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIB1716_HI_hints {}
    impl PACIB1716_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010000101011111u32 << 0u32,
            )
        }
    }
}
pub mod AUTIA1716_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIA1716_HI_hints {}
    impl AUTIA1716_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010000110011111u32 << 0u32,
            )
        }
    }
}
pub mod AUTIB1716_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIB1716_HI_hints {}
    impl AUTIB1716_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010000111011111u32 << 0u32,
            )
        }
    }
}
pub mod ESB_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ESB_HI_hints {}
    impl ESB_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010001000011111u32 << 0u32,
            )
        }
    }
}
pub mod PSB_HC_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PSB_HC_hints {}
    impl PSB_HC_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010001000111111u32 << 0u32,
            )
        }
    }
}
pub mod TSB_HC_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TSB_HC_hints {}
    impl TSB_HC_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010001001011111u32 << 0u32,
            )
        }
    }
}
pub mod GCSB_HD_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct GCSB_HD_hints {}
    impl GCSB_HD_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010001001111111u32 << 0u32,
            )
        }
    }
}
pub mod CSDB_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CSDB_HI_hints {}
    impl CSDB_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010001010011111u32 << 0u32,
            )
        }
    }
}
pub mod CLRBHB_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CLRBHB_HI_hints {}
    impl CLRBHB_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010001011011111u32 << 0u32,
            )
        }
    }
}
pub mod PACIAZ_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIAZ_HI_hints {}
    impl PACIAZ_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010001100011111u32 << 0u32,
            )
        }
    }
}
pub mod PACIASP_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIASP_HI_hints {}
    impl PACIASP_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010001100111111u32 << 0u32,
            )
        }
    }
}
pub mod PACIBZ_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIBZ_HI_hints {}
    impl PACIBZ_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010001101011111u32 << 0u32,
            )
        }
    }
}
pub mod PACIBSP_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIBSP_HI_hints {}
    impl PACIBSP_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010001101111111u32 << 0u32,
            )
        }
    }
}
pub mod AUTIAZ_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIAZ_HI_hints {}
    impl AUTIAZ_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010001110011111u32 << 0u32,
            )
        }
    }
}
pub mod AUTIASP_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIASP_HI_hints {}
    impl AUTIASP_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010001110111111u32 << 0u32,
            )
        }
    }
}
pub mod AUTIBZ_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIBZ_HI_hints {}
    impl AUTIBZ_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010001111011111u32 << 0u32,
            )
        }
    }
}
pub mod AUTIBSP_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIBSP_HI_hints {}
    impl AUTIBSP_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010001111111111u32 << 0u32,
            )
        }
    }
}
pub mod BTI_HB_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BTI_HB_hints {
        pub op2: ::aarchmrs_types::BitValue<3>,
    }
    impl BTI_HB_hints {
        #[inline]
        pub const fn new(op2: ::aarchmrs_types::BitValue<3>) -> Self {
            Self { op2 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110101010000001100100100u32 << 8u32
                    | self.op2.into_inner() << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod PACM_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACM_HI_hints {}
    impl PACM_HI_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010010011111111u32 << 0u32,
            )
        }
    }
}
pub mod CHKFEAT_HF_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CHKFEAT_HF_hints {}
    impl CHKFEAT_HF_hints {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110010010100011111u32 << 0u32,
            )
        }
    }
}
pub mod STSHH_HI_hints {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STSHH_HI_hints {
        pub op2: ::aarchmrs_types::BitValue<3>,
    }
    impl STSHH_HI_hints {
        #[inline]
        pub const fn new(op2: ::aarchmrs_types::BitValue<3>) -> Self {
            Self { op2 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110101010000001100100110u32 << 8u32
                    | self.op2.into_inner() << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
