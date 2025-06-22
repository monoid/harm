/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mova_mz4_za_b1 {
    pub const OPCODE_MASK: u32 = 0b11111111111111110001111110000011u32;
    pub const OPCODE: u32 = 0b11000000000001100000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "mova_mz4_za_b1";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_mz4_za_b1 {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub off2: ::aarchmrs_types::BitValue<2>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl mova_mz4_za_b1 {
        #[inline]
        pub const fn new(
            V: ::aarchmrs_types::BitValue<1>,
            Rs: ::aarchmrs_types::BitValue<2>,
            off2: ::aarchmrs_types::BitValue<2>,
            Zd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { V, Rs, off2, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000000000110u32 << 16u32
                    | self.V.into_inner() << 15u32
                    | self.Rs.into_inner() << 13u32
                    | 0b001000u32 << 7u32
                    | self.off2.into_inner() << 5u32
                    | self.Zd.into_inner() << 2u32
                    | 0b00u32 << 0u32,
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
pub mod mova_mz4_za_h1 {
    pub const OPCODE_MASK: u32 = 0b11111111111111110001111110000011u32;
    pub const OPCODE: u32 = 0b11000000010001100000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "mova_mz4_za_h1";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_mz4_za_h1 {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub ZAn: ::aarchmrs_types::BitValue<1>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl mova_mz4_za_h1 {
        #[inline]
        pub const fn new(
            V: ::aarchmrs_types::BitValue<1>,
            Rs: ::aarchmrs_types::BitValue<2>,
            ZAn: ::aarchmrs_types::BitValue<1>,
            o1: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { V, Rs, ZAn, o1, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000001000110u32 << 16u32
                    | self.V.into_inner() << 15u32
                    | self.Rs.into_inner() << 13u32
                    | 0b001000u32 << 7u32
                    | self.ZAn.into_inner() << 6u32
                    | self.o1.into_inner() << 5u32
                    | self.Zd.into_inner() << 2u32
                    | 0b00u32 << 0u32,
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
pub mod mova_mz4_za_w1 {
    pub const OPCODE_MASK: u32 = 0b11111111111111110001111110000011u32;
    pub const OPCODE: u32 = 0b11000000100001100000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "mova_mz4_za_w1";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_mz4_za_w1 {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub ZAn: ::aarchmrs_types::BitValue<2>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl mova_mz4_za_w1 {
        #[inline]
        pub const fn new(
            V: ::aarchmrs_types::BitValue<1>,
            Rs: ::aarchmrs_types::BitValue<2>,
            ZAn: ::aarchmrs_types::BitValue<2>,
            Zd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { V, Rs, ZAn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000010000110u32 << 16u32
                    | self.V.into_inner() << 15u32
                    | self.Rs.into_inner() << 13u32
                    | 0b001000u32 << 7u32
                    | self.ZAn.into_inner() << 5u32
                    | self.Zd.into_inner() << 2u32
                    | 0b00u32 << 0u32,
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
pub mod mova_mz4_za_d1 {
    pub const OPCODE_MASK: u32 = 0b11111111111111110001111100000011u32;
    pub const OPCODE: u32 = 0b11000000110001100000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "mova_mz4_za_d1";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mova_mz4_za_d1 {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub ZAn: ::aarchmrs_types::BitValue<3>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl mova_mz4_za_d1 {
        #[inline]
        pub const fn new(
            V: ::aarchmrs_types::BitValue<1>,
            Rs: ::aarchmrs_types::BitValue<2>,
            ZAn: ::aarchmrs_types::BitValue<3>,
            Zd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { V, Rs, ZAn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000011000110u32 << 16u32
                    | self.V.into_inner() << 15u32
                    | self.Rs.into_inner() << 13u32
                    | 0b00100u32 << 8u32
                    | self.ZAn.into_inner() << 5u32
                    | self.Zd.into_inner() << 2u32
                    | 0b00u32 << 0u32,
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
