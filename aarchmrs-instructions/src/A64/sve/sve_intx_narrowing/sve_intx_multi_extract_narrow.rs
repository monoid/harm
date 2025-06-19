/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqcvtn_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqcvtn_z_mz2_ {
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqcvtn_z_mz2_ {
        #[inline]
        pub const fn new(
            U: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<4>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { U, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101001100010100u32 << 12u32
                    | self.U.into_inner() << 11u32
                    | 0b0u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0u32 << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod sqcvtun_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqcvtun_z_mz2_ {
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqcvtun_z_mz2_ {
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
                0b0100010100110001010100u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0u32 << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod uqcvtn_z_mz2_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqcvtn_z_mz2_ {
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uqcvtn_z_mz2_ {
        #[inline]
        pub const fn new(
            U: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<4>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { U, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101001100010100u32 << 12u32
                    | self.U.into_inner() << 11u32
                    | 0b0u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0u32 << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
