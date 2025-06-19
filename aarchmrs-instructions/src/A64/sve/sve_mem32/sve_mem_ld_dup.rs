/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1rb_z_p_bi_u8 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rb_z_p_bi_u8 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rb_z_p_bi_u8 {
        #[inline]
        pub fn new(
            dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtypeh: dtypeh.into(),
                imm6: imm6.into(),
                dtypel: dtypel.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | u32::from(self.dtypeh) << 23u32
                    | 0b1u32 << 22u32
                    | u32::from(self.imm6) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.dtypel) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1rb_z_p_bi_u16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rb_z_p_bi_u16 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rb_z_p_bi_u16 {
        #[inline]
        pub fn new(
            dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtypeh: dtypeh.into(),
                imm6: imm6.into(),
                dtypel: dtypel.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | u32::from(self.dtypeh) << 23u32
                    | 0b1u32 << 22u32
                    | u32::from(self.imm6) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.dtypel) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1rb_z_p_bi_u32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rb_z_p_bi_u32 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rb_z_p_bi_u32 {
        #[inline]
        pub fn new(
            dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtypeh: dtypeh.into(),
                imm6: imm6.into(),
                dtypel: dtypel.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | u32::from(self.dtypeh) << 23u32
                    | 0b1u32 << 22u32
                    | u32::from(self.imm6) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.dtypel) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1rb_z_p_bi_u64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rb_z_p_bi_u64 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rb_z_p_bi_u64 {
        #[inline]
        pub fn new(
            dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtypeh: dtypeh.into(),
                imm6: imm6.into(),
                dtypel: dtypel.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | u32::from(self.dtypeh) << 23u32
                    | 0b1u32 << 22u32
                    | u32::from(self.imm6) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.dtypel) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1rsw_z_p_bi_s64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rsw_z_p_bi_s64 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rsw_z_p_bi_s64 {
        #[inline]
        pub fn new(
            dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtypeh: dtypeh.into(),
                imm6: imm6.into(),
                dtypel: dtypel.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | u32::from(self.dtypeh) << 23u32
                    | 0b1u32 << 22u32
                    | u32::from(self.imm6) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.dtypel) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1rh_z_p_bi_u16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rh_z_p_bi_u16 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rh_z_p_bi_u16 {
        #[inline]
        pub fn new(
            dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtypeh: dtypeh.into(),
                imm6: imm6.into(),
                dtypel: dtypel.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | u32::from(self.dtypeh) << 23u32
                    | 0b1u32 << 22u32
                    | u32::from(self.imm6) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.dtypel) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1rh_z_p_bi_u32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rh_z_p_bi_u32 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rh_z_p_bi_u32 {
        #[inline]
        pub fn new(
            dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtypeh: dtypeh.into(),
                imm6: imm6.into(),
                dtypel: dtypel.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | u32::from(self.dtypeh) << 23u32
                    | 0b1u32 << 22u32
                    | u32::from(self.imm6) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.dtypel) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1rh_z_p_bi_u64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rh_z_p_bi_u64 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rh_z_p_bi_u64 {
        #[inline]
        pub fn new(
            dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtypeh: dtypeh.into(),
                imm6: imm6.into(),
                dtypel: dtypel.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | u32::from(self.dtypeh) << 23u32
                    | 0b1u32 << 22u32
                    | u32::from(self.imm6) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.dtypel) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1rsh_z_p_bi_s64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rsh_z_p_bi_s64 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rsh_z_p_bi_s64 {
        #[inline]
        pub fn new(
            dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtypeh: dtypeh.into(),
                imm6: imm6.into(),
                dtypel: dtypel.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | u32::from(self.dtypeh) << 23u32
                    | 0b1u32 << 22u32
                    | u32::from(self.imm6) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.dtypel) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1rsh_z_p_bi_s32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rsh_z_p_bi_s32 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rsh_z_p_bi_s32 {
        #[inline]
        pub fn new(
            dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtypeh: dtypeh.into(),
                imm6: imm6.into(),
                dtypel: dtypel.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | u32::from(self.dtypeh) << 23u32
                    | 0b1u32 << 22u32
                    | u32::from(self.imm6) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.dtypel) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1rw_z_p_bi_u32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rw_z_p_bi_u32 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rw_z_p_bi_u32 {
        #[inline]
        pub fn new(
            dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtypeh: dtypeh.into(),
                imm6: imm6.into(),
                dtypel: dtypel.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | u32::from(self.dtypeh) << 23u32
                    | 0b1u32 << 22u32
                    | u32::from(self.imm6) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.dtypel) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1rw_z_p_bi_u64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rw_z_p_bi_u64 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rw_z_p_bi_u64 {
        #[inline]
        pub fn new(
            dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtypeh: dtypeh.into(),
                imm6: imm6.into(),
                dtypel: dtypel.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | u32::from(self.dtypeh) << 23u32
                    | 0b1u32 << 22u32
                    | u32::from(self.imm6) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.dtypel) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1rsb_z_p_bi_s64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rsb_z_p_bi_s64 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rsb_z_p_bi_s64 {
        #[inline]
        pub fn new(
            dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtypeh: dtypeh.into(),
                imm6: imm6.into(),
                dtypel: dtypel.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | u32::from(self.dtypeh) << 23u32
                    | 0b1u32 << 22u32
                    | u32::from(self.imm6) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.dtypel) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1rsb_z_p_bi_s32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rsb_z_p_bi_s32 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rsb_z_p_bi_s32 {
        #[inline]
        pub fn new(
            dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtypeh: dtypeh.into(),
                imm6: imm6.into(),
                dtypel: dtypel.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | u32::from(self.dtypeh) << 23u32
                    | 0b1u32 << 22u32
                    | u32::from(self.imm6) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.dtypel) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1rsb_z_p_bi_s16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rsb_z_p_bi_s16 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rsb_z_p_bi_s16 {
        #[inline]
        pub fn new(
            dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtypeh: dtypeh.into(),
                imm6: imm6.into(),
                dtypel: dtypel.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | u32::from(self.dtypeh) << 23u32
                    | 0b1u32 << 22u32
                    | u32::from(self.imm6) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.dtypel) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
pub mod ld1rd_z_p_bi_u64 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rd_z_p_bi_u64 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rd_z_p_bi_u64 {
        #[inline]
        pub fn new(
            dtypeh: impl Into<::aarchmrs_types::BitValue<2>>,
            imm6: impl Into<::aarchmrs_types::BitValue<6>>,
            dtypel: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                dtypeh: dtypeh.into(),
                imm6: imm6.into(),
                dtypel: dtypel.into(),
                Pg: Pg.into(),
                Rn: Rn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | u32::from(self.dtypeh) << 23u32
                    | 0b1u32 << 22u32
                    | u32::from(self.imm6) << 16u32
                    | 0b1u32 << 15u32
                    | u32::from(self.dtypel) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
