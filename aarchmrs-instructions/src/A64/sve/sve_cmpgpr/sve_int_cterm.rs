/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ctermeq_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ctermeq_rr_ {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl ctermeq_rr_ {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Rm: Rm.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001011u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b00000u32 << 0u32,
            )
        }
    }
}
pub mod ctermne_rr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ctermne_rr_ {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl ctermne_rr_ {
        #[inline]
        pub fn new(
            sz: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                sz: sz.into(),
                Rm: Rm.into(),
                Rn: Rn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001011u32 << 23u32
                    | u32::from(self.sz) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b10000u32 << 0u32,
            )
        }
    }
}
