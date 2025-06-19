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
        pub const fn new(
            op1: ::aarchmrs_types::BitValue<3>,
            CRn: ::aarchmrs_types::BitValue<4>,
            CRm: ::aarchmrs_types::BitValue<4>,
            op2: ::aarchmrs_types::BitValue<3>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                op1,
                CRn,
                CRm,
                op2,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101010100001u32 << 19u32
                    | self.op1.into_inner() << 16u32
                    | self.CRn.into_inner() << 12u32
                    | self.CRm.into_inner() << 8u32
                    | self.op2.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
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
        pub const fn new(
            op1: ::aarchmrs_types::BitValue<3>,
            CRn: ::aarchmrs_types::BitValue<4>,
            CRm: ::aarchmrs_types::BitValue<4>,
            op2: ::aarchmrs_types::BitValue<3>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                op1,
                CRn,
                CRm,
                op2,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101010100101u32 << 19u32
                    | self.op1.into_inner() << 16u32
                    | self.CRn.into_inner() << 12u32
                    | self.CRm.into_inner() << 8u32
                    | self.op2.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
