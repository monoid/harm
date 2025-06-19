/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smaxqv_z_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smaxqv_z_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Vd: ::aarchmrs_types::BitValue<5>,
    }
    impl smaxqv_z_p_z_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Vd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                U,
                Pg,
                Zn,
                Vd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b00110u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Vd.into_inner() << 0u32,
            )
        }
    }
}
pub mod sminqv_z_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sminqv_z_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Vd: ::aarchmrs_types::BitValue<5>,
    }
    impl sminqv_z_p_z_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Vd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                U,
                Pg,
                Zn,
                Vd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b00111u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Vd.into_inner() << 0u32,
            )
        }
    }
}
pub mod umaxqv_z_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umaxqv_z_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Vd: ::aarchmrs_types::BitValue<5>,
    }
    impl umaxqv_z_p_z_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Vd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                U,
                Pg,
                Zn,
                Vd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b00110u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Vd.into_inner() << 0u32,
            )
        }
    }
}
pub mod uminqv_z_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uminqv_z_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Vd: ::aarchmrs_types::BitValue<5>,
    }
    impl uminqv_z_p_z_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Vd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                U,
                Pg,
                Zn,
                Vd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b00111u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Vd.into_inner() << 0u32,
            )
        }
    }
}
