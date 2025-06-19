/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod shadd_z_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct shadd_z_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl shadd_z_p_zz_ {
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
                0b01000100u32 << 24u32
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
pub mod shsub_z_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct shsub_z_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl shsub_z_p_zz_ {
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
                0b01000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b01001u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod srhadd_z_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct srhadd_z_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl srhadd_z_p_zz_ {
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
                0b01000100u32 << 24u32
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
pub mod shsubr_z_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct shsubr_z_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl shsubr_z_p_zz_ {
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
                0b01000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b01011u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod uhadd_z_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uhadd_z_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uhadd_z_p_zz_ {
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
                0b01000100u32 << 24u32
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
pub mod uhsub_z_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uhsub_z_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uhsub_z_p_zz_ {
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
                0b01000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b01001u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod urhadd_z_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct urhadd_z_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl urhadd_z_p_zz_ {
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
                0b01000100u32 << 24u32
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
pub mod uhsubr_z_p_zz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uhsubr_z_p_zz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uhsubr_z_p_zz_ {
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
                0b01000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b01011u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
