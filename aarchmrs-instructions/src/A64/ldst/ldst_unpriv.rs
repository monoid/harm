/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STTRB_32_ldst_unpriv {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111000000000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STTRB_32_ldst_unpriv";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STTRB_32_ldst_unpriv {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STTRB_32_ldst_unpriv {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod LDTRB_32_ldst_unpriv {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111000010000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDTRB_32_ldst_unpriv";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTRB_32_ldst_unpriv {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTRB_32_ldst_unpriv {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod LDTRSB_64_ldst_unpriv {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111000100000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDTRSB_64_ldst_unpriv";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTRSB_64_ldst_unpriv {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTRSB_64_ldst_unpriv {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000100u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod LDTRSB_32_ldst_unpriv {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111000110000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDTRSB_32_ldst_unpriv";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTRSB_32_ldst_unpriv {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTRSB_32_ldst_unpriv {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000110u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod STTRH_32_ldst_unpriv {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01111000000000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STTRH_32_ldst_unpriv";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STTRH_32_ldst_unpriv {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STTRH_32_ldst_unpriv {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111000000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod LDTRH_32_ldst_unpriv {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01111000010000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDTRH_32_ldst_unpriv";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTRH_32_ldst_unpriv {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTRH_32_ldst_unpriv {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111000010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod LDTRSH_64_ldst_unpriv {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01111000100000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDTRSH_64_ldst_unpriv";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTRSH_64_ldst_unpriv {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTRSH_64_ldst_unpriv {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111000100u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod LDTRSH_32_ldst_unpriv {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01111000110000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDTRSH_32_ldst_unpriv";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTRSH_32_ldst_unpriv {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTRSH_32_ldst_unpriv {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111000110u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod STTR_32_ldst_unpriv {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b10111000000000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STTR_32_ldst_unpriv";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STTR_32_ldst_unpriv {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STTR_32_ldst_unpriv {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111000000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod LDTR_32_ldst_unpriv {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b10111000010000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDTR_32_ldst_unpriv";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTR_32_ldst_unpriv {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTR_32_ldst_unpriv {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111000010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod LDTRSW_64_ldst_unpriv {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b10111000100000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDTRSW_64_ldst_unpriv";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTRSW_64_ldst_unpriv {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTRSW_64_ldst_unpriv {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111000100u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod STTR_64_ldst_unpriv {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b11111000000000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STTR_64_ldst_unpriv";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STTR_64_ldst_unpriv {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STTR_64_ldst_unpriv {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111000000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod LDTR_64_ldst_unpriv {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b11111000010000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDTR_64_ldst_unpriv";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDTR_64_ldst_unpriv {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDTR_64_ldst_unpriv {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111000010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Rn.into_inner() << 5u32
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
