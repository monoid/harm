/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MSR_SR_systemmove {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MSR_SR_systemmove {
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub op1: ::aarchmrs_types::BitValue<3>,
        pub CRn: ::aarchmrs_types::BitValue<4>,
        pub CRm: ::aarchmrs_types::BitValue<4>,
        pub op2: ::aarchmrs_types::BitValue<3>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl MSR_SR_systemmove {
        #[inline]
        pub fn new(
            o0: impl Into<::aarchmrs_types::BitValue<1>>,
            op1: impl Into<::aarchmrs_types::BitValue<3>>,
            CRn: impl Into<::aarchmrs_types::BitValue<4>>,
            CRm: impl Into<::aarchmrs_types::BitValue<4>>,
            op2: impl Into<::aarchmrs_types::BitValue<3>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                o0: o0.into(),
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
                0b110101010001u32 << 20u32
                    | u32::from(self.o0) << 19u32
                    | u32::from(self.op1) << 16u32
                    | u32::from(self.CRn) << 12u32
                    | u32::from(self.CRm) << 8u32
                    | u32::from(self.op2) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
pub mod MRS_RS_systemmove {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MRS_RS_systemmove {
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub op1: ::aarchmrs_types::BitValue<3>,
        pub CRn: ::aarchmrs_types::BitValue<4>,
        pub CRm: ::aarchmrs_types::BitValue<4>,
        pub op2: ::aarchmrs_types::BitValue<3>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl MRS_RS_systemmove {
        #[inline]
        pub fn new(
            o0: impl Into<::aarchmrs_types::BitValue<1>>,
            op1: impl Into<::aarchmrs_types::BitValue<3>>,
            CRn: impl Into<::aarchmrs_types::BitValue<4>>,
            CRm: impl Into<::aarchmrs_types::BitValue<4>>,
            op2: impl Into<::aarchmrs_types::BitValue<3>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                o0: o0.into(),
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
                0b110101010011u32 << 20u32
                    | u32::from(self.o0) << 19u32
                    | u32::from(self.op1) << 16u32
                    | u32::from(self.CRn) << 12u32
                    | u32::from(self.CRm) << 8u32
                    | u32::from(self.op2) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
