/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqshrunb_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqshrunb_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqshrunb_z_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | u32::from(self.imm3) << 16u32
                    | 0b00000u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sqrshrunb_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqrshrunb_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqrshrunb_z_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | u32::from(self.imm3) << 16u32
                    | 0b00001u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod shrnb_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct shrnb_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl shrnb_z_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | u32::from(self.imm3) << 16u32
                    | 0b00010u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod rshrnb_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct rshrnb_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl rshrnb_z_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | u32::from(self.imm3) << 16u32
                    | 0b00011u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sqshrnb_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqshrnb_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqshrnb_z_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                U: U.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | u32::from(self.imm3) << 16u32
                    | 0b001u32 << 13u32
                    | u32::from(self.U) << 12u32
                    | 0b0u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sqrshrnb_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqrshrnb_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqrshrnb_z_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                U: U.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | u32::from(self.imm3) << 16u32
                    | 0b001u32 << 13u32
                    | u32::from(self.U) << 12u32
                    | 0b1u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sqshrunt_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqshrunt_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqshrunt_z_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | u32::from(self.imm3) << 16u32
                    | 0b00000u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sqrshrunt_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqrshrunt_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqrshrunt_z_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | u32::from(self.imm3) << 16u32
                    | 0b00001u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod shrnt_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct shrnt_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl shrnt_z_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | u32::from(self.imm3) << 16u32
                    | 0b00010u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod rshrnt_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct rshrnt_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl rshrnt_z_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | u32::from(self.imm3) << 16u32
                    | 0b00011u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sqshrnt_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqshrnt_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqshrnt_z_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                U: U.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | u32::from(self.imm3) << 16u32
                    | 0b001u32 << 13u32
                    | u32::from(self.U) << 12u32
                    | 0b0u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod sqrshrnt_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqrshrnt_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqrshrnt_z_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                U: U.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | u32::from(self.imm3) << 16u32
                    | 0b001u32 << 13u32
                    | u32::from(self.U) << 12u32
                    | 0b1u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod uqshrnb_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqshrnb_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uqshrnb_z_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                U: U.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | u32::from(self.imm3) << 16u32
                    | 0b001u32 << 13u32
                    | u32::from(self.U) << 12u32
                    | 0b0u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod uqrshrnb_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqrshrnb_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uqrshrnb_z_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                U: U.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | u32::from(self.imm3) << 16u32
                    | 0b001u32 << 13u32
                    | u32::from(self.U) << 12u32
                    | 0b1u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod uqshrnt_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqshrnt_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uqshrnt_z_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                U: U.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | u32::from(self.imm3) << 16u32
                    | 0b001u32 << 13u32
                    | u32::from(self.U) << 12u32
                    | 0b0u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod uqrshrnt_z_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqrshrnt_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<1>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub T: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uqrshrnt_z_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<1>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            T: impl Into<::aarchmrs_types::BitValue<1>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                U: U.into(),
                T: T.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | u32::from(self.tszh) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.tszl) << 19u32
                    | u32::from(self.imm3) << 16u32
                    | 0b001u32 << 13u32
                    | u32::from(self.U) << 12u32
                    | 0b1u32 << 11u32
                    | u32::from(self.T) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
