/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod scvtf_mz_z_2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct scvtf_mz_z_2 {
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<4>,
    }
    impl scvtf_mz_z_2 {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Zn, U, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000100100010111000u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | self.U.into_inner() << 5u32
                    | self.Zd.into_inner() << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
pub mod ucvtf_mz_z_2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ucvtf_mz_z_2 {
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<4>,
    }
    impl ucvtf_mz_z_2 {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<4>,
            U: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Zn, U, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000100100010111000u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | self.U.into_inner() << 5u32
                    | self.Zd.into_inner() << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
