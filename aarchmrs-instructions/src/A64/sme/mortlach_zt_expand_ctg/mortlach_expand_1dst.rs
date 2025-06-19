/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod luti2_z_ztz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct luti2_z_ztz_ {
        pub i4: ::aarchmrs_types::BitValue<4>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl luti2_z_ztz_ {
        #[inline]
        pub const fn new(
            i4: ::aarchmrs_types::BitValue<4>,
            size: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { i4, size, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000110011u32 << 18u32
                    | self.i4.into_inner() << 14u32
                    | self.size.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod luti4_z_ztz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct luti4_z_ztz_ {
        pub i3: ::aarchmrs_types::BitValue<3>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl luti4_z_ztz_ {
        #[inline]
        pub const fn new(
            i3: ::aarchmrs_types::BitValue<3>,
            size: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { i3, size, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000001100101u32 << 17u32
                    | self.i3.into_inner() << 14u32
                    | self.size.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
