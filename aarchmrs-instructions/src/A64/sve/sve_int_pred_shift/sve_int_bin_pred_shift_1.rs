/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod asr_z_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct asr_z_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl asr_z_p_zz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                Pg: Pg.into(),
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b01000u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod lsl_z_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct lsl_z_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl lsl_z_p_zz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pg: Pg.into(),
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b010011100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod asrr_z_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct asrr_z_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl asrr_z_p_zz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                Pg: Pg.into(),
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b01010u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod lslr_z_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct lslr_z_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl lslr_z_p_zz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pg: Pg.into(),
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b010111100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod lsr_z_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct lsr_z_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl lsr_z_p_zz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                Pg: Pg.into(),
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b01000u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod lsrr_z_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct lsrr_z_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl lsrr_z_p_zz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                Pg: Pg.into(),
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b01010u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
