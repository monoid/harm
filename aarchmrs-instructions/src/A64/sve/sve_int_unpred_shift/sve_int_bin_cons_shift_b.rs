/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod asr_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "asr_z_zi_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct asr_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl asr_z_zi_ {
        #[inline]
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<2>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                U,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b10010u32 << 11u32
                    | self.U.into_inner() << 10u32
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
pub mod lsl_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001111110000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001001110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "lsl_z_zi_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct lsl_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl lsl_z_zi_ {
        #[inline]
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<2>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b100111u32 << 10u32
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
pub mod lsr_z_zi_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001111100000000000u32;
    pub const OPCODE: u32 = 0b00000100001000001001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "lsr_z_zi_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct lsr_z_zi_ {
        pub tszh: ::aarchmrs_types::BitValue<2>,
        pub tszl: ::aarchmrs_types::BitValue<2>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl lsr_z_zi_ {
        #[inline]
        pub const fn new(
            tszh: ::aarchmrs_types::BitValue<2>,
            tszl: ::aarchmrs_types::BitValue<2>,
            imm3: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tszh,
                tszl,
                imm3,
                U,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.tszh.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.tszl.into_inner() << 19u32
                    | self.imm3.into_inner() << 16u32
                    | 0b10010u32 << 11u32
                    | self.U.into_inner() << 10u32
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
