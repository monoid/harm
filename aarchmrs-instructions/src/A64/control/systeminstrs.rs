/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SYS_CR_systeminstrs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SYS_CR_systeminstrs {
        pub op1: ::aarchmrs_types::BitValue<3>,
        pub CRn: ::aarchmrs_types::BitValue<4>,
        pub CRm: ::aarchmrs_types::BitValue<4>,
        pub op2: ::aarchmrs_types::BitValue<3>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SYS_CR_systeminstrs {
        #[inline]
        pub fn new(
            op1: impl Into<::aarchmrs_types::BitValue<3>>,
            CRn: impl Into<::aarchmrs_types::BitValue<4>>,
            CRm: impl Into<::aarchmrs_types::BitValue<4>>,
            op2: impl Into<::aarchmrs_types::BitValue<3>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                op1: op1.into(),
                CRn: CRn.into(),
                CRm: CRm.into(),
                op2: op2.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101010100001u32 << 19u32
                    | u32::from(self.op1) << 16u32
                    | u32::from(self.CRn) << 12u32
                    | u32::from(self.CRm) << 8u32
                    | u32::from(self.op2) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod SYSL_RC_systeminstrs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SYSL_RC_systeminstrs {
        pub op1: ::aarchmrs_types::BitValue<3>,
        pub CRn: ::aarchmrs_types::BitValue<4>,
        pub CRm: ::aarchmrs_types::BitValue<4>,
        pub op2: ::aarchmrs_types::BitValue<3>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SYSL_RC_systeminstrs {
        #[inline]
        pub fn new(
            op1: impl Into<::aarchmrs_types::BitValue<3>>,
            CRn: impl Into<::aarchmrs_types::BitValue<4>>,
            CRm: impl Into<::aarchmrs_types::BitValue<4>>,
            op2: impl Into<::aarchmrs_types::BitValue<3>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                op1: op1.into(),
                CRn: CRn.into(),
                CRm: CRm.into(),
                op2: op2.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101010100101u32 << 19u32
                    | u32::from(self.op1) << 16u32
                    | u32::from(self.CRn) << 12u32
                    | u32::from(self.CRm) << 8u32
                    | u32::from(self.op2) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
