/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mova_z_p_rza_b {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_z_p_rza_b {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub off4: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl mova_z_p_rza_b {
        #[inline]
        pub const fn new(
            V: ::aarchmrs_types::BitValue<1>,
            Rs: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            off4: ::aarchmrs_types::BitValue<4>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                V,
                Rs,
                Pg,
                off4,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000000000010u32 << 16u32
                    | self.V.into_inner() << 15u32
                    | self.Rs.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.off4.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod mova_z_p_rza_h {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_z_p_rza_h {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub ZAn: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl mova_z_p_rza_h {
        #[inline]
        pub const fn new(
            V: ::aarchmrs_types::BitValue<1>,
            Rs: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            ZAn: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                V,
                Rs,
                Pg,
                ZAn,
                off3,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000001000010u32 << 16u32
                    | self.V.into_inner() << 15u32
                    | self.Rs.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.ZAn.into_inner() << 8u32
                    | self.off3.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod mova_z_p_rza_w {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_z_p_rza_w {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub ZAn: ::aarchmrs_types::BitValue<2>,
        pub off2: ::aarchmrs_types::BitValue<2>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl mova_z_p_rza_w {
        #[inline]
        pub const fn new(
            V: ::aarchmrs_types::BitValue<1>,
            Rs: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            ZAn: ::aarchmrs_types::BitValue<2>,
            off2: ::aarchmrs_types::BitValue<2>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                V,
                Rs,
                Pg,
                ZAn,
                off2,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000010000010u32 << 16u32
                    | self.V.into_inner() << 15u32
                    | self.Rs.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.ZAn.into_inner() << 7u32
                    | self.off2.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod mova_z_p_rza_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_z_p_rza_d {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub ZAn: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl mova_z_p_rza_d {
        #[inline]
        pub const fn new(
            V: ::aarchmrs_types::BitValue<1>,
            Rs: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            ZAn: ::aarchmrs_types::BitValue<3>,
            o1: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                V,
                Rs,
                Pg,
                ZAn,
                o1,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000011000010u32 << 16u32
                    | self.V.into_inner() << 15u32
                    | self.Rs.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.ZAn.into_inner() << 6u32
                    | self.o1.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod mova_z_p_rza_q {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_z_p_rza_q {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub ZAn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl mova_z_p_rza_q {
        #[inline]
        pub const fn new(
            V: ::aarchmrs_types::BitValue<1>,
            Rs: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            ZAn: ::aarchmrs_types::BitValue<4>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { V, Rs, Pg, ZAn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000011000011u32 << 16u32
                    | self.V.into_inner() << 15u32
                    | self.Rs.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.ZAn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
