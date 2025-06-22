/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FCCMP_S_floatccmp {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    pub const OPCODE: u32 = 0b00011110001000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FCCMP_S_floatccmp";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCCMP_S_floatccmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl FCCMP_S_floatccmp {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            cond: ::aarchmrs_types::BitValue<4>,
            Rn: ::aarchmrs_types::BitValue<5>,
            nzcv: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Rm, cond, Rn, nzcv }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.cond.into_inner() << 12u32
                    | 0b01u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.nzcv.into_inner() << 0u32,
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
pub mod FCCMPE_S_floatccmp {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    pub const OPCODE: u32 = 0b00011110001000000000010000010000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FCCMPE_S_floatccmp";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCCMPE_S_floatccmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl FCCMPE_S_floatccmp {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            cond: ::aarchmrs_types::BitValue<4>,
            Rn: ::aarchmrs_types::BitValue<5>,
            nzcv: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Rm, cond, Rn, nzcv }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.cond.into_inner() << 12u32
                    | 0b01u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.nzcv.into_inner() << 0u32,
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
pub mod FCCMP_D_floatccmp {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    pub const OPCODE: u32 = 0b00011110011000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FCCMP_D_floatccmp";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCCMP_D_floatccmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl FCCMP_D_floatccmp {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            cond: ::aarchmrs_types::BitValue<4>,
            Rn: ::aarchmrs_types::BitValue<5>,
            nzcv: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Rm, cond, Rn, nzcv }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.cond.into_inner() << 12u32
                    | 0b01u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.nzcv.into_inner() << 0u32,
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
pub mod FCCMPE_D_floatccmp {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    pub const OPCODE: u32 = 0b00011110011000000000010000010000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FCCMPE_D_floatccmp";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCCMPE_D_floatccmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl FCCMPE_D_floatccmp {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            cond: ::aarchmrs_types::BitValue<4>,
            Rn: ::aarchmrs_types::BitValue<5>,
            nzcv: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Rm, cond, Rn, nzcv }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.cond.into_inner() << 12u32
                    | 0b01u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.nzcv.into_inner() << 0u32,
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
pub mod FCCMP_H_floatccmp {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    pub const OPCODE: u32 = 0b00011110111000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FCCMP_H_floatccmp";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCCMP_H_floatccmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl FCCMP_H_floatccmp {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            cond: ::aarchmrs_types::BitValue<4>,
            Rn: ::aarchmrs_types::BitValue<5>,
            nzcv: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Rm, cond, Rn, nzcv }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110111u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.cond.into_inner() << 12u32
                    | 0b01u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.nzcv.into_inner() << 0u32,
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
pub mod FCCMPE_H_floatccmp {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    pub const OPCODE: u32 = 0b00011110111000000000010000010000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FCCMPE_H_floatccmp";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCCMPE_H_floatccmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl FCCMPE_H_floatccmp {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            cond: ::aarchmrs_types::BitValue<4>,
            Rn: ::aarchmrs_types::BitValue<5>,
            nzcv: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Rm, cond, Rn, nzcv }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110111u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.cond.into_inner() << 12u32
                    | 0b01u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.nzcv.into_inner() << 0u32,
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
