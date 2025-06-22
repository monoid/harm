/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod zip1_z_zz_q {
    pub const OPCODE_MASK: u32 = 0b11111111111000001111100000000000u32;
    pub const OPCODE: u32 = 0b00000101101000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "zip1_z_zz_q";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct zip1_z_zz_q {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl zip1_z_zz_q {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            H: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, H, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101101u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b00000u32 << 11u32
                    | self.H.into_inner() << 10u32
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
pub mod uzp1_z_zz_q {
    pub const OPCODE_MASK: u32 = 0b11111111111000001111100000000000u32;
    pub const OPCODE: u32 = 0b00000101101000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uzp1_z_zz_q";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uzp1_z_zz_q {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uzp1_z_zz_q {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            H: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, H, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101101u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b00001u32 << 11u32
                    | self.H.into_inner() << 10u32
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
pub mod trn1_z_zz_q {
    pub const OPCODE_MASK: u32 = 0b11111111111000001111100000000000u32;
    pub const OPCODE: u32 = 0b00000101101000000001100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "trn1_z_zz_q";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct trn1_z_zz_q {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl trn1_z_zz_q {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            H: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, H, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101101u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b00011u32 << 11u32
                    | self.H.into_inner() << 10u32
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
pub mod zip2_z_zz_q {
    pub const OPCODE_MASK: u32 = 0b11111111111000001111100000000000u32;
    pub const OPCODE: u32 = 0b00000101101000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "zip2_z_zz_q";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct zip2_z_zz_q {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl zip2_z_zz_q {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            H: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, H, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101101u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b00000u32 << 11u32
                    | self.H.into_inner() << 10u32
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
pub mod uzp2_z_zz_q {
    pub const OPCODE_MASK: u32 = 0b11111111111000001111100000000000u32;
    pub const OPCODE: u32 = 0b00000101101000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uzp2_z_zz_q";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uzp2_z_zz_q {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uzp2_z_zz_q {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            H: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, H, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101101u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b00001u32 << 11u32
                    | self.H.into_inner() << 10u32
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
pub mod trn2_z_zz_q {
    pub const OPCODE_MASK: u32 = 0b11111111111000001111100000000000u32;
    pub const OPCODE: u32 = 0b00000101101000000001100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "trn2_z_zz_q";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct trn2_z_zz_q {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl trn2_z_zz_q {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            H: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, H, Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101101u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b00011u32 << 11u32
                    | self.H.into_inner() << 10u32
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
