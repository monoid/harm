/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ctermeq_rr_ {
    pub const OPCODE_MASK: u32 = 0b11111111101000001111110000011111u32;
    pub const OPCODE: u32 = 0b00100101101000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ctermeq_rr_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ctermeq_rr_ {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl ctermeq_rr_ {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rm, Rn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001011u32 << 23u32
                    | self.sz.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b00000u32 << 0u32,
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
pub mod ctermne_rr_ {
    pub const OPCODE_MASK: u32 = 0b11111111101000001111110000011111u32;
    pub const OPCODE: u32 = 0b00100101101000000010000000010000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ctermne_rr_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ctermne_rr_ {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl ctermne_rr_ {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rm, Rn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001011u32 << 23u32
                    | self.sz.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b10000u32 << 0u32,
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
