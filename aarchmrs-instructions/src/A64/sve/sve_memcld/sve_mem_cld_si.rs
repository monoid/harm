/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1b_z_p_bi_u8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1b_z_p_bi_u8 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1b_z_p_bi_u8 {
        #[inline]
        pub const fn new(
            dtype: ::aarchmrs_types::BitValue<4>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtype,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.dtype.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1b_z_p_bi_u16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1b_z_p_bi_u16 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1b_z_p_bi_u16 {
        #[inline]
        pub const fn new(
            dtype: ::aarchmrs_types::BitValue<4>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtype,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.dtype.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1b_z_p_bi_u32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1b_z_p_bi_u32 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1b_z_p_bi_u32 {
        #[inline]
        pub const fn new(
            dtype: ::aarchmrs_types::BitValue<4>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtype,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.dtype.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1b_z_p_bi_u64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1b_z_p_bi_u64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1b_z_p_bi_u64 {
        #[inline]
        pub const fn new(
            dtype: ::aarchmrs_types::BitValue<4>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtype,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.dtype.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1sw_z_p_bi_s64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1sw_z_p_bi_s64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1sw_z_p_bi_s64 {
        #[inline]
        pub const fn new(
            dtype: ::aarchmrs_types::BitValue<4>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtype,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.dtype.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1h_z_p_bi_u16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1h_z_p_bi_u16 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1h_z_p_bi_u16 {
        #[inline]
        pub const fn new(
            dtype: ::aarchmrs_types::BitValue<4>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtype,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.dtype.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1h_z_p_bi_u32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1h_z_p_bi_u32 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1h_z_p_bi_u32 {
        #[inline]
        pub const fn new(
            dtype: ::aarchmrs_types::BitValue<4>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtype,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.dtype.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1h_z_p_bi_u64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1h_z_p_bi_u64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1h_z_p_bi_u64 {
        #[inline]
        pub const fn new(
            dtype: ::aarchmrs_types::BitValue<4>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtype,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.dtype.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1sh_z_p_bi_s64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1sh_z_p_bi_s64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1sh_z_p_bi_s64 {
        #[inline]
        pub const fn new(
            dtype: ::aarchmrs_types::BitValue<4>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtype,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.dtype.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1sh_z_p_bi_s32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1sh_z_p_bi_s32 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1sh_z_p_bi_s32 {
        #[inline]
        pub const fn new(
            dtype: ::aarchmrs_types::BitValue<4>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtype,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.dtype.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1w_z_p_bi_u32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1w_z_p_bi_u32 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1w_z_p_bi_u32 {
        #[inline]
        pub const fn new(
            dtype: ::aarchmrs_types::BitValue<4>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtype,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.dtype.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1w_z_p_bi_u64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1w_z_p_bi_u64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1w_z_p_bi_u64 {
        #[inline]
        pub const fn new(
            dtype: ::aarchmrs_types::BitValue<4>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtype,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.dtype.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1sb_z_p_bi_s64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1sb_z_p_bi_s64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1sb_z_p_bi_s64 {
        #[inline]
        pub const fn new(
            dtype: ::aarchmrs_types::BitValue<4>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtype,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.dtype.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1sb_z_p_bi_s32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1sb_z_p_bi_s32 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1sb_z_p_bi_s32 {
        #[inline]
        pub const fn new(
            dtype: ::aarchmrs_types::BitValue<4>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtype,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.dtype.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1sb_z_p_bi_s16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1sb_z_p_bi_s16 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1sb_z_p_bi_s16 {
        #[inline]
        pub const fn new(
            dtype: ::aarchmrs_types::BitValue<4>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtype,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.dtype.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod ld1d_z_p_bi_u64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1d_z_p_bi_u64 {
        pub dtype: ::aarchmrs_types::BitValue<4>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1d_z_p_bi_u64 {
        #[inline]
        pub const fn new(
            dtype: ::aarchmrs_types::BitValue<4>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtype,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.dtype.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
