/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod st2b_z_p_bi_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st2b_z_p_bi_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st2b_z_p_bi_contiguous {
        #[inline]
        pub fn new(
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                msz: msz.into(),
                imm4: imm4.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110010u32 << 25u32
                    | u32::from(self.msz) << 23u32
                    | 0b011u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod st3b_z_p_bi_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st3b_z_p_bi_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st3b_z_p_bi_contiguous {
        #[inline]
        pub fn new(
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                msz: msz.into(),
                imm4: imm4.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110010u32 << 25u32
                    | u32::from(self.msz) << 23u32
                    | 0b101u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod st4b_z_p_bi_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st4b_z_p_bi_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st4b_z_p_bi_contiguous {
        #[inline]
        pub fn new(
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                msz: msz.into(),
                imm4: imm4.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110010u32 << 25u32
                    | u32::from(self.msz) << 23u32
                    | 0b111u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod st2h_z_p_bi_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st2h_z_p_bi_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st2h_z_p_bi_contiguous {
        #[inline]
        pub fn new(
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                msz: msz.into(),
                imm4: imm4.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110010u32 << 25u32
                    | u32::from(self.msz) << 23u32
                    | 0b011u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod st3h_z_p_bi_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st3h_z_p_bi_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st3h_z_p_bi_contiguous {
        #[inline]
        pub fn new(
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                msz: msz.into(),
                imm4: imm4.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110010u32 << 25u32
                    | u32::from(self.msz) << 23u32
                    | 0b101u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod st4h_z_p_bi_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st4h_z_p_bi_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st4h_z_p_bi_contiguous {
        #[inline]
        pub fn new(
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                msz: msz.into(),
                imm4: imm4.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110010u32 << 25u32
                    | u32::from(self.msz) << 23u32
                    | 0b111u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod st2w_z_p_bi_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st2w_z_p_bi_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st2w_z_p_bi_contiguous {
        #[inline]
        pub fn new(
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                msz: msz.into(),
                imm4: imm4.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110010u32 << 25u32
                    | u32::from(self.msz) << 23u32
                    | 0b011u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod st3w_z_p_bi_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st3w_z_p_bi_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st3w_z_p_bi_contiguous {
        #[inline]
        pub fn new(
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                msz: msz.into(),
                imm4: imm4.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110010u32 << 25u32
                    | u32::from(self.msz) << 23u32
                    | 0b101u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod st4w_z_p_bi_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st4w_z_p_bi_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st4w_z_p_bi_contiguous {
        #[inline]
        pub fn new(
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                msz: msz.into(),
                imm4: imm4.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110010u32 << 25u32
                    | u32::from(self.msz) << 23u32
                    | 0b111u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod st2d_z_p_bi_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st2d_z_p_bi_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st2d_z_p_bi_contiguous {
        #[inline]
        pub fn new(
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                msz: msz.into(),
                imm4: imm4.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110010u32 << 25u32
                    | u32::from(self.msz) << 23u32
                    | 0b011u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod st3d_z_p_bi_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st3d_z_p_bi_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st3d_z_p_bi_contiguous {
        #[inline]
        pub fn new(
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                msz: msz.into(),
                imm4: imm4.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110010u32 << 25u32
                    | u32::from(self.msz) << 23u32
                    | 0b101u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod st4d_z_p_bi_contiguous {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st4d_z_p_bi_contiguous {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st4d_z_p_bi_contiguous {
        #[inline]
        pub fn new(
            msz: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                msz: msz.into(),
                imm4: imm4.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1110010u32 << 25u32
                    | u32::from(self.msz) << 23u32
                    | 0b111u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
