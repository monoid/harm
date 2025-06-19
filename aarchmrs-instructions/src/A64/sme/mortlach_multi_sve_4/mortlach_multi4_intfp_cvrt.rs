/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod scvtf_mz_z_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct scvtf_mz_z_4 {
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl scvtf_mz_z_4 {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { Zn, U, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000100110010111000u32 << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b0u32 << 6u32
                    | self.U.into_inner() << 5u32
                    | self.Zd.into_inner() << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
pub mod ucvtf_mz_z_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ucvtf_mz_z_4 {
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl ucvtf_mz_z_4 {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { Zn, U, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000100110010111000u32 << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b0u32 << 6u32
                    | self.U.into_inner() << 5u32
                    | self.Zd.into_inner() << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
