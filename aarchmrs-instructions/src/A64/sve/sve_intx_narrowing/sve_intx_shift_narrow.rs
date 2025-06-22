/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqshrunb_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111101000001111100000000000u32;
    pub const OPCODE: u32 = 0b01000101001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqshrunb_z_zi_";
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<1>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b00000u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod sqrshrunb_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111101000001111100000000000u32;
    pub const OPCODE: u32 = 0b01000101001000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqrshrunb_z_zi_";
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<1>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b00001u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod shrnb_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111101000001111100000000000u32;
    pub const OPCODE: u32 = 0b01000101001000000001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "shrnb_z_zi_";
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<1>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b00010u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod rshrnb_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111101000001111100000000000u32;
    pub const OPCODE: u32 = 0b01000101001000000001100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "rshrnb_z_zi_";
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<1>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b00011u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod sqshrnb_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111101000001110100000000000u32;
    pub const OPCODE: u32 = 0b01000101001000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqshrnb_z_zi_";
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<1>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                U,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.U.into_inner() << 12u32
                    | 0b0u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod sqrshrnb_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111101000001110100000000000u32;
    pub const OPCODE: u32 = 0b01000101001000000010100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqrshrnb_z_zi_";
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<1>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                U,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.U.into_inner() << 12u32
                    | 0b1u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod sqshrunt_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111101000001111100000000000u32;
    pub const OPCODE: u32 = 0b01000101001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqshrunt_z_zi_";
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<1>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b00000u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod sqrshrunt_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111101000001111100000000000u32;
    pub const OPCODE: u32 = 0b01000101001000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqrshrunt_z_zi_";
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<1>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b00001u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod shrnt_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111101000001111100000000000u32;
    pub const OPCODE: u32 = 0b01000101001000000001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "shrnt_z_zi_";
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<1>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b00010u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod rshrnt_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111101000001111100000000000u32;
    pub const OPCODE: u32 = 0b01000101001000000001100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "rshrnt_z_zi_";
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<1>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b00011u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod sqshrnt_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111101000001110100000000000u32;
    pub const OPCODE: u32 = 0b01000101001000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqshrnt_z_zi_";
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<1>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                U,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.U.into_inner() << 12u32
                    | 0b0u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod sqrshrnt_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111101000001110100000000000u32;
    pub const OPCODE: u32 = 0b01000101001000000010100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqrshrnt_z_zi_";
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<1>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                U,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.U.into_inner() << 12u32
                    | 0b1u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod uqshrnb_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111101000001110100000000000u32;
    pub const OPCODE: u32 = 0b01000101001000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqshrnb_z_zi_";
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<1>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                U,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.U.into_inner() << 12u32
                    | 0b0u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod uqrshrnb_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111101000001110100000000000u32;
    pub const OPCODE: u32 = 0b01000101001000000010100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqrshrnb_z_zi_";
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<1>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                U,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.U.into_inner() << 12u32
                    | 0b1u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod uqshrnt_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111101000001110100000000000u32;
    pub const OPCODE: u32 = 0b01000101001000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqshrnt_z_zi_";
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<1>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                U,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.U.into_inner() << 12u32
                    | 0b0u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod uqrshrnt_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111101000001110100000000000u32;
    pub const OPCODE: u32 = 0b01000101001000000010100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqrshrnt_z_zi_";
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
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<1>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            T: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                U,
                T,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010001010u32 << 23u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.U.into_inner() << 12u32
                    | 0b1u32 << 11u32
                    | self.T.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
