/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmop4a_za16_z8z8_b1x1 {
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111110u32;
    pub const OPCODE: u32 = 0b10000000001000000000000000001000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "fmop4a_za16_z8z8_b1x1";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4a_za16_z8z8_b1x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<1>,
    }
    impl fmop4a_za16_z8z8_b1x1 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000001u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0000000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b00100u32 << 1u32
                    | self.ZAda.into_inner() << 0u32,
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
pub mod fmop4a_za16_z8z8_b1x2 {
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111110u32;
    pub const OPCODE: u32 = 0b10000000001000000000000000001000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "fmop4a_za16_z8z8_b1x2";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4a_za16_z8z8_b1x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<1>,
    }
    impl fmop4a_za16_z8z8_b1x2 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000001u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0000000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b00100u32 << 1u32
                    | self.ZAda.into_inner() << 0u32,
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
pub mod fmop4a_za16_z8z8_b2x1 {
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111110u32;
    pub const OPCODE: u32 = 0b10000000001000000000000000001000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "fmop4a_za16_z8z8_b2x1";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4a_za16_z8z8_b2x1 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<1>,
    }
    impl fmop4a_za16_z8z8_b2x1 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000001u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0000000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b00100u32 << 1u32
                    | self.ZAda.into_inner() << 0u32,
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
pub mod fmop4a_za16_z8z8_b2x2 {
    pub const OPCODE_MASK: u32 = 0b11111111111000011111110000111110u32;
    pub const OPCODE: u32 = 0b10000000001000000000000000001000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "fmop4a_za16_z8z8_b2x2";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmop4a_za16_z8z8_b2x2 {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub ZAda: ::aarchmrs_types::BitValue<1>,
    }
    impl fmop4a_za16_z8z8_b2x2 {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            ZAda: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self { M, Zm, N, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000001u32 << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Zm.into_inner() << 17u32
                    | 0b0000000u32 << 10u32
                    | self.N.into_inner() << 9u32
                    | self.Zn.into_inner() << 6u32
                    | 0b00100u32 << 1u32
                    | self.ZAda.into_inner() << 0u32,
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
