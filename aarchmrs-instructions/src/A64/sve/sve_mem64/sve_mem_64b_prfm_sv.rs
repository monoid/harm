/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod prfb_i_p_bz_d_x32_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfb_i_p_bz_d_x32_scaled {
        pub xs: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfb_i_p_bz_d_x32_scaled {
        #[inline]
        pub const fn new(
            xs: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            prfop: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                xs,
                Zm,
                msz,
                Pg,
                Rn,
                prfop,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110001000u32 << 23u32
                    | self.xs.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.prfop.into_inner() << 0u32,
            )
        }
    }
}
pub mod prfh_i_p_bz_d_x32_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfh_i_p_bz_d_x32_scaled {
        pub xs: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfh_i_p_bz_d_x32_scaled {
        #[inline]
        pub const fn new(
            xs: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            prfop: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                xs,
                Zm,
                msz,
                Pg,
                Rn,
                prfop,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110001000u32 << 23u32
                    | self.xs.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.prfop.into_inner() << 0u32,
            )
        }
    }
}
pub mod prfw_i_p_bz_d_x32_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfw_i_p_bz_d_x32_scaled {
        pub xs: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfw_i_p_bz_d_x32_scaled {
        #[inline]
        pub const fn new(
            xs: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            prfop: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                xs,
                Zm,
                msz,
                Pg,
                Rn,
                prfop,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110001000u32 << 23u32
                    | self.xs.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.prfop.into_inner() << 0u32,
            )
        }
    }
}
pub mod prfd_i_p_bz_d_x32_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfd_i_p_bz_d_x32_scaled {
        pub xs: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfd_i_p_bz_d_x32_scaled {
        #[inline]
        pub const fn new(
            xs: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            prfop: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                xs,
                Zm,
                msz,
                Pg,
                Rn,
                prfop,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110001000u32 << 23u32
                    | self.xs.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.prfop.into_inner() << 0u32,
            )
        }
    }
}
