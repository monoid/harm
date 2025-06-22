/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod pext_pn_rr_ {
    pub const OPCODE_MASK: u32 = 0b11111111001111111111110000010000u32;
    pub const OPCODE: u32 = 0b00100101001000000111000000010000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "pext_pn_rr_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct pext_pn_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm2: ::aarchmrs_types::BitValue<2>,
        pub PNn: ::aarchmrs_types::BitValue<3>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl pext_pn_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm2: ::aarchmrs_types::BitValue<2>,
            PNn: ::aarchmrs_types::BitValue<3>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                size,
                imm2,
                PNn,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b100000011100u32 << 10u32
                    | self.imm2.into_inner() << 8u32
                    | self.PNn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
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
pub mod pext_pp_rr_ {
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000010000u32;
    pub const OPCODE: u32 = 0b00100101001000000111010000010000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "pext_pp_rr_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct pext_pp_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub PNn: ::aarchmrs_types::BitValue<3>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl pext_pp_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            i1: ::aarchmrs_types::BitValue<1>,
            PNn: ::aarchmrs_types::BitValue<3>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { size, i1, PNn, Pd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1000000111010u32 << 9u32
                    | self.i1.into_inner() << 8u32
                    | self.PNn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.Pd.into_inner() << 0u32,
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
