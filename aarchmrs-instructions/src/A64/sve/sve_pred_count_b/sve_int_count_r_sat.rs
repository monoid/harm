/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqincp_r_p_r_sx {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqincp_r_p_r_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqincp_r_p_r_sx {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pm: Pm.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1010001000100u32 << 9u32
                    | u32::from(self.Pm) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqincp_r_p_r_uw {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqincp_r_p_r_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqincp_r_p_r_uw {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pm: Pm.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1010011000100u32 << 9u32
                    | u32::from(self.Pm) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod sqdecp_r_p_r_sx {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdecp_r_p_r_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdecp_r_p_r_sx {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pm: Pm.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1010101000100u32 << 9u32
                    | u32::from(self.Pm) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqdecp_r_p_r_uw {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdecp_r_p_r_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdecp_r_p_r_uw {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pm: Pm.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1010111000100u32 << 9u32
                    | u32::from(self.Pm) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod sqincp_r_p_r_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqincp_r_p_r_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqincp_r_p_r_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Pm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                Pm: Pm.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10100u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b1000110u32 << 9u32
                    | u32::from(self.Pm) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod sqdecp_r_p_r_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdecp_r_p_r_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdecp_r_p_r_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Pm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                Pm: Pm.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10101u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b1000110u32 << 9u32
                    | u32::from(self.Pm) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqincp_r_p_r_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqincp_r_p_r_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqincp_r_p_r_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Pm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                Pm: Pm.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10100u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b1000110u32 << 9u32
                    | u32::from(self.Pm) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqdecp_r_p_r_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdecp_r_p_r_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdecp_r_p_r_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Pm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                U: U.into(),
                Pm: Pm.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10101u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b1000110u32 << 9u32
                    | u32::from(self.Pm) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
