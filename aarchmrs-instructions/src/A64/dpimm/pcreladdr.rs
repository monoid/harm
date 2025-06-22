/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADR_only_pcreladdr {
    pub const OPCODE_MASK: u32 = 0b10011111000000000000000000000000u32;
    pub const OPCODE: u32 = 0b00010000000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ADR_only_pcreladdr";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADR_only_pcreladdr {
        pub immlo: ::aarchmrs_types::BitValue<2>,
        pub immhi: ::aarchmrs_types::BitValue<19>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADR_only_pcreladdr {
        #[inline]
        pub const fn new(
            immlo: ::aarchmrs_types::BitValue<2>,
            immhi: ::aarchmrs_types::BitValue<19>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { immlo, immhi, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.immlo.into_inner() << 29u32
                    | 0b10000u32 << 24u32
                    | self.immhi.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod ADRP_only_pcreladdr {
    pub const OPCODE_MASK: u32 = 0b10011111000000000000000000000000u32;
    pub const OPCODE: u32 = 0b10010000000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ADRP_only_pcreladdr";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADRP_only_pcreladdr {
        pub immlo: ::aarchmrs_types::BitValue<2>,
        pub immhi: ::aarchmrs_types::BitValue<19>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADRP_only_pcreladdr {
        #[inline]
        pub const fn new(
            immlo: ::aarchmrs_types::BitValue<2>,
            immhi: ::aarchmrs_types::BitValue<19>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { immlo, immhi, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1u32 << 31u32
                    | self.immlo.into_inner() << 29u32
                    | 0b10000u32 << 24u32
                    | self.immhi.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
