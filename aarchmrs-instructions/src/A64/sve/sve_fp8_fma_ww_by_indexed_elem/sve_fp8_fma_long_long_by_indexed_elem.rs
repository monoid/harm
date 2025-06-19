/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlallbb_z32_z8z8z8i_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlallbb_z32_z8z8z8i_ {
        pub TT: ::aarchmrs_types::BitValue<2>,
        pub i4h: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub i4l: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl fmlallbb_z32_z8z8z8i_ {
        #[inline]
        pub fn new(
            TT: impl Into<::aarchmrs_types::BitValue<2>>,
            i4h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            i4l: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                TT: TT.into(),
                i4h: i4h.into(),
                Zm: Zm.into(),
                i4l: i4l.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100u32 << 24u32
                    | u32::from(self.TT) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.i4h) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b1100u32 << 12u32
                    | u32::from(self.i4l) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod fmlallbt_z32_z8z8z8i_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlallbt_z32_z8z8z8i_ {
        pub TT: ::aarchmrs_types::BitValue<2>,
        pub i4h: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub i4l: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl fmlallbt_z32_z8z8z8i_ {
        #[inline]
        pub fn new(
            TT: impl Into<::aarchmrs_types::BitValue<2>>,
            i4h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            i4l: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                TT: TT.into(),
                i4h: i4h.into(),
                Zm: Zm.into(),
                i4l: i4l.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100u32 << 24u32
                    | u32::from(self.TT) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.i4h) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b1100u32 << 12u32
                    | u32::from(self.i4l) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod fmlalltb_z32_z8z8z8i_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlalltb_z32_z8z8z8i_ {
        pub TT: ::aarchmrs_types::BitValue<2>,
        pub i4h: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub i4l: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl fmlalltb_z32_z8z8z8i_ {
        #[inline]
        pub fn new(
            TT: impl Into<::aarchmrs_types::BitValue<2>>,
            i4h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            i4l: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                TT: TT.into(),
                i4h: i4h.into(),
                Zm: Zm.into(),
                i4l: i4l.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100u32 << 24u32
                    | u32::from(self.TT) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.i4h) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b1100u32 << 12u32
                    | u32::from(self.i4l) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
pub mod fmlalltt_z32_z8z8z8i_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlalltt_z32_z8z8z8i_ {
        pub TT: ::aarchmrs_types::BitValue<2>,
        pub i4h: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub i4l: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl fmlalltt_z32_z8z8z8i_ {
        #[inline]
        pub fn new(
            TT: impl Into<::aarchmrs_types::BitValue<2>>,
            i4h: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<3>>,
            i4l: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zda: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                TT: TT.into(),
                i4h: i4h.into(),
                Zm: Zm.into(),
                i4l: i4l.into(),
                Zn: Zn.into(),
                Zda: Zda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100u32 << 24u32
                    | u32::from(self.TT) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.i4h) << 19u32
                    | u32::from(self.Zm) << 16u32
                    | 0b1100u32 << 12u32
                    | u32::from(self.i4l) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zda) << 0u32,
            )
        }
    }
}
