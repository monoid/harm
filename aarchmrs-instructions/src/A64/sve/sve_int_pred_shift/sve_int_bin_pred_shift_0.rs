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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                U,
                Pg,
                tszl,
                imm3,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.tszh.into_inner() << 22u32
                    | 0b00000u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b100u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.tszl.into_inner() << 8u32
                    | self.imm3.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                Pg,
                tszl,
                imm3,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.tszh.into_inner() << 22u32
                    | 0b000011100u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.tszl.into_inner() << 8u32
                    | self.imm3.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                Pg,
                tszl,
                imm3,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.tszh.into_inner() << 22u32
                    | 0b000100100u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.tszl.into_inner() << 8u32
                    | self.imm3.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                U,
                Pg,
                tszl,
                imm3,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.tszh.into_inner() << 22u32
                    | 0b00011u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b100u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.tszl.into_inner() << 8u32
                    | self.imm3.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                U,
                Pg,
                tszl,
                imm3,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.tszh.into_inner() << 22u32
                    | 0b00110u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b100u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.tszl.into_inner() << 8u32
                    | self.imm3.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                Pg,
                tszl,
                imm3,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.tszh.into_inner() << 22u32
                    | 0b001111100u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.tszl.into_inner() << 8u32
                    | self.imm3.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                U,
                Pg,
                tszl,
                imm3,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.tszh.into_inner() << 22u32
                    | 0b00000u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b100u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.tszl.into_inner() << 8u32
                    | self.imm3.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                U,
                Pg,
                tszl,
                imm3,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.tszh.into_inner() << 22u32
                    | 0b00011u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b100u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.tszl.into_inner() << 8u32
                    | self.imm3.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<2>,
            U: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                U,
                Pg,
                tszl,
                imm3,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.tszh.into_inner() << 22u32
                    | 0b00110u32 << 17u32
                    | self.U.into_inner() << 16u32
                    | 0b100u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.tszl.into_inner() << 8u32
                    | self.imm3.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
            )
        }
    }
}
