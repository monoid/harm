/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CLREX_BN_barriers {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000011111111u32;
    pub const OPCODE: u32 = 0b11010101000000110011000001011111u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CLREX_BN_barriers";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CLREX_BN_barriers {
        pub CRm: ::aarchmrs_types::BitValue<4>,
    }
    impl CLREX_BN_barriers {
        #[inline]
        pub const fn new(CRm: ::aarchmrs_types::BitValue<4>) -> Self {
            Self { CRm }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110011u32 << 12u32
                    | self.CRm.into_inner() << 8u32
                    | 0b01011111u32 << 0u32,
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
pub mod DSB_BO_barriers {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000010011111u32;
    pub const OPCODE: u32 = 0b11010101000000110011000010011111u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "DSB_BO_barriers";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct DSB_BO_barriers {
        pub CRm: ::aarchmrs_types::BitValue<4>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl DSB_BO_barriers {
        #[inline]
        pub const fn new(
            CRm: ::aarchmrs_types::BitValue<4>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { CRm, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110011u32 << 12u32
                    | self.CRm.into_inner() << 8u32
                    | 0b1u32 << 7u32
                    | self.opc.into_inner() << 5u32
                    | 0b11111u32 << 0u32,
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
pub mod DMB_BO_barriers {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000010011111u32;
    pub const OPCODE: u32 = 0b11010101000000110011000010011111u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "DMB_BO_barriers";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct DMB_BO_barriers {
        pub CRm: ::aarchmrs_types::BitValue<4>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl DMB_BO_barriers {
        #[inline]
        pub const fn new(
            CRm: ::aarchmrs_types::BitValue<4>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { CRm, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110011u32 << 12u32
                    | self.CRm.into_inner() << 8u32
                    | 0b1u32 << 7u32
                    | self.opc.into_inner() << 5u32
                    | 0b11111u32 << 0u32,
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
pub mod ISB_BI_barriers {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000010011111u32;
    pub const OPCODE: u32 = 0b11010101000000110011000010011111u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ISB_BI_barriers";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ISB_BI_barriers {
        pub CRm: ::aarchmrs_types::BitValue<4>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl ISB_BI_barriers {
        #[inline]
        pub const fn new(
            CRm: ::aarchmrs_types::BitValue<4>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { CRm, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110011u32 << 12u32
                    | self.CRm.into_inner() << 8u32
                    | 0b1u32 << 7u32
                    | self.opc.into_inner() << 5u32
                    | 0b11111u32 << 0u32,
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
pub mod SB_only_barriers {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111110011111u32;
    pub const OPCODE: u32 = 0b11010101000000110011000010011111u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111100000000u32;
    pub const NAME: &str = "SB_only_barriers";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SB_only_barriers {
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl SB_only_barriers {
        #[inline]
        pub const fn new(opc: ::aarchmrs_types::BitValue<2>) -> Self {
            Self { opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101010100000011001100001u32 << 7u32
                    | self.opc.into_inner() << 5u32
                    | 0b11111u32 << 0u32,
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
pub mod DSB_BOn_barriers {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111001111111111u32;
    pub const OPCODE: u32 = 0b11010101000000110011001000111111u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "DSB_BOn_barriers";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct DSB_BOn_barriers {
        pub imm2: ::aarchmrs_types::BitValue<2>,
    }
    impl DSB_BOn_barriers {
        #[inline]
        pub const fn new(imm2: ::aarchmrs_types::BitValue<2>) -> Self {
            Self { imm2 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110011u32 << 12u32
                    | self.imm2.into_inner() << 10u32
                    | 0b1000111111u32 << 0u32,
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
pub mod TCOMMIT_only_barriers {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    pub const OPCODE: u32 = 0b11010101000000110011000001111111u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "TCOMMIT_only_barriers";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TCOMMIT_only_barriers {}
    impl TCOMMIT_only_barriers {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010101000000110011000001111111u32 << 0u32,
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
