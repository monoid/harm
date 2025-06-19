/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlallbb_z32_z8z8z8_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlallbb_z32_z8z8z8_ {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub TT: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl fmlallbb_z32_z8z8z8_ {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            TT: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                TT: TT.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100001u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.TT) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod fmlallbt_z32_z8z8z8_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlallbt_z32_z8z8z8_ {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub TT: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl fmlallbt_z32_z8z8z8_ {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            TT: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                TT: TT.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100001u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.TT) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod fmlalltb_z32_z8z8z8_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlalltb_z32_z8z8z8_ {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub TT: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl fmlalltb_z32_z8z8z8_ {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            TT: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                TT: TT.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100001u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.TT) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod fmlalltt_z32_z8z8z8_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlalltt_z32_z8z8z8_ {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub TT: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl fmlalltt_z32_z8z8z8_ {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            TT: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                TT: TT.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100001u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.TT) << 12u32
                    | 0b10u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
