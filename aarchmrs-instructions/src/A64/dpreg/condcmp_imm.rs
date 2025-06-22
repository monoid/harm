/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CCMN_32_condcmp_imm {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    pub const OPCODE: u32 = 0b00111010010000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CCMN_32_condcmp_imm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CCMN_32_condcmp_imm {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl CCMN_32_condcmp_imm {
        #[inline]
        pub const fn new(
            imm5: ::aarchmrs_types::BitValue<5>,
            cond: ::aarchmrs_types::BitValue<4>,
            Rn: ::aarchmrs_types::BitValue<5>,
            nzcv: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm5,
                cond,
                Rn,
                nzcv,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111010010u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | self.cond.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.nzcv.into_inner() << 0u32,
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
pub mod CCMP_32_condcmp_imm {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    pub const OPCODE: u32 = 0b01111010010000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CCMP_32_condcmp_imm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CCMP_32_condcmp_imm {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl CCMP_32_condcmp_imm {
        #[inline]
        pub const fn new(
            imm5: ::aarchmrs_types::BitValue<5>,
            cond: ::aarchmrs_types::BitValue<4>,
            Rn: ::aarchmrs_types::BitValue<5>,
            nzcv: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm5,
                cond,
                Rn,
                nzcv,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111010010u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | self.cond.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.nzcv.into_inner() << 0u32,
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
pub mod CCMN_64_condcmp_imm {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    pub const OPCODE: u32 = 0b10111010010000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CCMN_64_condcmp_imm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CCMN_64_condcmp_imm {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl CCMN_64_condcmp_imm {
        #[inline]
        pub const fn new(
            imm5: ::aarchmrs_types::BitValue<5>,
            cond: ::aarchmrs_types::BitValue<4>,
            Rn: ::aarchmrs_types::BitValue<5>,
            nzcv: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm5,
                cond,
                Rn,
                nzcv,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111010010u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | self.cond.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.nzcv.into_inner() << 0u32,
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
pub mod CCMP_64_condcmp_imm {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000010000u32;
    pub const OPCODE: u32 = 0b11111010010000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CCMP_64_condcmp_imm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CCMP_64_condcmp_imm {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl CCMP_64_condcmp_imm {
        #[inline]
        pub const fn new(
            imm5: ::aarchmrs_types::BitValue<5>,
            cond: ::aarchmrs_types::BitValue<4>,
            Rn: ::aarchmrs_types::BitValue<5>,
            nzcv: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm5,
                cond,
                Rn,
                nzcv,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111010010u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | self.cond.into_inner() << 12u32
                    | 0b10u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.nzcv.into_inner() << 0u32,
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
