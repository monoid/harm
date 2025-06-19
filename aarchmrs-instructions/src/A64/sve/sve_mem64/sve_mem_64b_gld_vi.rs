/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1sb_z_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1sb_z_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1sb_z_p_ai_d {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            ff: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                U: U.into(),
                ff: ff.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100001u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.U) << 14u32
                    | u32::from(self.ff) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1sh_z_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1sh_z_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1sh_z_p_ai_d {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            ff: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                U: U.into(),
                ff: ff.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100101u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.U) << 14u32
                    | u32::from(self.ff) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1sw_z_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1sw_z_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1sw_z_p_ai_d {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            ff: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                U: U.into(),
                ff: ff.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000101001u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.U) << 14u32
                    | u32::from(self.ff) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1d_z_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1d_z_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1d_z_p_ai_d {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            ff: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                ff: ff.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000101101u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b11u32 << 14u32
                    | u32::from(self.ff) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1b_z_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1b_z_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1b_z_p_ai_d {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            ff: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                U: U.into(),
                ff: ff.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100001u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.U) << 14u32
                    | u32::from(self.ff) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1h_z_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1h_z_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1h_z_p_ai_d {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            ff: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                U: U.into(),
                ff: ff.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100101u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.U) << 14u32
                    | u32::from(self.ff) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1w_z_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1w_z_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1w_z_p_ai_d {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            ff: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                U: U.into(),
                ff: ff.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000101001u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.U) << 14u32
                    | u32::from(self.ff) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldff1sb_z_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldff1sb_z_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldff1sb_z_p_ai_d {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            ff: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                U: U.into(),
                ff: ff.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100001u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.U) << 14u32
                    | u32::from(self.ff) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldff1sh_z_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldff1sh_z_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldff1sh_z_p_ai_d {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            ff: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                U: U.into(),
                ff: ff.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100101u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.U) << 14u32
                    | u32::from(self.ff) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldff1sw_z_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldff1sw_z_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldff1sw_z_p_ai_d {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            ff: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                U: U.into(),
                ff: ff.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000101001u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.U) << 14u32
                    | u32::from(self.ff) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldff1d_z_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldff1d_z_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldff1d_z_p_ai_d {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            ff: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                ff: ff.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000101101u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b11u32 << 14u32
                    | u32::from(self.ff) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldff1b_z_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldff1b_z_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldff1b_z_p_ai_d {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            ff: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                U: U.into(),
                ff: ff.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100001u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.U) << 14u32
                    | u32::from(self.ff) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldff1h_z_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldff1h_z_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldff1h_z_p_ai_d {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            ff: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                U: U.into(),
                ff: ff.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100101u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.U) << 14u32
                    | u32::from(self.ff) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ldff1w_z_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldff1w_z_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldff1w_z_p_ai_d {
        #[inline]
        pub fn new(
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            ff: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                imm5: imm5.into(),
                U: U.into(),
                ff: ff.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000101001u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.U) << 14u32
                    | u32::from(self.ff) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
