/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STURB_32_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111000000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STURB_32_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STURB_32_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STURB_32_ldst_unscaled {
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
                    | 0b00u32 << 10u32
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
pub mod LDURB_32_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111000010000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDURB_32_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDURB_32_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDURB_32_ldst_unscaled {
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
                    | 0b00u32 << 10u32
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
pub mod LDURSB_64_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111000100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDURSB_64_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDURSB_64_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDURSB_64_ldst_unscaled {
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
                    | 0b00u32 << 10u32
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
pub mod LDURSB_32_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111000110000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDURSB_32_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDURSB_32_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDURSB_32_ldst_unscaled {
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
                    | 0b00u32 << 10u32
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
pub mod STUR_B_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111100000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STUR_B_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STUR_B_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STUR_B_ldst_unscaled {
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
                0b00111100000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
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
pub mod LDUR_B_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111100010000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDUR_B_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUR_B_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUR_B_ldst_unscaled {
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
                0b00111100010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
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
pub mod STUR_Q_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111100100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STUR_Q_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STUR_Q_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STUR_Q_ldst_unscaled {
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
                0b00111100100u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
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
pub mod LDUR_Q_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111100110000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDUR_Q_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUR_Q_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUR_Q_ldst_unscaled {
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
                0b00111100110u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
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
pub mod STURH_32_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01111000000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STURH_32_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STURH_32_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STURH_32_ldst_unscaled {
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
                    | 0b00u32 << 10u32
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
pub mod LDURH_32_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01111000010000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDURH_32_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDURH_32_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDURH_32_ldst_unscaled {
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
                    | 0b00u32 << 10u32
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
pub mod LDURSH_64_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01111000100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDURSH_64_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDURSH_64_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDURSH_64_ldst_unscaled {
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
                    | 0b00u32 << 10u32
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
pub mod LDURSH_32_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01111000110000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDURSH_32_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDURSH_32_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDURSH_32_ldst_unscaled {
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
                    | 0b00u32 << 10u32
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
pub mod STUR_H_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01111100000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STUR_H_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STUR_H_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STUR_H_ldst_unscaled {
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
                0b01111100000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
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
pub mod LDUR_H_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01111100010000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDUR_H_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUR_H_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUR_H_ldst_unscaled {
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
                0b01111100010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
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
pub mod STUR_32_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b10111000000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STUR_32_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STUR_32_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STUR_32_ldst_unscaled {
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
                    | 0b00u32 << 10u32
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
pub mod LDUR_32_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b10111000010000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDUR_32_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUR_32_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUR_32_ldst_unscaled {
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
                    | 0b00u32 << 10u32
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
pub mod LDURSW_64_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b10111000100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDURSW_64_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDURSW_64_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDURSW_64_ldst_unscaled {
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
                    | 0b00u32 << 10u32
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
pub mod STUR_S_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b10111100000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STUR_S_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STUR_S_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STUR_S_ldst_unscaled {
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
                0b10111100000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
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
pub mod LDUR_S_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b10111100010000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDUR_S_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUR_S_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUR_S_ldst_unscaled {
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
                0b10111100010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
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
pub mod STUR_64_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b11111000000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STUR_64_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STUR_64_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STUR_64_ldst_unscaled {
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
                    | 0b00u32 << 10u32
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
pub mod LDUR_64_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b11111000010000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDUR_64_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUR_64_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUR_64_ldst_unscaled {
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
                    | 0b00u32 << 10u32
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
pub mod PRFUM_P_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b11111000100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "PRFUM_P_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PRFUM_P_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl PRFUM_P_ldst_unscaled {
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
                0b11111000100u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
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
pub mod STUR_D_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b11111100000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STUR_D_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STUR_D_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STUR_D_ldst_unscaled {
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
                0b11111100000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
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
pub mod LDUR_D_ldst_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b11111100010000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDUR_D_ldst_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDUR_D_ldst_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDUR_D_ldst_unscaled {
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
                0b11111100010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
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
