/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ldnt1sb_z_p_ar_d_64_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1sb_z_p_ar_d_64_unscaled {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnt1sb_z_p_ar_d_64_unscaled {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, U, Pg, Zn, Zt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.U.into_inner() << 14u32
                    | 0b0u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ldnt1sh_z_p_ar_d_64_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1sh_z_p_ar_d_64_unscaled {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnt1sh_z_p_ar_d_64_unscaled {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, U, Pg, Zn, Zt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100100u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.U.into_inner() << 14u32
                    | 0b0u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ldnt1sw_z_p_ar_d_64_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1sw_z_p_ar_d_64_unscaled {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnt1sw_z_p_ar_d_64_unscaled {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, U, Pg, Zn, Zt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000101000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.U.into_inner() << 14u32
                    | 0b0u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ldnt1d_z_p_ar_d_64_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1d_z_p_ar_d_64_unscaled {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnt1d_z_p_ar_d_64_unscaled {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, Pg, Zn, Zt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000101100u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b110u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ldnt1b_z_p_ar_d_64_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1b_z_p_ar_d_64_unscaled {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnt1b_z_p_ar_d_64_unscaled {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, U, Pg, Zn, Zt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.U.into_inner() << 14u32
                    | 0b0u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ldnt1h_z_p_ar_d_64_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1h_z_p_ar_d_64_unscaled {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnt1h_z_p_ar_d_64_unscaled {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, U, Pg, Zn, Zt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100100u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.U.into_inner() << 14u32
                    | 0b0u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ldnt1w_z_p_ar_d_64_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1w_z_p_ar_d_64_unscaled {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnt1w_z_p_ar_d_64_unscaled {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, U, Pg, Zn, Zt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000101000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.U.into_inner() << 14u32
                    | 0b0u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
