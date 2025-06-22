/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmlallbb_z32_z8z8z8_ {
    pub const OPCODE_MASK: u32 = 0b11111111111000001100110000000000u32;
    pub const OPCODE: u32 = 0b01100100001000001000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "fmlallbb_z32_z8z8z8_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlallbb_z32_z8z8z8_ {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub TT: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl fmlallbb_z32_z8z8z8_ {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            TT: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, TT, Zn, Zda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100001u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.TT.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 0u32,
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
pub mod fmlallbt_z32_z8z8z8_ {
    pub const OPCODE_MASK: u32 = 0b11111111111000001100110000000000u32;
    pub const OPCODE: u32 = 0b01100100001000001000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "fmlallbt_z32_z8z8z8_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlallbt_z32_z8z8z8_ {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub TT: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl fmlallbt_z32_z8z8z8_ {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            TT: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, TT, Zn, Zda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100001u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.TT.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 0u32,
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
pub mod fmlalltb_z32_z8z8z8_ {
    pub const OPCODE_MASK: u32 = 0b11111111111000001100110000000000u32;
    pub const OPCODE: u32 = 0b01100100001000001000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "fmlalltb_z32_z8z8z8_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlalltb_z32_z8z8z8_ {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub TT: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl fmlalltb_z32_z8z8z8_ {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            TT: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, TT, Zn, Zda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100001u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.TT.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 0u32,
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
pub mod fmlalltt_z32_z8z8z8_ {
    pub const OPCODE_MASK: u32 = 0b11111111111000001100110000000000u32;
    pub const OPCODE: u32 = 0b01100100001000001000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "fmlalltt_z32_z8z8z8_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmlalltt_z32_z8z8z8_ {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub TT: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl fmlalltt_z32_z8z8z8_ {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            TT: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, TT, Zn, Zda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100100001u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.TT.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 0u32,
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
