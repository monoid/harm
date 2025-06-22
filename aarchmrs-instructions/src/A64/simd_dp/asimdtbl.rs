/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod TBL_asimdtbl_L1_1 {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00001110000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "TBL_asimdtbl_L1_1";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TBL_asimdtbl_L1_1 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl TBL_asimdtbl_L1_1 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, Rm, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001110000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b000000u32 << 10u32
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
pub mod TBX_asimdtbl_L1_1 {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00001110000000000001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "TBX_asimdtbl_L1_1";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TBX_asimdtbl_L1_1 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl TBX_asimdtbl_L1_1 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, Rm, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001110000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b000100u32 << 10u32
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
pub mod TBL_asimdtbl_L2_2 {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00001110000000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "TBL_asimdtbl_L2_2";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TBL_asimdtbl_L2_2 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl TBL_asimdtbl_L2_2 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, Rm, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001110000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b001000u32 << 10u32
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
pub mod TBX_asimdtbl_L2_2 {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00001110000000000011000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "TBX_asimdtbl_L2_2";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TBX_asimdtbl_L2_2 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl TBX_asimdtbl_L2_2 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, Rm, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001110000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b001100u32 << 10u32
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
pub mod TBL_asimdtbl_L3_3 {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00001110000000000100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "TBL_asimdtbl_L3_3";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TBL_asimdtbl_L3_3 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl TBL_asimdtbl_L3_3 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, Rm, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001110000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b010000u32 << 10u32
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
pub mod TBX_asimdtbl_L3_3 {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00001110000000000101000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "TBX_asimdtbl_L3_3";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TBX_asimdtbl_L3_3 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl TBX_asimdtbl_L3_3 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, Rm, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001110000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b010100u32 << 10u32
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
pub mod TBL_asimdtbl_L4_4 {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00001110000000000110000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "TBL_asimdtbl_L4_4";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TBL_asimdtbl_L4_4 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl TBL_asimdtbl_L4_4 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, Rm, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001110000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b011000u32 << 10u32
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
pub mod TBX_asimdtbl_L4_4 {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00001110000000000111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "TBX_asimdtbl_L4_4";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TBX_asimdtbl_L4_4 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl TBX_asimdtbl_L4_4 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, Rm, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001110000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b011100u32 << 10u32
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
pub mod LUTI4_asimdtbl_L7 {
    pub const OPCODE_MASK: u32 = 0b11111111111000001001110000000000u32;
    pub const OPCODE: u32 = 0b01001110010000000001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LUTI4_asimdtbl_L7";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LUTI4_asimdtbl_L7 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub len: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl LUTI4_asimdtbl_L7 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            len: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, len, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01001110010u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.len.into_inner() << 13u32
                    | 0b100u32 << 10u32
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
pub mod LUTI4_asimdtbl_L5 {
    pub const OPCODE_MASK: u32 = 0b11111111111000001011110000000000u32;
    pub const OPCODE: u32 = 0b01001110010000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LUTI4_asimdtbl_L5";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LUTI4_asimdtbl_L5 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub len: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl LUTI4_asimdtbl_L5 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            len: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, len, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01001110010u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.len.into_inner() << 14u32
                    | 0b1000u32 << 10u32
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
pub mod LUTI2_asimdtbl_L5 {
    pub const OPCODE_MASK: u32 = 0b11111111111000001001110000000000u32;
    pub const OPCODE: u32 = 0b01001110100000000001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LUTI2_asimdtbl_L5";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LUTI2_asimdtbl_L5 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub len: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl LUTI2_asimdtbl_L5 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            len: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, len, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01001110100u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.len.into_inner() << 13u32
                    | 0b100u32 << 10u32
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
pub mod LUTI2_asimdtbl_L6 {
    pub const OPCODE_MASK: u32 = 0b11111111111000001000110000000000u32;
    pub const OPCODE: u32 = 0b01001110110000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LUTI2_asimdtbl_L6";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LUTI2_asimdtbl_L6 {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub len: ::aarchmrs_types::BitValue<2>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl LUTI2_asimdtbl_L6 {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            len: ::aarchmrs_types::BitValue<2>,
            op: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Rm,
                len,
                op,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01001110110u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.len.into_inner() << 13u32
                    | self.op.into_inner() << 12u32
                    | 0b00u32 << 10u32
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
