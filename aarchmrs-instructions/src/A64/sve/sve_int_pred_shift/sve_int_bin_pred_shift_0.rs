/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod asr_z_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct asr_z_p_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl asr_z_p_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                U: U.into(),
                Pg: Pg.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.tszh) << 22u32
                    | 0b00000u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.tszl) << 8u32
                    | u32::from(self.imm3) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod lsl_z_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct lsl_z_p_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl lsl_z_p_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                Pg: Pg.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.tszh) << 22u32
                    | 0b000011100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.tszl) << 8u32
                    | u32::from(self.imm3) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod asrd_z_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct asrd_z_p_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl asrd_z_p_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                Pg: Pg.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.tszh) << 22u32
                    | 0b000100100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.tszl) << 8u32
                    | u32::from(self.imm3) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod sqshl_z_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqshl_z_p_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqshl_z_p_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                U: U.into(),
                Pg: Pg.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.tszh) << 22u32
                    | 0b00011u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.tszl) << 8u32
                    | u32::from(self.imm3) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod srshr_z_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct srshr_z_p_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl srshr_z_p_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                U: U.into(),
                Pg: Pg.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.tszh) << 22u32
                    | 0b00110u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.tszl) << 8u32
                    | u32::from(self.imm3) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod sqshlu_z_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqshlu_z_p_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl sqshlu_z_p_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<2>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                Pg: Pg.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.tszh) << 22u32
                    | 0b001111100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.tszl) << 8u32
                    | u32::from(self.imm3) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod lsr_z_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct lsr_z_p_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl lsr_z_p_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                U: U.into(),
                Pg: Pg.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.tszh) << 22u32
                    | 0b00000u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.tszl) << 8u32
                    | u32::from(self.imm3) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod uqshl_z_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqshl_z_p_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl uqshl_z_p_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                U: U.into(),
                Pg: Pg.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.tszh) << 22u32
                    | 0b00011u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.tszl) << 8u32
                    | u32::from(self.imm3) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod urshr_z_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct urshr_z_p_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl urshr_z_p_zi_ {
        #[inline]
        pub fn new(
            tszh: impl Into<::aarchmrs_types::BitValue<2>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            tszl: impl Into<::aarchmrs_types::BitValue<2>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                tszh: tszh.into(),
                U: U.into(),
                Pg: Pg.into(),
                tszl: tszl.into(),
                imm3: imm3.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.tszh) << 22u32
                    | 0b00110u32 << 17u32
                    | u32::from(self.U) << 16u32
                    | 0b100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.tszl) << 8u32
                    | u32::from(self.imm3) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
