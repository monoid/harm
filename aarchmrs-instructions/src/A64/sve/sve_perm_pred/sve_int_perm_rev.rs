/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod revb_z_z_m {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct revb_z_z_m {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl revb_z_z_m {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pg, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b100100100u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod revb_z_z_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct revb_z_z_z {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl revb_z_z_z {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pg, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b100100101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod revh_z_z_m {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct revh_z_z_m {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl revh_z_z_m {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pg, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b100101100u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod revh_z_z_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct revh_z_z_z {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl revh_z_z_z {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pg, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b100101101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod revw_z_z_m {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct revw_z_z_m {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl revw_z_z_m {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pg, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b100110100u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod revw_z_z_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct revw_z_z_z {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl revw_z_z_z {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pg, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b100110101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod rbit_z_p_z_m {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct rbit_z_p_z_m {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl rbit_z_p_z_m {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pg, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b100111100u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
pub mod rbit_z_p_z_z {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct rbit_z_p_z_z {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl rbit_z_p_z_z {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pg, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b100111101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
