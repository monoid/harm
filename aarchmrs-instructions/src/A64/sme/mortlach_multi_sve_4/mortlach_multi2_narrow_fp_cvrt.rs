/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod bfcvt_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfcvt_z_mz2_ {
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl bfcvt_z_mz2_ {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<4>,
            N: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zn, N, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000101100000111000u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | self.N.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod fcvt_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcvt_z_mz2_ {
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl fcvt_z_mz2_ {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<4>,
            N: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zn, N, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000100100000111000u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | self.N.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod bfcvtn_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfcvtn_z_mz2_ {
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl bfcvtn_z_mz2_ {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<4>,
            N: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zn, N, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000101100000111000u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | self.N.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod fcvtn_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fcvtn_z_mz2_ {
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl fcvtn_z_mz2_ {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<4>,
            N: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zn, N, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000100100000111000u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | self.N.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
