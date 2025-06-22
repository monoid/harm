/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod AND_32_log_shift {
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    pub const OPCODE: u32 = 0b00001010000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "AND_32_log_shift";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AND_32_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AND_32_log_shift {
        #[inline]
        pub const fn new(
            shift: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            imm6: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                shift,
                Rm,
                imm6,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00001010u32 << 24u32
                    | self.shift.into_inner() << 22u32
                    | 0b0u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.imm6.into_inner() << 10u32
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
pub mod BIC_32_log_shift {
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    pub const OPCODE: u32 = 0b00001010001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BIC_32_log_shift";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BIC_32_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl BIC_32_log_shift {
        #[inline]
        pub const fn new(
            shift: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            imm6: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                shift,
                Rm,
                imm6,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00001010u32 << 24u32
                    | self.shift.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.imm6.into_inner() << 10u32
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
pub mod ORR_32_log_shift {
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    pub const OPCODE: u32 = 0b00101010000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ORR_32_log_shift";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ORR_32_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ORR_32_log_shift {
        #[inline]
        pub const fn new(
            shift: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            imm6: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                shift,
                Rm,
                imm6,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00101010u32 << 24u32
                    | self.shift.into_inner() << 22u32
                    | 0b0u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.imm6.into_inner() << 10u32
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
pub mod ORN_32_log_shift {
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    pub const OPCODE: u32 = 0b00101010001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ORN_32_log_shift";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ORN_32_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ORN_32_log_shift {
        #[inline]
        pub const fn new(
            shift: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            imm6: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                shift,
                Rm,
                imm6,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00101010u32 << 24u32
                    | self.shift.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.imm6.into_inner() << 10u32
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
pub mod EOR_32_log_shift {
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    pub const OPCODE: u32 = 0b01001010000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "EOR_32_log_shift";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct EOR_32_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl EOR_32_log_shift {
        #[inline]
        pub const fn new(
            shift: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            imm6: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                shift,
                Rm,
                imm6,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01001010u32 << 24u32
                    | self.shift.into_inner() << 22u32
                    | 0b0u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.imm6.into_inner() << 10u32
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
pub mod EON_32_log_shift {
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    pub const OPCODE: u32 = 0b01001010001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "EON_32_log_shift";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct EON_32_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl EON_32_log_shift {
        #[inline]
        pub const fn new(
            shift: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            imm6: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                shift,
                Rm,
                imm6,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01001010u32 << 24u32
                    | self.shift.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.imm6.into_inner() << 10u32
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
pub mod ANDS_32_log_shift {
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    pub const OPCODE: u32 = 0b01101010000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ANDS_32_log_shift";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ANDS_32_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ANDS_32_log_shift {
        #[inline]
        pub const fn new(
            shift: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            imm6: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                shift,
                Rm,
                imm6,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01101010u32 << 24u32
                    | self.shift.into_inner() << 22u32
                    | 0b0u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.imm6.into_inner() << 10u32
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
pub mod BICS_32_log_shift {
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    pub const OPCODE: u32 = 0b01101010001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BICS_32_log_shift";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BICS_32_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl BICS_32_log_shift {
        #[inline]
        pub const fn new(
            shift: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            imm6: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                shift,
                Rm,
                imm6,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01101010u32 << 24u32
                    | self.shift.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.imm6.into_inner() << 10u32
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
pub mod AND_64_log_shift {
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    pub const OPCODE: u32 = 0b10001010000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "AND_64_log_shift";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AND_64_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AND_64_log_shift {
        #[inline]
        pub const fn new(
            shift: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            imm6: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                shift,
                Rm,
                imm6,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10001010u32 << 24u32
                    | self.shift.into_inner() << 22u32
                    | 0b0u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.imm6.into_inner() << 10u32
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
pub mod BIC_64_log_shift {
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    pub const OPCODE: u32 = 0b10001010001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BIC_64_log_shift";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BIC_64_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl BIC_64_log_shift {
        #[inline]
        pub const fn new(
            shift: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            imm6: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                shift,
                Rm,
                imm6,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10001010u32 << 24u32
                    | self.shift.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.imm6.into_inner() << 10u32
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
pub mod ORR_64_log_shift {
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    pub const OPCODE: u32 = 0b10101010000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ORR_64_log_shift";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ORR_64_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ORR_64_log_shift {
        #[inline]
        pub const fn new(
            shift: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            imm6: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                shift,
                Rm,
                imm6,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10101010u32 << 24u32
                    | self.shift.into_inner() << 22u32
                    | 0b0u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.imm6.into_inner() << 10u32
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
pub mod ORN_64_log_shift {
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    pub const OPCODE: u32 = 0b10101010001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ORN_64_log_shift";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ORN_64_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ORN_64_log_shift {
        #[inline]
        pub const fn new(
            shift: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            imm6: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                shift,
                Rm,
                imm6,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10101010u32 << 24u32
                    | self.shift.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.imm6.into_inner() << 10u32
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
pub mod EOR_64_log_shift {
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    pub const OPCODE: u32 = 0b11001010000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "EOR_64_log_shift";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct EOR_64_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl EOR_64_log_shift {
        #[inline]
        pub const fn new(
            shift: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            imm6: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                shift,
                Rm,
                imm6,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001010u32 << 24u32
                    | self.shift.into_inner() << 22u32
                    | 0b0u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.imm6.into_inner() << 10u32
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
pub mod EON_64_log_shift {
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    pub const OPCODE: u32 = 0b11001010001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "EON_64_log_shift";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct EON_64_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl EON_64_log_shift {
        #[inline]
        pub const fn new(
            shift: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            imm6: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                shift,
                Rm,
                imm6,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001010u32 << 24u32
                    | self.shift.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.imm6.into_inner() << 10u32
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
pub mod ANDS_64_log_shift {
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    pub const OPCODE: u32 = 0b11101010000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ANDS_64_log_shift";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ANDS_64_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ANDS_64_log_shift {
        #[inline]
        pub const fn new(
            shift: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            imm6: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                shift,
                Rm,
                imm6,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11101010u32 << 24u32
                    | self.shift.into_inner() << 22u32
                    | 0b0u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.imm6.into_inner() << 10u32
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
pub mod BICS_64_log_shift {
    pub const OPCODE_MASK: u32 = 0b11111111001000000000000000000000u32;
    pub const OPCODE: u32 = 0b11101010001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BICS_64_log_shift";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BICS_64_log_shift {
        pub shift: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl BICS_64_log_shift {
        #[inline]
        pub const fn new(
            shift: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            imm6: ::aarchmrs_types::BitValue<6>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                shift,
                Rm,
                imm6,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11101010u32 << 24u32
                    | self.shift.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.imm6.into_inner() << 10u32
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
