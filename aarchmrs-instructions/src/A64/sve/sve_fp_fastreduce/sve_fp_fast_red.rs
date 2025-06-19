/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod faddv_v_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct faddv_v_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Vd: ::aarchmrs_types::BitValue<5>,
    }
    impl faddv_v_p_z_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Vd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pg, Zn, Vd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b000000001u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Vd.into_inner() << 0u32,
            )
        }
    }
}
pub mod fmaxnmv_v_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmaxnmv_v_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Vd: ::aarchmrs_types::BitValue<5>,
    }
    impl fmaxnmv_v_p_z_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Vd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pg, Zn, Vd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b000100001u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Vd.into_inner() << 0u32,
            )
        }
    }
}
pub mod fminnmv_v_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fminnmv_v_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Vd: ::aarchmrs_types::BitValue<5>,
    }
    impl fminnmv_v_p_z_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Vd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pg, Zn, Vd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b000101001u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Vd.into_inner() << 0u32,
            )
        }
    }
}
pub mod fmaxv_v_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmaxv_v_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Vd: ::aarchmrs_types::BitValue<5>,
    }
    impl fmaxv_v_p_z_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Vd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pg, Zn, Vd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b000110001u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Vd.into_inner() << 0u32,
            )
        }
    }
}
pub mod fminv_v_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fminv_v_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Vd: ::aarchmrs_types::BitValue<5>,
    }
    impl fminv_v_p_z_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Vd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { size, Pg, Zn, Vd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b000111001u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Vd.into_inner() << 0u32,
            )
        }
    }
}
