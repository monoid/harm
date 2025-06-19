/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1b_za_p_rrr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1b_za_p_rrr_ {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub off4: ::aarchmrs_types::BitValue<4>,
    }
    impl ld1b_za_p_rrr_ {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            V: ::aarchmrs_types::BitValue<1>,
            Rs: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            off4: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                Rm,
                V,
                Rs,
                Pg,
                Rn,
                off4,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100000000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.V.into_inner() << 15u32
                    | self.Rs.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.off4.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1h_za_p_rrr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1h_za_p_rrr_ {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub ZAt: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl ld1h_za_p_rrr_ {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            V: ::aarchmrs_types::BitValue<1>,
            Rs: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            ZAt: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                Rm,
                V,
                Rs,
                Pg,
                Rn,
                ZAt,
                off3,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100000010u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.V.into_inner() << 15u32
                    | self.Rs.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.ZAt.into_inner() << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1w_za_p_rrr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1w_za_p_rrr_ {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub ZAt: ::aarchmrs_types::BitValue<2>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl ld1w_za_p_rrr_ {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            V: ::aarchmrs_types::BitValue<1>,
            Rs: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            ZAt: ::aarchmrs_types::BitValue<2>,
            off2: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Rm,
                V,
                Rs,
                Pg,
                Rn,
                ZAt,
                off2,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100000100u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.V.into_inner() << 15u32
                    | self.Rs.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.ZAt.into_inner() << 2u32
                    | self.off2.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1d_za_p_rrr_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1d_za_p_rrr_ {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub ZAt: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
    }
    impl ld1d_za_p_rrr_ {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            V: ::aarchmrs_types::BitValue<1>,
            Rs: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            ZAt: ::aarchmrs_types::BitValue<3>,
            o1: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self {
                Rm,
                V,
                Rs,
                Pg,
                Rn,
                ZAt,
                o1,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100000110u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.V.into_inner() << 15u32
                    | self.Rs.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.ZAt.into_inner() << 1u32
                    | self.o1.into_inner() << 0u32,
            )
        }
    }
}
