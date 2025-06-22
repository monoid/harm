/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod aese_mz_zzi_4x1 {
    pub const OPCODE_MASK: u32 = 0b11111111111001111111110000000011u32;
    pub const OPCODE: u32 = 0b01000101001001101110100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "aese_mz_zzi_4x1";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct aese_mz_zzi_4x1 {
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<3>,
    }
    impl aese_mz_zzi_4x1 {
        #[inline]
        pub const fn new(
            i2: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { i2, Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101001u32 << 21u32
                    | self.i2.into_inner() << 19u32
                    | 0b110111010u32 << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 2u32
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
pub mod aesd_mz_zzi_4x1 {
    pub const OPCODE_MASK: u32 = 0b11111111111001111111110000000011u32;
    pub const OPCODE: u32 = 0b01000101001001101110110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "aesd_mz_zzi_4x1";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct aesd_mz_zzi_4x1 {
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<3>,
    }
    impl aesd_mz_zzi_4x1 {
        #[inline]
        pub const fn new(
            i2: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { i2, Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101001u32 << 21u32
                    | self.i2.into_inner() << 19u32
                    | 0b110111011u32 << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 2u32
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
pub mod aesemc_mz_zzi_4x1 {
    pub const OPCODE_MASK: u32 = 0b11111111111001111111110000000011u32;
    pub const OPCODE: u32 = 0b01000101001001111110100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "aesemc_mz_zzi_4x1";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct aesemc_mz_zzi_4x1 {
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<3>,
    }
    impl aesemc_mz_zzi_4x1 {
        #[inline]
        pub const fn new(
            i2: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { i2, Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101001u32 << 21u32
                    | self.i2.into_inner() << 19u32
                    | 0b111111010u32 << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 2u32
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
pub mod aesdimc_mz_zzi_4x1 {
    pub const OPCODE_MASK: u32 = 0b11111111111001111111110000000011u32;
    pub const OPCODE: u32 = 0b01000101001001111110110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "aesdimc_mz_zzi_4x1";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct aesdimc_mz_zzi_4x1 {
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<3>,
    }
    impl aesdimc_mz_zzi_4x1 {
        #[inline]
        pub const fn new(
            i2: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { i2, Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101001u32 << 21u32
                    | self.i2.into_inner() << 19u32
                    | 0b111111011u32 << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 2u32
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
