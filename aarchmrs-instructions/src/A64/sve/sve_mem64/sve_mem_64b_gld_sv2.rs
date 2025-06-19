/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1sh_z_p_bz_d_64_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1sh_z_p_bz_d_64_scaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1sh_z_p_bz_d_64_scaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            ff: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Zm,
                U,
                ff,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100111u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.U.into_inner() << 14u32
                    | self.ff.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1sw_z_p_bz_d_64_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1sw_z_p_bz_d_64_scaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1sw_z_p_bz_d_64_scaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            ff: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Zm,
                U,
                ff,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000101011u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.U.into_inner() << 14u32
                    | self.ff.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1d_z_p_bz_d_64_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1d_z_p_bz_d_64_scaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1d_z_p_bz_d_64_scaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            ff: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, ff, Pg, Rn, Zt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000101111u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b11u32 << 14u32
                    | self.ff.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1h_z_p_bz_d_64_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1h_z_p_bz_d_64_scaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1h_z_p_bz_d_64_scaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            ff: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Zm,
                U,
                ff,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100111u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.U.into_inner() << 14u32
                    | self.ff.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1w_z_p_bz_d_64_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1w_z_p_bz_d_64_scaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1w_z_p_bz_d_64_scaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            ff: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Zm,
                U,
                ff,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000101011u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.U.into_inner() << 14u32
                    | self.ff.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ldff1sh_z_p_bz_d_64_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldff1sh_z_p_bz_d_64_scaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldff1sh_z_p_bz_d_64_scaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            ff: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Zm,
                U,
                ff,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100111u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.U.into_inner() << 14u32
                    | self.ff.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ldff1sw_z_p_bz_d_64_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldff1sw_z_p_bz_d_64_scaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldff1sw_z_p_bz_d_64_scaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            ff: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Zm,
                U,
                ff,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000101011u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.U.into_inner() << 14u32
                    | self.ff.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ldff1d_z_p_bz_d_64_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldff1d_z_p_bz_d_64_scaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldff1d_z_p_bz_d_64_scaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            ff: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, ff, Pg, Rn, Zt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000101111u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b11u32 << 14u32
                    | self.ff.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ldff1h_z_p_bz_d_64_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldff1h_z_p_bz_d_64_scaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldff1h_z_p_bz_d_64_scaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            ff: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Zm,
                U,
                ff,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100111u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.U.into_inner() << 14u32
                    | self.ff.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ldff1w_z_p_bz_d_64_scaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldff1w_z_p_bz_d_64_scaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldff1w_z_p_bz_d_64_scaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            ff: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Zm,
                U,
                ff,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000101011u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.U.into_inner() << 14u32
                    | self.ff.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
