/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod AND_32_log_imm {
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    pub const OPCODE: u32 = 0b00010010000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "AND_32_log_imm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AND_32_log_imm {
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AND_32_log_imm {
        #[inline]
        pub const fn new(
            immr: ::aarchmrs_types::BitValue<6>,
            imms: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { immr, imms, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0001001000u32 << 22u32
                    | self.immr.into_inner() << 16u32
                    | self.imms.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod ORR_32_log_imm {
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    pub const OPCODE: u32 = 0b00110010000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ORR_32_log_imm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ORR_32_log_imm {
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ORR_32_log_imm {
        #[inline]
        pub const fn new(
            immr: ::aarchmrs_types::BitValue<6>,
            imms: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { immr, imms, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0011001000u32 << 22u32
                    | self.immr.into_inner() << 16u32
                    | self.imms.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod EOR_32_log_imm {
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    pub const OPCODE: u32 = 0b01010010000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "EOR_32_log_imm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct EOR_32_log_imm {
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl EOR_32_log_imm {
        #[inline]
        pub const fn new(
            immr: ::aarchmrs_types::BitValue<6>,
            imms: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { immr, imms, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101001000u32 << 22u32
                    | self.immr.into_inner() << 16u32
                    | self.imms.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod ANDS_32S_log_imm {
    pub const OPCODE_MASK: u32 = 0b11111111110000000000000000000000u32;
    pub const OPCODE: u32 = 0b01110010000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ANDS_32S_log_imm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ANDS_32S_log_imm {
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ANDS_32S_log_imm {
        #[inline]
        pub const fn new(
            immr: ::aarchmrs_types::BitValue<6>,
            imms: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { immr, imms, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0111001000u32 << 22u32
                    | self.immr.into_inner() << 16u32
                    | self.imms.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod AND_64_log_imm {
    pub const OPCODE_MASK: u32 = 0b11111111100000000000000000000000u32;
    pub const OPCODE: u32 = 0b10010010000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "AND_64_log_imm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AND_64_log_imm {
        pub N: ::aarchmrs_types::BitValue<1>,
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AND_64_log_imm {
        #[inline]
        pub const fn new(
            N: ::aarchmrs_types::BitValue<1>,
            immr: ::aarchmrs_types::BitValue<6>,
            imms: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                N,
                immr,
                imms,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b100100100u32 << 23u32
                    | self.N.into_inner() << 22u32
                    | self.immr.into_inner() << 16u32
                    | self.imms.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod ORR_64_log_imm {
    pub const OPCODE_MASK: u32 = 0b11111111100000000000000000000000u32;
    pub const OPCODE: u32 = 0b10110010000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ORR_64_log_imm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ORR_64_log_imm {
        pub N: ::aarchmrs_types::BitValue<1>,
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ORR_64_log_imm {
        #[inline]
        pub const fn new(
            N: ::aarchmrs_types::BitValue<1>,
            immr: ::aarchmrs_types::BitValue<6>,
            imms: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                N,
                immr,
                imms,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101100100u32 << 23u32
                    | self.N.into_inner() << 22u32
                    | self.immr.into_inner() << 16u32
                    | self.imms.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod EOR_64_log_imm {
    pub const OPCODE_MASK: u32 = 0b11111111100000000000000000000000u32;
    pub const OPCODE: u32 = 0b11010010000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "EOR_64_log_imm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct EOR_64_log_imm {
        pub N: ::aarchmrs_types::BitValue<1>,
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl EOR_64_log_imm {
        #[inline]
        pub const fn new(
            N: ::aarchmrs_types::BitValue<1>,
            immr: ::aarchmrs_types::BitValue<6>,
            imms: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                N,
                immr,
                imms,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110100100u32 << 23u32
                    | self.N.into_inner() << 22u32
                    | self.immr.into_inner() << 16u32
                    | self.imms.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
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
pub mod ANDS_64S_log_imm {
    pub const OPCODE_MASK: u32 = 0b11111111100000000000000000000000u32;
    pub const OPCODE: u32 = 0b11110010000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ANDS_64S_log_imm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ANDS_64S_log_imm {
        pub N: ::aarchmrs_types::BitValue<1>,
        pub immr: ::aarchmrs_types::BitValue<6>,
        pub imms: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ANDS_64S_log_imm {
        #[inline]
        pub const fn new(
            N: ::aarchmrs_types::BitValue<1>,
            immr: ::aarchmrs_types::BitValue<6>,
            imms: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                N,
                immr,
                imms,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b111100100u32 << 23u32
                    | self.N.into_inner() << 22u32
                    | self.immr.into_inner() << 16u32
                    | self.imms.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
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
