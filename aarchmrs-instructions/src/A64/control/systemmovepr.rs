/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MSRR_SR_systemmovepr {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MSRR_SR_systemmovepr {
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub op1: ::aarchmrs_types::BitValue<3>,
        pub CRn: ::aarchmrs_types::BitValue<4>,
        pub CRm: ::aarchmrs_types::BitValue<4>,
        pub op2: ::aarchmrs_types::BitValue<3>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl MSRR_SR_systemmovepr {
        #[inline]
        pub const fn new(
            o0: ::aarchmrs_types::BitValue<1>,
            op1: ::aarchmrs_types::BitValue<3>,
            CRn: ::aarchmrs_types::BitValue<4>,
            CRm: ::aarchmrs_types::BitValue<4>,
            op2: ::aarchmrs_types::BitValue<3>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                o0,
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
                0b110101010101u32 << 20u32
                    | self.o0.into_inner() << 19u32
                    | self.op1.into_inner() << 16u32
                    | self.CRn.into_inner() << 12u32
                    | self.CRm.into_inner() << 8u32
                    | self.op2.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod MRRS_RS_systemmovepr {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MRRS_RS_systemmovepr {
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub op1: ::aarchmrs_types::BitValue<3>,
        pub CRn: ::aarchmrs_types::BitValue<4>,
        pub CRm: ::aarchmrs_types::BitValue<4>,
        pub op2: ::aarchmrs_types::BitValue<3>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl MRRS_RS_systemmovepr {
        #[inline]
        pub const fn new(
            o0: ::aarchmrs_types::BitValue<1>,
            op1: ::aarchmrs_types::BitValue<3>,
            CRn: ::aarchmrs_types::BitValue<4>,
            CRm: ::aarchmrs_types::BitValue<4>,
            op2: ::aarchmrs_types::BitValue<3>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                o0,
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
                0b110101010111u32 << 20u32
                    | self.o0.into_inner() << 19u32
                    | self.op1.into_inner() << 16u32
                    | self.CRn.into_inner() << 12u32
                    | self.CRm.into_inner() << 8u32
                    | self.op2.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
