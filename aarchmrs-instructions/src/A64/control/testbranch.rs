/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod TBZ_only_testbranch {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TBZ_only_testbranch {
        pub b5: ::aarchmrs_types::BitValue<1>,
        pub b40: ::aarchmrs_types::BitValue<5>,
        pub imm14: ::aarchmrs_types::BitValue<14>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl TBZ_only_testbranch {
        #[inline]
        pub const fn new(
            b5: ::aarchmrs_types::BitValue<1>,
            b40: ::aarchmrs_types::BitValue<5>,
            imm14: ::aarchmrs_types::BitValue<14>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { b5, b40, imm14, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.b5.into_inner() << 31u32
                    | 0b0110110u32 << 24u32
                    | self.b40.into_inner() << 19u32
                    | self.imm14.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod TBNZ_only_testbranch {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TBNZ_only_testbranch {
        pub b5: ::aarchmrs_types::BitValue<1>,
        pub b40: ::aarchmrs_types::BitValue<5>,
        pub imm14: ::aarchmrs_types::BitValue<14>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl TBNZ_only_testbranch {
        #[inline]
        pub const fn new(
            b5: ::aarchmrs_types::BitValue<1>,
            b40: ::aarchmrs_types::BitValue<5>,
            imm14: ::aarchmrs_types::BitValue<14>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { b5, b40, imm14, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.b5.into_inner() << 31u32
                    | 0b0110111u32 << 24u32
                    | self.b40.into_inner() << 19u32
                    | self.imm14.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
