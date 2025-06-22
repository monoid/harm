/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADD_32_addsub_ext {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000000000u32;
    pub const OPCODE: u32 = 0b00001011001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ADD_32_addsub_ext";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADD_32_addsub_ext {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADD_32_addsub_ext {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            imm3: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                imm3,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00001011001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.imm3.into_inner() << 10u32
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
pub mod ADDS_32S_addsub_ext {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000000000u32;
    pub const OPCODE: u32 = 0b00101011001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ADDS_32S_addsub_ext";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADDS_32S_addsub_ext {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADDS_32S_addsub_ext {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            imm3: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                imm3,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00101011001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.imm3.into_inner() << 10u32
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
pub mod SUB_32_addsub_ext {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000000000u32;
    pub const OPCODE: u32 = 0b01001011001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SUB_32_addsub_ext";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUB_32_addsub_ext {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUB_32_addsub_ext {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            imm3: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                imm3,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01001011001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.imm3.into_inner() << 10u32
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
pub mod SUBS_32S_addsub_ext {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000000000u32;
    pub const OPCODE: u32 = 0b01101011001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SUBS_32S_addsub_ext";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUBS_32S_addsub_ext {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUBS_32S_addsub_ext {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            imm3: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                imm3,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01101011001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.imm3.into_inner() << 10u32
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
pub mod ADD_64_addsub_ext {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000000000u32;
    pub const OPCODE: u32 = 0b10001011001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ADD_64_addsub_ext";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADD_64_addsub_ext {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADD_64_addsub_ext {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            imm3: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                imm3,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10001011001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.imm3.into_inner() << 10u32
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
pub mod ADDS_64S_addsub_ext {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000000000u32;
    pub const OPCODE: u32 = 0b10101011001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ADDS_64S_addsub_ext";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADDS_64S_addsub_ext {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADDS_64S_addsub_ext {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            imm3: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                imm3,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10101011001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.imm3.into_inner() << 10u32
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
pub mod SUB_64_addsub_ext {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000000000u32;
    pub const OPCODE: u32 = 0b11001011001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SUB_64_addsub_ext";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUB_64_addsub_ext {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUB_64_addsub_ext {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            imm3: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                imm3,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001011001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.imm3.into_inner() << 10u32
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
pub mod SUBS_64S_addsub_ext {
    pub const OPCODE_MASK: u32 = 0b11111111111000000000000000000000u32;
    pub const OPCODE: u32 = 0b11101011001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SUBS_64S_addsub_ext";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUBS_64S_addsub_ext {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUBS_64S_addsub_ext {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            option: ::aarchmrs_types::BitValue<3>,
            imm3: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                option,
                imm3,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11101011001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | self.option.into_inner() << 13u32
                    | self.imm3.into_inner() << 10u32
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
