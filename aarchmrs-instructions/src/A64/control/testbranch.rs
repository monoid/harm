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
        pub fn new(
            b5: impl Into<::aarchmrs_types::BitValue<1>>,
            b40: impl Into<::aarchmrs_types::BitValue<5>>,
            imm14: impl Into<::aarchmrs_types::BitValue<14>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                b5: b5.into(),
                b40: b40.into(),
                imm14: imm14.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                u32::from(self.b5) << 31u32
                    | 0b0110110u32 << 24u32
                    | u32::from(self.b40) << 19u32
                    | u32::from(self.imm14) << 5u32
                    | u32::from(self.Rt) << 0u32,
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
        pub fn new(
            b5: impl Into<::aarchmrs_types::BitValue<1>>,
            b40: impl Into<::aarchmrs_types::BitValue<5>>,
            imm14: impl Into<::aarchmrs_types::BitValue<14>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                b5: b5.into(),
                b40: b40.into(),
                imm14: imm14.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                u32::from(self.b5) << 31u32
                    | 0b0110111u32 << 24u32
                    | u32::from(self.b40) << 19u32
                    | u32::from(self.imm14) << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
