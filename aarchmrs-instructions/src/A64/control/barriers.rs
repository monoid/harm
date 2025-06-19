/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CLREX_BN_barriers {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CLREX_BN_barriers {
        pub CRm: ::aarchmrs_types::BitValue<4>,
    }
    impl CLREX_BN_barriers {
        #[inline]
        pub fn new(CRm: impl Into<::aarchmrs_types::BitValue<4>>) -> Self {
            Self { CRm: CRm.into() }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110011u32 << 12u32
                    | u32::from(self.CRm) << 8u32
                    | 0b01011111u32 << 0u32,
            )
        }
    }
}
pub mod DSB_BO_barriers {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct DSB_BO_barriers {
        pub CRm: ::aarchmrs_types::BitValue<4>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl DSB_BO_barriers {
        #[inline]
        pub fn new(
            CRm: impl Into<::aarchmrs_types::BitValue<4>>,
            opc: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                CRm: CRm.into(),
                opc: opc.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110011u32 << 12u32
                    | u32::from(self.CRm) << 8u32
                    | 0b1u32 << 7u32
                    | u32::from(self.opc) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod DMB_BO_barriers {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct DMB_BO_barriers {
        pub CRm: ::aarchmrs_types::BitValue<4>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl DMB_BO_barriers {
        #[inline]
        pub fn new(
            CRm: impl Into<::aarchmrs_types::BitValue<4>>,
            opc: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                CRm: CRm.into(),
                opc: opc.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110011u32 << 12u32
                    | u32::from(self.CRm) << 8u32
                    | 0b1u32 << 7u32
                    | u32::from(self.opc) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod ISB_BI_barriers {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ISB_BI_barriers {
        pub CRm: ::aarchmrs_types::BitValue<4>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl ISB_BI_barriers {
        #[inline]
        pub fn new(
            CRm: impl Into<::aarchmrs_types::BitValue<4>>,
            opc: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                CRm: CRm.into(),
                opc: opc.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110011u32 << 12u32
                    | u32::from(self.CRm) << 8u32
                    | 0b1u32 << 7u32
                    | u32::from(self.opc) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod SB_only_barriers {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SB_only_barriers {
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl SB_only_barriers {
        #[inline]
        pub fn new(opc: impl Into<::aarchmrs_types::BitValue<2>>) -> Self {
            Self { opc: opc.into() }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101010100000011001100001u32 << 7u32
                    | u32::from(self.opc) << 5u32
                    | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod DSB_BOn_barriers {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct DSB_BOn_barriers {
        pub imm2: ::aarchmrs_types::BitValue<2>,
    }
    impl DSB_BOn_barriers {
        #[inline]
        pub fn new(imm2: impl Into<::aarchmrs_types::BitValue<2>>) -> Self {
            Self { imm2: imm2.into() }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110011u32 << 12u32
                    | u32::from(self.imm2) << 10u32
                    | 0b1000111111u32 << 0u32,
            )
        }
    }
}
pub mod TCOMMIT_only_barriers {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TCOMMIT_only_barriers {}
    impl TCOMMIT_only_barriers {
        #[inline]
        pub fn new() -> Self {
            Self {}
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110011000001111111u32 << 0u32,
            )
        }
    }
}
