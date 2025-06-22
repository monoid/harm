/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqrshr_z_mz4_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001111100001000000u32;
    pub const OPCODE: u32 = 0b11000001001000001101100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqrshr_z_mz4_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqrshr_z_mz4_ {
        pub tsize: ::aarchmrs_types::BitValue<2>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqrshr_z_mz4_ {
        #[inline]
        pub const fn new(
            tsize: ::aarchmrs_types::BitValue<2>,
            imm5: ::aarchmrs_types::BitValue<5>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tsize,
                imm5,
                N,
                Zn,
                U,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | self.tsize.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b11011u32 << 11u32
                    | self.N.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b0u32 << 6u32
                    | self.U.into_inner() << 5u32
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
pub mod sqrshru_z_mz4_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001111100001100000u32;
    pub const OPCODE: u32 = 0b11000001001000001101100001000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqrshru_z_mz4_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqrshru_z_mz4_ {
        pub tsize: ::aarchmrs_types::BitValue<2>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqrshru_z_mz4_ {
        #[inline]
        pub const fn new(
            tsize: ::aarchmrs_types::BitValue<2>,
            imm5: ::aarchmrs_types::BitValue<5>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tsize,
                imm5,
                N,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | self.tsize.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b11011u32 << 11u32
                    | self.N.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b10u32 << 5u32
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
pub mod sqrshrn_z_mz4_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001111100001000000u32;
    pub const OPCODE: u32 = 0b11000001001000001101100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqrshrn_z_mz4_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqrshrn_z_mz4_ {
        pub tsize: ::aarchmrs_types::BitValue<2>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqrshrn_z_mz4_ {
        #[inline]
        pub const fn new(
            tsize: ::aarchmrs_types::BitValue<2>,
            imm5: ::aarchmrs_types::BitValue<5>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tsize,
                imm5,
                N,
                Zn,
                U,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | self.tsize.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b11011u32 << 11u32
                    | self.N.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b0u32 << 6u32
                    | self.U.into_inner() << 5u32
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
pub mod sqrshrun_z_mz4_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001111100001100000u32;
    pub const OPCODE: u32 = 0b11000001001000001101100001000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "sqrshrun_z_mz4_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sqrshrun_z_mz4_ {
        pub tsize: ::aarchmrs_types::BitValue<2>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl sqrshrun_z_mz4_ {
        #[inline]
        pub const fn new(
            tsize: ::aarchmrs_types::BitValue<2>,
            imm5: ::aarchmrs_types::BitValue<5>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tsize,
                imm5,
                N,
                Zn,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | self.tsize.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b11011u32 << 11u32
                    | self.N.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b10u32 << 5u32
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
pub mod uqrshr_z_mz4_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001111100001000000u32;
    pub const OPCODE: u32 = 0b11000001001000001101100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqrshr_z_mz4_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqrshr_z_mz4_ {
        pub tsize: ::aarchmrs_types::BitValue<2>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uqrshr_z_mz4_ {
        #[inline]
        pub const fn new(
            tsize: ::aarchmrs_types::BitValue<2>,
            imm5: ::aarchmrs_types::BitValue<5>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tsize,
                imm5,
                N,
                Zn,
                U,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | self.tsize.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b11011u32 << 11u32
                    | self.N.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b0u32 << 6u32
                    | self.U.into_inner() << 5u32
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
pub mod uqrshrn_z_mz4_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001111100001000000u32;
    pub const OPCODE: u32 = 0b11000001001000001101100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "uqrshrn_z_mz4_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uqrshrn_z_mz4_ {
        pub tsize: ::aarchmrs_types::BitValue<2>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub Zn: ::aarchmrs_types::BitValue<3>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl uqrshrn_z_mz4_ {
        #[inline]
        pub const fn new(
            tsize: ::aarchmrs_types::BitValue<2>,
            imm5: ::aarchmrs_types::BitValue<5>,
            N: ::aarchmrs_types::BitValue<1>,
            Zn: ::aarchmrs_types::BitValue<3>,
            U: ::aarchmrs_types::BitValue<1>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                tsize,
                imm5,
                N,
                Zn,
                U,
                Zd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | self.tsize.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b11011u32 << 11u32
                    | self.N.into_inner() << 10u32
                    | self.Zn.into_inner() << 7u32
                    | 0b0u32 << 6u32
                    | self.U.into_inner() << 5u32
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
