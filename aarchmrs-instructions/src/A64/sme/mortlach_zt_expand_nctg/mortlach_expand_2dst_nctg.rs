/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod luti2_mz2_ztz_8 {
    pub const OPCODE_MASK: u32 = 0b11111111111111000100110000001000u32;
    pub const OPCODE: u32 = 0b11000000100111000100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "luti2_mz2_ztz_8";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct luti2_mz2_ztz_8 {
        pub i3: ::aarchmrs_types::BitValue<3>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub D: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl luti2_mz2_ztz_8 {
        #[inline]
        pub const fn new(
            i3: ::aarchmrs_types::BitValue<3>,
            size: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            D: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                i3,
                size,
                Zn,
                D,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000100111u32 << 18u32
                    | self.i3.into_inner() << 15u32
                    | 0b1u32 << 14u32
                    | self.size.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.D.into_inner() << 4u32
                    | 0b0u32 << 3u32
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
pub mod luti4_mz2_ztz_8 {
    pub const OPCODE_MASK: u32 = 0b11111111111111100100110000001000u32;
    pub const OPCODE: u32 = 0b11000000100110100100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "luti4_mz2_ztz_8";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct luti4_mz2_ztz_8 {
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub D: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<3>,
    }
    impl luti4_mz2_ztz_8 {
        #[inline]
        pub const fn new(
            i2: ::aarchmrs_types::BitValue<2>,
            size: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            D: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                i2,
                size,
                Zn,
                D,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000001001101u32 << 17u32
                    | self.i2.into_inner() << 15u32
                    | 0b1u32 << 14u32
                    | self.size.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.D.into_inner() << 4u32
                    | 0b0u32 << 3u32
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
