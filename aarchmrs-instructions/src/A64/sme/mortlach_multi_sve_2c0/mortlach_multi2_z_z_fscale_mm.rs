/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fscale_mz_zzw_2x2 {
    pub const OPCODE_MASK: u32 = 0b11111111001000011111111111100001u32;
    pub const OPCODE: u32 = 0b11000001001000001011000110000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "fscale_mz_zzw_2x2";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fscale_mz_zzw_2x2 {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl fscale_mz_zzw_2x2 {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<4>,
            Zdn: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { size, Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Zm.into_inner() << 17u32
                    | 0b010110001100u32 << 5u32
                    | self.Zdn.into_inner() << 1u32
                    | 0b0u32 << 0u32,
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
pub mod bfscale_mz_zzw_2x2 {
    pub const OPCODE_MASK: u32 = 0b11111111111000011111111111100001u32;
    pub const OPCODE: u32 = 0b11000001001000001011000110000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "bfscale_mz_zzw_2x2";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfscale_mz_zzw_2x2 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<4>,
    }
    impl bfscale_mz_zzw_2x2 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Zdn: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Zm, Zdn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001001u32 << 21u32
                    | self.Zm.into_inner() << 17u32
                    | 0b010110001100u32 << 5u32
                    | self.Zdn.into_inner() << 1u32
                    | 0b0u32 << 0u32,
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
