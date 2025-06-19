/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod clasta_r_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct clasta_r_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub B: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl clasta_r_p_z_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            B: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                B,
                Pg,
                Zm,
                Rdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b11000u32 << 17u32
                    | self.B.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod clastb_r_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct clastb_r_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub B: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl clastb_r_p_z_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            B: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                B,
                Pg,
                Zm,
                Rdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b11000u32 << 17u32
                    | self.B.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
            )
        }
    }
}
