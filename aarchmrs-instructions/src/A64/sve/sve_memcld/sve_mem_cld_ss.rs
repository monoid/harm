/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1b_z_p_br_u8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1b_z_p_br_u8 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1b_z_p_br_u8 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | u32::from(self.dtype) << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b010u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1b_z_p_br_u16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1b_z_p_br_u16 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1b_z_p_br_u16 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | u32::from(self.dtype) << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b010u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1b_z_p_br_u32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1b_z_p_br_u32 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1b_z_p_br_u32 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | u32::from(self.dtype) << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b010u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1b_z_p_br_u64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1b_z_p_br_u64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1b_z_p_br_u64 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | u32::from(self.dtype) << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b010u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1sw_z_p_br_s64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1sw_z_p_br_s64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1sw_z_p_br_s64 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | u32::from(self.dtype) << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b010u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1h_z_p_br_u16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1h_z_p_br_u16 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1h_z_p_br_u16 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | u32::from(self.dtype) << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b010u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1h_z_p_br_u32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1h_z_p_br_u32 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1h_z_p_br_u32 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | u32::from(self.dtype) << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b010u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1h_z_p_br_u64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1h_z_p_br_u64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1h_z_p_br_u64 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | u32::from(self.dtype) << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b010u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1sh_z_p_br_s64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1sh_z_p_br_s64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1sh_z_p_br_s64 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | u32::from(self.dtype) << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b010u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1sh_z_p_br_s32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1sh_z_p_br_s32 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1sh_z_p_br_s32 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | u32::from(self.dtype) << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b010u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1w_z_p_br_u32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1w_z_p_br_u32 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1w_z_p_br_u32 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | u32::from(self.dtype) << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b010u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1w_z_p_br_u64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1w_z_p_br_u64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1w_z_p_br_u64 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | u32::from(self.dtype) << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b010u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1sb_z_p_br_s64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1sb_z_p_br_s64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1sb_z_p_br_s64 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | u32::from(self.dtype) << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b010u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1sb_z_p_br_s32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1sb_z_p_br_s32 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1sb_z_p_br_s32 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | u32::from(self.dtype) << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b010u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1sb_z_p_br_s16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1sb_z_p_br_s16 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1sb_z_p_br_s16 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | u32::from(self.dtype) << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b010u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1d_z_p_br_u64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1d_z_p_br_u64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1d_z_p_br_u64 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                Rm: Rm.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | u32::from(self.dtype) << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b010u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
