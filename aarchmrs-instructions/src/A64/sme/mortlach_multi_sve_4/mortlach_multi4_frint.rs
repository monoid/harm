/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod frintn_mz_z_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct frintn_mz_z_4 {
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl frintn_mz_z_4 {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<3>,
            Zd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000110111000111000u32 << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b00u32 << 5u32
                    | self.Zd.into_inner() << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
pub mod frintp_mz_z_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct frintp_mz_z_4 {
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl frintp_mz_z_4 {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<3>,
            Zd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000110111001111000u32 << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b00u32 << 5u32
                    | self.Zd.into_inner() << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
pub mod frintm_mz_z_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct frintm_mz_z_4 {
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl frintm_mz_z_4 {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<3>,
            Zd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000110111010111000u32 << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b00u32 << 5u32
                    | self.Zd.into_inner() << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
pub mod frinta_mz_z_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct frinta_mz_z_4 {
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl frinta_mz_z_4 {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<3>,
            Zd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000110111100111000u32 << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b00u32 << 5u32
                    | self.Zd.into_inner() << 2u32
                    | 0b00u32 << 0u32,
            )
        }
    }
}
