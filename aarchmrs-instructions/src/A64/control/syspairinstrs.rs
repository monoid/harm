/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SYSP_CR_syspairinstrs {
    pub const OPCODE_MASK: u32 = 0b11111111111110000000000000000000u32;
    pub const OPCODE: u32 = 0b11010101010010000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SYSP_CR_syspairinstrs";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SYSP_CR_syspairinstrs {
        pub op1: ::aarchmrs_types::BitValue<3>,
        pub CRn: ::aarchmrs_types::BitValue<4>,
        pub CRm: ::aarchmrs_types::BitValue<4>,
        pub op2: ::aarchmrs_types::BitValue<3>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl SYSP_CR_syspairinstrs {
        #[inline]
        pub const fn new(
            op1: ::aarchmrs_types::BitValue<3>,
            CRn: ::aarchmrs_types::BitValue<4>,
            CRm: ::aarchmrs_types::BitValue<4>,
            op2: ::aarchmrs_types::BitValue<3>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                op1,
                CRn,
                CRm,
                op2,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101010101001u32 << 19u32
                    | self.op1.into_inner() << 16u32
                    | self.CRn.into_inner() << 12u32
                    | self.CRm.into_inner() << 8u32
                    | self.op2.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
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
