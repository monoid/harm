/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ldnf1b_z_p_bi_u8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnf1b_z_p_bi_u8 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnf1b_z_p_bi_u8 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                imm4: imm4.into(),
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
                    | 0b1u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldnf1b_z_p_bi_u16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnf1b_z_p_bi_u16 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnf1b_z_p_bi_u16 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                imm4: imm4.into(),
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
                    | 0b1u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldnf1b_z_p_bi_u32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnf1b_z_p_bi_u32 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnf1b_z_p_bi_u32 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                imm4: imm4.into(),
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
                    | 0b1u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldnf1b_z_p_bi_u64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnf1b_z_p_bi_u64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnf1b_z_p_bi_u64 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                imm4: imm4.into(),
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
                    | 0b1u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldnf1sw_z_p_bi_s64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnf1sw_z_p_bi_s64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnf1sw_z_p_bi_s64 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                imm4: imm4.into(),
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
                    | 0b1u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldnf1h_z_p_bi_u16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnf1h_z_p_bi_u16 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnf1h_z_p_bi_u16 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                imm4: imm4.into(),
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
                    | 0b1u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldnf1h_z_p_bi_u32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnf1h_z_p_bi_u32 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnf1h_z_p_bi_u32 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                imm4: imm4.into(),
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
                    | 0b1u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldnf1h_z_p_bi_u64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnf1h_z_p_bi_u64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnf1h_z_p_bi_u64 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                imm4: imm4.into(),
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
                    | 0b1u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldnf1sh_z_p_bi_s64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnf1sh_z_p_bi_s64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnf1sh_z_p_bi_s64 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                imm4: imm4.into(),
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
                    | 0b1u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldnf1sh_z_p_bi_s32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnf1sh_z_p_bi_s32 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnf1sh_z_p_bi_s32 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                imm4: imm4.into(),
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
                    | 0b1u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldnf1w_z_p_bi_u32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnf1w_z_p_bi_u32 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnf1w_z_p_bi_u32 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                imm4: imm4.into(),
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
                    | 0b1u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldnf1w_z_p_bi_u64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnf1w_z_p_bi_u64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnf1w_z_p_bi_u64 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                imm4: imm4.into(),
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
                    | 0b1u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldnf1sb_z_p_bi_s64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnf1sb_z_p_bi_s64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnf1sb_z_p_bi_s64 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                imm4: imm4.into(),
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
                    | 0b1u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldnf1sb_z_p_bi_s32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnf1sb_z_p_bi_s32 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnf1sb_z_p_bi_s32 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                imm4: imm4.into(),
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
                    | 0b1u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldnf1sb_z_p_bi_s16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnf1sb_z_p_bi_s16 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnf1sb_z_p_bi_s16 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                imm4: imm4.into(),
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
                    | 0b1u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldnf1d_z_p_bi_u64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnf1d_z_p_bi_u64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldnf1d_z_p_bi_u64 {
        #[inline]
        pub fn new(
            dtype: impl Into<::aarchmrs_types::BitValue<4>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtype: dtype.into(),
                imm4: imm4.into(),
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
                    | 0b1u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
