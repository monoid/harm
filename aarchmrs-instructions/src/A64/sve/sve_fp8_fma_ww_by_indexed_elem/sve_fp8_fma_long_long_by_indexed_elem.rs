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
        pub const fn new(
            TT: ::aarchmrs_types::BitValue<2>,
            i4h: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<3>,
            i4l: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                TT,
                i4h,
                Zm,
                i4l,
                Zn,
                Zda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100u32 << 24u32
                    | self.TT.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.i4h.into_inner() << 19u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1100u32 << 12u32
                    | self.i4l.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 0u32,
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
        pub const fn new(
            TT: ::aarchmrs_types::BitValue<2>,
            i4h: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<3>,
            i4l: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                TT,
                i4h,
                Zm,
                i4l,
                Zn,
                Zda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100u32 << 24u32
                    | self.TT.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.i4h.into_inner() << 19u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1100u32 << 12u32
                    | self.i4l.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 0u32,
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
        pub const fn new(
            TT: ::aarchmrs_types::BitValue<2>,
            i4h: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<3>,
            i4l: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                TT,
                i4h,
                Zm,
                i4l,
                Zn,
                Zda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100u32 << 24u32
                    | self.TT.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.i4h.into_inner() << 19u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1100u32 << 12u32
                    | self.i4l.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 0u32,
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
        pub const fn new(
            TT: ::aarchmrs_types::BitValue<2>,
            i4h: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<3>,
            i4l: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                TT,
                i4h,
                Zm,
                i4l,
                Zn,
                Zda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100u32 << 24u32
                    | self.TT.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.i4h.into_inner() << 19u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1100u32 << 12u32
                    | self.i4l.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 0u32,
            )
        }
    }
}
