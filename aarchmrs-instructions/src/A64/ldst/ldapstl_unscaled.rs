/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STLURB_32_ldapstl_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00011001000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STLURB_32_ldapstl_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLURB_32_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLURB_32_ldapstl_unscaled {
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
                0b00011001000u32 << 21u32
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
pub mod LDAPURB_32_ldapstl_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00011001010000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDAPURB_32_ldapstl_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPURB_32_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPURB_32_ldapstl_unscaled {
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
                0b00011001010u32 << 21u32
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
pub mod LDAPURSB_64_ldapstl_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00011001100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDAPURSB_64_ldapstl_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPURSB_64_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPURSB_64_ldapstl_unscaled {
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
                0b00011001100u32 << 21u32
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
pub mod LDAPURSB_32_ldapstl_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00011001110000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDAPURSB_32_ldapstl_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPURSB_32_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPURSB_32_ldapstl_unscaled {
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
                0b00011001110u32 << 21u32
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
pub mod STLURH_32_ldapstl_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01011001000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STLURH_32_ldapstl_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLURH_32_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLURH_32_ldapstl_unscaled {
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
                0b01011001000u32 << 21u32
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
pub mod LDAPURH_32_ldapstl_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01011001010000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDAPURH_32_ldapstl_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPURH_32_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPURH_32_ldapstl_unscaled {
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
                0b01011001010u32 << 21u32
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
pub mod LDAPURSH_64_ldapstl_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01011001100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDAPURSH_64_ldapstl_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPURSH_64_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPURSH_64_ldapstl_unscaled {
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
                0b01011001100u32 << 21u32
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
pub mod LDAPURSH_32_ldapstl_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01011001110000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDAPURSH_32_ldapstl_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPURSH_32_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPURSH_32_ldapstl_unscaled {
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
                0b01011001110u32 << 21u32
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
pub mod STLUR_32_ldapstl_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b10011001000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STLUR_32_ldapstl_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLUR_32_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLUR_32_ldapstl_unscaled {
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
                0b10011001000u32 << 21u32
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
pub mod LDAPUR_32_ldapstl_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b10011001010000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDAPUR_32_ldapstl_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPUR_32_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPUR_32_ldapstl_unscaled {
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
                0b10011001010u32 << 21u32
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
pub mod LDAPURSW_64_ldapstl_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b10011001100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDAPURSW_64_ldapstl_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPURSW_64_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPURSW_64_ldapstl_unscaled {
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
                0b10011001100u32 << 21u32
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
pub mod STLUR_64_ldapstl_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b11011001000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STLUR_64_ldapstl_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLUR_64_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLUR_64_ldapstl_unscaled {
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
                0b11011001000u32 << 21u32
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
pub mod LDAPUR_64_ldapstl_unscaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b11011001010000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDAPUR_64_ldapstl_unscaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPUR_64_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPUR_64_ldapstl_unscaled {
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
                0b11011001010u32 << 21u32
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
