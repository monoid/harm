/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod addp_z_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct addp_z_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl addp_z_p_zz_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pg, Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b010001101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod smaxp_z_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smaxp_z_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl smaxp_z_p_zz_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                U,
                Pg,
                Zm,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b01010u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod sminp_z_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sminp_z_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sminp_z_p_zz_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                U,
                Pg,
                Zm,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b01011u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod umaxp_z_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umaxp_z_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl umaxp_z_p_zz_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                U,
                Pg,
                Zm,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b01010u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod uminp_z_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uminp_z_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uminp_z_p_zz_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                U,
                Pg,
                Zm,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b01011u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
