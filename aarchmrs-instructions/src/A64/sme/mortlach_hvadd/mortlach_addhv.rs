/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod addha_za_pp_z_32 {
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000011100u32;
    pub const OPCODE: u32 = 0b11000000100100000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "addha_za_pp_z_32";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct addha_za_pp_z_32 {
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl addha_za_pp_z_32 {
        #[inline]
        pub const fn new(
            Pm: ::aarchmrs_types::BitValue<3>,
            Pn: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Pm, Pn, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000010010000u32 << 16u32
                    | self.Pm.into_inner() << 13u32
                    | self.Pn.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b000u32 << 2u32
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
pub mod addha_za_pp_z_64 {
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000011000u32;
    pub const OPCODE: u32 = 0b11000000110100000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "addha_za_pp_z_64";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct addha_za_pp_z_64 {
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl addha_za_pp_z_64 {
        #[inline]
        pub const fn new(
            Pm: ::aarchmrs_types::BitValue<3>,
            Pn: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            ZAda: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { Pm, Pn, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000011010000u32 << 16u32
                    | self.Pm.into_inner() << 13u32
                    | self.Pn.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b00u32 << 3u32
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
pub mod addva_za_pp_z_32 {
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000011100u32;
    pub const OPCODE: u32 = 0b11000000100100010000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "addva_za_pp_z_32";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct addva_za_pp_z_32 {
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl addva_za_pp_z_32 {
        #[inline]
        pub const fn new(
            Pm: ::aarchmrs_types::BitValue<3>,
            Pn: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Pm, Pn, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000010010001u32 << 16u32
                    | self.Pm.into_inner() << 13u32
                    | self.Pn.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b000u32 << 2u32
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
pub mod addva_za_pp_z_64 {
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000011000u32;
    pub const OPCODE: u32 = 0b11000000110100010000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "addva_za_pp_z_64";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct addva_za_pp_z_64 {
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<3>,
    }
    impl addva_za_pp_z_64 {
        #[inline]
        pub const fn new(
            Pm: ::aarchmrs_types::BitValue<3>,
            Pn: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            ZAda: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { Pm, Pn, Zn, ZAda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000011010001u32 << 16u32
                    | self.Pm.into_inner() << 13u32
                    | self.Pn.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b00u32 << 3u32
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
