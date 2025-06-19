/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod lasta_r_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct lasta_r_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub B: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl lasta_r_p_z_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            B: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                B,
                Pg,
                Zn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10000u32 << 17u32
                    | self.B.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod lastb_r_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct lastb_r_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub B: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl lastb_r_p_z_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            B: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                B,
                Pg,
                Zn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b10000u32 << 17u32
                    | self.B.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
