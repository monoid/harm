/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqcvt_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqcvt_z_mz2_ {
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqcvt_z_mz2_ {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zn, U, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000100100011111000u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | self.U.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod sqcvtu_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqcvtu_z_mz2_ {
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqcvtu_z_mz2_ {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<4>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000101100011111000u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0u32 << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod uqcvt_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqcvt_z_mz2_ {
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uqcvt_z_mz2_ {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zn, U, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000100100011111000u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | self.U.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
