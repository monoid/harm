/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod add_za_zzw_4x4 {
    pub const OPCODE_MASK: u32 = 0b11111111101000111001110001110000u32;
    pub const OPCODE: u32 = 0b11000001101000010001100000010000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "add_za_zzw_4x4";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct add_za_zzw_4x4 {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl add_za_zzw_4x4 {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            Rv: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                sz,
                Zm,
                Rv,
                Zn,
                S,
                off3,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011u32 << 23u32
                    | self.sz.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Zm.into_inner() << 18u32
                    | 0b010u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b110u32 << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b001u32 << 4u32
                    | self.S.into_inner() << 3u32
                    | self.off3.into_inner() << 0u32,
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
pub mod sub_za_zzw_4x4 {
    pub const OPCODE_MASK: u32 = 0b11111111101000111001110001110000u32;
    pub const OPCODE: u32 = 0b11000001101000010001100000010000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sub_za_zzw_4x4";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sub_za_zzw_4x4 {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl sub_za_zzw_4x4 {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<3>,
            Rv: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                sz,
                Zm,
                Rv,
                Zn,
                S,
                off3,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011u32 << 23u32
                    | self.sz.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Zm.into_inner() << 18u32
                    | 0b010u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b110u32 << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b001u32 << 4u32
                    | self.S.into_inner() << 3u32
                    | self.off3.into_inner() << 0u32,
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
