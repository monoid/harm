/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlall_za_zzi_d {
    pub const OPCODE_MASK: u32 = 0b11111111111100000001000000000100u32;
    pub const OPCODE: u32 = 0b11000001100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "smlall_za_zzi_d";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smlall_za_zzi_d {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub i3h: ::aarchmrs_types::BitValue<1>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i3l: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl smlall_za_zzi_d {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            i3h: ::aarchmrs_types::BitValue<1>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i3l: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            S: ::aarchmrs_types::BitValue<1>,
            off2: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                i3h,
                Rv,
                i3l,
                Zn,
                U,
                S,
                off2,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011000u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | self.i3h.into_inner() << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b0u32 << 12u32
                    | self.i3l.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.U.into_inner() << 4u32
                    | self.S.into_inner() << 3u32
                    | 0b0u32 << 2u32
                    | self.off2.into_inner() << 0u32,
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
pub mod smlsll_za_zzi_d {
    pub const OPCODE_MASK: u32 = 0b11111111111100000001000000000100u32;
    pub const OPCODE: u32 = 0b11000001100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "smlsll_za_zzi_d";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct smlsll_za_zzi_d {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub i3h: ::aarchmrs_types::BitValue<1>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i3l: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl smlsll_za_zzi_d {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            i3h: ::aarchmrs_types::BitValue<1>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i3l: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            S: ::aarchmrs_types::BitValue<1>,
            off2: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                i3h,
                Rv,
                i3l,
                Zn,
                U,
                S,
                off2,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011000u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | self.i3h.into_inner() << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b0u32 << 12u32
                    | self.i3l.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.U.into_inner() << 4u32
                    | self.S.into_inner() << 3u32
                    | 0b0u32 << 2u32
                    | self.off2.into_inner() << 0u32,
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
pub mod umlall_za_zzi_d {
    pub const OPCODE_MASK: u32 = 0b11111111111100000001000000000100u32;
    pub const OPCODE: u32 = 0b11000001100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "umlall_za_zzi_d";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umlall_za_zzi_d {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub i3h: ::aarchmrs_types::BitValue<1>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i3l: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl umlall_za_zzi_d {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            i3h: ::aarchmrs_types::BitValue<1>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i3l: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            S: ::aarchmrs_types::BitValue<1>,
            off2: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                i3h,
                Rv,
                i3l,
                Zn,
                U,
                S,
                off2,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011000u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | self.i3h.into_inner() << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b0u32 << 12u32
                    | self.i3l.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.U.into_inner() << 4u32
                    | self.S.into_inner() << 3u32
                    | 0b0u32 << 2u32
                    | self.off2.into_inner() << 0u32,
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
pub mod umlsll_za_zzi_d {
    pub const OPCODE_MASK: u32 = 0b11111111111100000001000000000100u32;
    pub const OPCODE: u32 = 0b11000001100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "umlsll_za_zzi_d";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct umlsll_za_zzi_d {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub i3h: ::aarchmrs_types::BitValue<1>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i3l: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl umlsll_za_zzi_d {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            i3h: ::aarchmrs_types::BitValue<1>,
            Rv: ::aarchmrs_types::BitValue<2>,
            i3l: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            U: ::aarchmrs_types::BitValue<1>,
            S: ::aarchmrs_types::BitValue<1>,
            off2: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                i3h,
                Rv,
                i3l,
                Zn,
                U,
                S,
                off2,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000011000u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | self.i3h.into_inner() << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b0u32 << 12u32
                    | self.i3l.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.U.into_inner() << 4u32
                    | self.S.into_inner() << 3u32
                    | 0b0u32 << 2u32
                    | self.off2.into_inner() << 0u32,
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
