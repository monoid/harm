/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod st1b_z_p_bz_s_x32_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1b_z_p_bz_s_x32_unscaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub xs: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st1b_z_p_bz_s_x32_unscaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            xs: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, xs, Pg, Rn, Zt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100100010u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.xs.into_inner() << 14u32
                    | 0b0u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod st1h_z_p_bz_s_x32_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1h_z_p_bz_s_x32_unscaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub xs: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st1h_z_p_bz_s_x32_unscaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            xs: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, xs, Pg, Rn, Zt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100100110u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.xs.into_inner() << 14u32
                    | 0b0u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod st1w_z_p_bz_s_x32_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1w_z_p_bz_s_x32_unscaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub xs: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st1w_z_p_bz_s_x32_unscaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            xs: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, xs, Pg, Rn, Zt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100101010u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.xs.into_inner() << 14u32
                    | 0b0u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
