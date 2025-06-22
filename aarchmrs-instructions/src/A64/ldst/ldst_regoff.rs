/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STRB_32B_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111000001000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STRB_32B_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STRB_32B_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STRB_32B_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod STRB_32BL_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000001110110000000000u32;
    pub const OPCODE: u32 = 0b00111000001000000110100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STRB_32BL_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STRB_32BL_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STRB_32BL_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, S, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b011u32 << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod LDRB_32B_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111000011000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDRB_32B_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRB_32B_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRB_32B_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod LDRB_32BL_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000001110110000000000u32;
    pub const OPCODE: u32 = 0b00111000011000000110100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDRB_32BL_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRB_32BL_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRB_32BL_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, S, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b011u32 << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod LDRSB_64B_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111000101000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDRSB_64B_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSB_64B_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSB_64B_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000101u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod LDRSB_64BL_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000001110110000000000u32;
    pub const OPCODE: u32 = 0b00111000101000000110100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDRSB_64BL_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSB_64BL_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSB_64BL_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, S, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000101u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b011u32 << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod LDRSB_32B_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111000111000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDRSB_32B_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSB_32B_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSB_32B_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000111u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod LDRSB_32BL_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000001110110000000000u32;
    pub const OPCODE: u32 = 0b00111000111000000110100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDRSB_32BL_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSB_32BL_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSB_32BL_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, S, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000111u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b011u32 << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod STR_B_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111100001000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STR_B_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_B_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_B_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod STR_BL_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000001110110000000000u32;
    pub const OPCODE: u32 = 0b00111100001000000110100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STR_BL_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_BL_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_BL_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, S, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b011u32 << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod LDR_B_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111100011000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDR_B_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_B_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_B_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod LDR_BL_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000001110110000000000u32;
    pub const OPCODE: u32 = 0b00111100011000000110100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDR_BL_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_BL_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_BL_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, S, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b011u32 << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod STR_Q_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111100101000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STR_Q_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_Q_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_Q_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100101u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod LDR_Q_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b00111100111000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDR_Q_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_Q_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_Q_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100111u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod STRH_32_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01111000001000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STRH_32_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STRH_32_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STRH_32_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111000001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod LDRH_32_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01111000011000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDRH_32_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRH_32_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRH_32_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111000011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod LDRSH_64_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01111000101000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDRSH_64_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSH_64_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSH_64_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111000101u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod LDRSH_32_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01111000111000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDRSH_32_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSH_32_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSH_32_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111000111u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod STR_H_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01111100001000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STR_H_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_H_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_H_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111100001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod LDR_H_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b01111100011000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDR_H_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_H_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_H_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111100011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod STR_32_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b10111000001000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STR_32_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_32_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_32_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111000001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod LDR_32_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b10111000011000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDR_32_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_32_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_32_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111000011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod LDRSW_64_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b10111000101000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDRSW_64_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSW_64_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSW_64_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111000101u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod STR_S_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b10111100001000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STR_S_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_S_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_S_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111100001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod LDR_S_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b10111100011000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDR_S_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_S_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_S_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111100011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod STR_64_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b11111000001000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STR_64_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_64_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_64_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111000001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod LDR_64_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b11111000011000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDR_64_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_64_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_64_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111000011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod PRFM_P_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b11111000101000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "PRFM_P_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PRFM_P_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl PRFM_P_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111000101u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod RPRFM_R_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b11111000101000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "RPRFM_R_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RPRFM_R_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl RPRFM_R_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111000101u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod STR_D_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b11111100001000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "STR_D_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_D_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_D_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111100001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
pub mod LDR_D_ldst_regoff {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000110000000000u32;
    pub const OPCODE: u32 = 0b11111100011000000000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDR_D_ldst_regoff";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_D_ldst_regoff {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_D_ldst_regoff {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                S,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111100011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.S.into_inner() << 12u32
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
