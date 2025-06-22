/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod LDR_32_loadlit {
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    pub const OPCODE: u32 = 0b00011000000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDR_32_loadlit";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_32_loadlit {
        pub imm19: ::aarchmrs_types::BitValue<19>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_32_loadlit {
        #[inline]
        pub const fn new(
            imm19: ::aarchmrs_types::BitValue<19>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm19, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011000u32 << 24u32
                    | self.imm19.into_inner() << 5u32
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
pub mod LDR_S_loadlit {
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    pub const OPCODE: u32 = 0b00011100000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDR_S_loadlit";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_S_loadlit {
        pub imm19: ::aarchmrs_types::BitValue<19>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_S_loadlit {
        #[inline]
        pub const fn new(
            imm19: ::aarchmrs_types::BitValue<19>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm19, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011100u32 << 24u32
                    | self.imm19.into_inner() << 5u32
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
pub mod LDR_64_loadlit {
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    pub const OPCODE: u32 = 0b01011000000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDR_64_loadlit";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_64_loadlit {
        pub imm19: ::aarchmrs_types::BitValue<19>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_64_loadlit {
        #[inline]
        pub const fn new(
            imm19: ::aarchmrs_types::BitValue<19>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm19, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011000u32 << 24u32
                    | self.imm19.into_inner() << 5u32
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
pub mod LDR_D_loadlit {
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    pub const OPCODE: u32 = 0b01011100000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDR_D_loadlit";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_D_loadlit {
        pub imm19: ::aarchmrs_types::BitValue<19>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_D_loadlit {
        #[inline]
        pub const fn new(
            imm19: ::aarchmrs_types::BitValue<19>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm19, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011100u32 << 24u32
                    | self.imm19.into_inner() << 5u32
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
pub mod LDRSW_64_loadlit {
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    pub const OPCODE: u32 = 0b10011000000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDRSW_64_loadlit";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSW_64_loadlit {
        pub imm19: ::aarchmrs_types::BitValue<19>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSW_64_loadlit {
        #[inline]
        pub const fn new(
            imm19: ::aarchmrs_types::BitValue<19>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm19, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011000u32 << 24u32
                    | self.imm19.into_inner() << 5u32
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
pub mod LDR_Q_loadlit {
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    pub const OPCODE: u32 = 0b10011100000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LDR_Q_loadlit";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_Q_loadlit {
        pub imm19: ::aarchmrs_types::BitValue<19>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_Q_loadlit {
        #[inline]
        pub const fn new(
            imm19: ::aarchmrs_types::BitValue<19>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm19, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011100u32 << 24u32
                    | self.imm19.into_inner() << 5u32
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
pub mod PRFM_P_loadlit {
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000000000u32;
    pub const OPCODE: u32 = 0b11011000000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "PRFM_P_loadlit";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PRFM_P_loadlit {
        pub imm19: ::aarchmrs_types::BitValue<19>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl PRFM_P_loadlit {
        #[inline]
        pub const fn new(
            imm19: ::aarchmrs_types::BitValue<19>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm19, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011000u32 << 24u32
                    | self.imm19.into_inner() << 5u32
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
