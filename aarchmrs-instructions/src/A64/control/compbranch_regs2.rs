/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CBBGT_8_regs {
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    pub const OPCODE: u32 = 0b01110100000000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CBBGT_8_regs";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBBGT_8_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBBGT_8_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110100000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.imm9.into_inner() << 5u32
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
pub mod CBBGE_8_regs {
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    pub const OPCODE: u32 = 0b01110100001000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CBBGE_8_regs";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBBGE_8_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBBGE_8_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110100001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.imm9.into_inner() << 5u32
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
pub mod CBBHI_8_regs {
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    pub const OPCODE: u32 = 0b01110100010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CBBHI_8_regs";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBBHI_8_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBBHI_8_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110100010u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.imm9.into_inner() << 5u32
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
pub mod CBBHS_8_regs {
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    pub const OPCODE: u32 = 0b01110100011000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CBBHS_8_regs";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBBHS_8_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBBHS_8_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110100011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.imm9.into_inner() << 5u32
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
pub mod CBBEQ_8_regs {
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    pub const OPCODE: u32 = 0b01110100110000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CBBEQ_8_regs";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBBEQ_8_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBBEQ_8_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110100110u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.imm9.into_inner() << 5u32
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
pub mod CBBNE_8_regs {
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    pub const OPCODE: u32 = 0b01110100111000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CBBNE_8_regs";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBBNE_8_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBBNE_8_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110100111u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.imm9.into_inner() << 5u32
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
pub mod CBHGT_16_regs {
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    pub const OPCODE: u32 = 0b01110100000000001100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CBHGT_16_regs";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBHGT_16_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBHGT_16_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110100000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b11u32 << 14u32
                    | self.imm9.into_inner() << 5u32
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
pub mod CBHGE_16_regs {
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    pub const OPCODE: u32 = 0b01110100001000001100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CBHGE_16_regs";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBHGE_16_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBHGE_16_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110100001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b11u32 << 14u32
                    | self.imm9.into_inner() << 5u32
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
pub mod CBHHI_16_regs {
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    pub const OPCODE: u32 = 0b01110100010000001100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CBHHI_16_regs";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBHHI_16_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBHHI_16_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110100010u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b11u32 << 14u32
                    | self.imm9.into_inner() << 5u32
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
pub mod CBHHS_16_regs {
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    pub const OPCODE: u32 = 0b01110100011000001100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CBHHS_16_regs";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBHHS_16_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBHHS_16_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110100011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b11u32 << 14u32
                    | self.imm9.into_inner() << 5u32
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
pub mod CBHEQ_16_regs {
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    pub const OPCODE: u32 = 0b01110100110000001100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CBHEQ_16_regs";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBHEQ_16_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBHEQ_16_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110100110u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b11u32 << 14u32
                    | self.imm9.into_inner() << 5u32
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
pub mod CBHNE_16_regs {
    pub const OPCODE_MASK: u32 = 0b11111111111000001100000000000000u32;
    pub const OPCODE: u32 = 0b01110100111000001100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CBHNE_16_regs";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBHNE_16_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBHNE_16_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110100111u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b11u32 << 14u32
                    | self.imm9.into_inner() << 5u32
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
