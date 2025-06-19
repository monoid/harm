/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqinch_z_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqinch_z_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqinch_z_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11000u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod sqdech_z_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdech_z_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdech_z_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11001u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod uqinch_z_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqinch_z_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqinch_z_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11000u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod uqdech_z_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdech_z_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdech_z_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11001u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod sqincw_z_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqincw_z_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqincw_z_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11000u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod sqdecw_z_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdecw_z_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdecw_z_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11001u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod uqincw_z_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqincw_z_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqincw_z_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11000u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod uqdecw_z_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdecw_z_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdecw_z_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11001u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod sqincd_z_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqincd_z_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqincd_z_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11000u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod sqdecd_z_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqdecd_z_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqdecd_z_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11001u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod uqincd_z_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqincd_z_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqincd_z_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11000u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod uqdecd_z_zs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqdecd_z_zs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqdecd_z_zs_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm4: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            pattern: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm4: imm4.into(),
                U: U.into(),
                pattern: pattern.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10u32 << 20u32
                    | u32::from(self.imm4) << 16u32
                    | 0b11001u32 << 11u32
                    | u32::from(self.U) << 10u32
                    | u32::from(self.pattern) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
