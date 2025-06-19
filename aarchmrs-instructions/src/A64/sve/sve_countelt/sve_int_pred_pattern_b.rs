/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqincb_r_rs_sx {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqincb_r_rs_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqincb_r_rs_sx {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111100u32 << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqincb_r_rs_uw {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqincb_r_rs_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqincb_r_rs_uw {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111101u32 << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod sqdecb_r_rs_sx {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdecb_r_rs_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdecb_r_rs_sx {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111110u32 << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqdecb_r_rs_uw {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdecb_r_rs_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdecb_r_rs_uw {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111111u32 << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod sqincb_r_rs_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqincb_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqincb_r_rs_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b11u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11110u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod sqdecb_r_rs_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdecb_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdecb_r_rs_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b11u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11111u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod sqinch_r_rs_sx {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqinch_r_rs_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqinch_r_rs_sx {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111100u32 << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqinch_r_rs_uw {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqinch_r_rs_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqinch_r_rs_uw {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111101u32 << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod sqdech_r_rs_sx {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdech_r_rs_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdech_r_rs_sx {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111110u32 << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqdech_r_rs_uw {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdech_r_rs_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdech_r_rs_uw {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111111u32 << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod sqinch_r_rs_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqinch_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqinch_r_rs_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b11u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11110u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod sqdech_r_rs_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdech_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdech_r_rs_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b11u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11111u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod sqincw_r_rs_sx {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqincw_r_rs_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqincw_r_rs_sx {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111100u32 << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqincw_r_rs_uw {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqincw_r_rs_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqincw_r_rs_uw {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111101u32 << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod sqdecw_r_rs_sx {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdecw_r_rs_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdecw_r_rs_sx {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111110u32 << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqdecw_r_rs_uw {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdecw_r_rs_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdecw_r_rs_uw {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111111u32 << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod sqincw_r_rs_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqincw_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqincw_r_rs_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b11u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11110u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod sqdecw_r_rs_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdecw_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdecw_r_rs_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b11u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11111u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod sqincd_r_rs_sx {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqincd_r_rs_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqincd_r_rs_sx {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111100u32 << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqincd_r_rs_uw {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqincd_r_rs_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqincd_r_rs_uw {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111101u32 << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod sqdecd_r_rs_sx {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdecd_r_rs_sx {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdecd_r_rs_sx {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111110u32 << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqdecd_r_rs_uw {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdecd_r_rs_uw {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdecd_r_rs_uw {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b111111u32 << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod sqincd_r_rs_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqincd_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqincd_r_rs_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b11u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11110u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod sqdecd_r_rs_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdecd_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdecd_r_rs_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b11u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11111u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqincb_r_rs_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqincb_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqincb_r_rs_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b11u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11110u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqdecb_r_rs_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdecb_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdecb_r_rs_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b11u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11111u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqinch_r_rs_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqinch_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqinch_r_rs_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b11u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11110u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqdech_r_rs_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdech_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdech_r_rs_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b11u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11111u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqincw_r_rs_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqincw_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqincw_r_rs_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b11u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11110u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqdecw_r_rs_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdecw_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdecw_r_rs_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b11u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11111u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqincd_r_rs_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqincd_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqincd_r_rs_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b11u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11110u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
pub mod uqdecd_r_rs_x {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdecd_r_rs_x {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdecd_r_rs_x {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Rdn: Rdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b11u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11111u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Rdn) << 0u32,
            )
        }
    }
}
