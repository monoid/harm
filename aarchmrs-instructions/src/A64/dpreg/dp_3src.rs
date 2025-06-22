/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MADD_32A_dp_3src {
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    pub const OPCODE: u32 = 0b00011011000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MADD_32A_dp_3src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MADD_32A_dp_3src {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MADD_32A_dp_3src {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Ra: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, Ra, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011011000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.Ra.into_inner() << 10u32
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
pub mod MSUB_32A_dp_3src {
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    pub const OPCODE: u32 = 0b00011011000000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MSUB_32A_dp_3src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MSUB_32A_dp_3src {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MSUB_32A_dp_3src {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Ra: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, Ra, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011011000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.Ra.into_inner() << 10u32
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
pub mod MADD_64A_dp_3src {
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    pub const OPCODE: u32 = 0b10011011000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MADD_64A_dp_3src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MADD_64A_dp_3src {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MADD_64A_dp_3src {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Ra: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, Ra, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.Ra.into_inner() << 10u32
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
pub mod MSUB_64A_dp_3src {
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    pub const OPCODE: u32 = 0b10011011000000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MSUB_64A_dp_3src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MSUB_64A_dp_3src {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MSUB_64A_dp_3src {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Ra: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, Ra, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.Ra.into_inner() << 10u32
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
pub mod SMADDL_64WA_dp_3src {
    pub const OPCODE_MASK: u32 = 0b11111111011000001000000000000000u32;
    pub const OPCODE: u32 = 0b10011011001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SMADDL_64WA_dp_3src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SMADDL_64WA_dp_3src {
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SMADDL_64WA_dp_3src {
        #[inline]
        pub const fn new(
            U: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Ra: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { U, Rm, Ra, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011u32 << 24u32
                    | self.U.into_inner() << 23u32
                    | 0b01u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.Ra.into_inner() << 10u32
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
pub mod SMSUBL_64WA_dp_3src {
    pub const OPCODE_MASK: u32 = 0b11111111011000001000000000000000u32;
    pub const OPCODE: u32 = 0b10011011001000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SMSUBL_64WA_dp_3src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SMSUBL_64WA_dp_3src {
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SMSUBL_64WA_dp_3src {
        #[inline]
        pub const fn new(
            U: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Ra: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { U, Rm, Ra, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011u32 << 24u32
                    | self.U.into_inner() << 23u32
                    | 0b01u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.Ra.into_inner() << 10u32
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
pub mod SMULH_64_dp_3src {
    pub const OPCODE_MASK: u32 = 0b11111111011000001111110000000000u32;
    pub const OPCODE: u32 = 0b10011011010000000111110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000111110000000000u32;
    pub const NAME: &str = "SMULH_64_dp_3src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SMULH_64_dp_3src {
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SMULH_64_dp_3src {
        #[inline]
        pub const fn new(
            U: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { U, Rm, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011u32 << 24u32
                    | self.U.into_inner() << 23u32
                    | 0b10u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b011111u32 << 10u32
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
pub mod MADDPT_64A_dp_3src {
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    pub const OPCODE: u32 = 0b10011011011000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MADDPT_64A_dp_3src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MADDPT_64A_dp_3src {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MADDPT_64A_dp_3src {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Ra: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, Ra, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.Ra.into_inner() << 10u32
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
pub mod MSUBPT_64A_dp_3src {
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    pub const OPCODE: u32 = 0b10011011011000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MSUBPT_64A_dp_3src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MSUBPT_64A_dp_3src {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MSUBPT_64A_dp_3src {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Ra: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, Ra, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.Ra.into_inner() << 10u32
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
pub mod UMADDL_64WA_dp_3src {
    pub const OPCODE_MASK: u32 = 0b11111111011000001000000000000000u32;
    pub const OPCODE: u32 = 0b10011011001000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "UMADDL_64WA_dp_3src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UMADDL_64WA_dp_3src {
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UMADDL_64WA_dp_3src {
        #[inline]
        pub const fn new(
            U: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Ra: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { U, Rm, Ra, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011u32 << 24u32
                    | self.U.into_inner() << 23u32
                    | 0b01u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.Ra.into_inner() << 10u32
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
pub mod UMSUBL_64WA_dp_3src {
    pub const OPCODE_MASK: u32 = 0b11111111011000001000000000000000u32;
    pub const OPCODE: u32 = 0b10011011001000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "UMSUBL_64WA_dp_3src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UMSUBL_64WA_dp_3src {
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Ra: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UMSUBL_64WA_dp_3src {
        #[inline]
        pub const fn new(
            U: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Ra: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { U, Rm, Ra, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011u32 << 24u32
                    | self.U.into_inner() << 23u32
                    | 0b01u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.Ra.into_inner() << 10u32
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
pub mod UMULH_64_dp_3src {
    pub const OPCODE_MASK: u32 = 0b11111111011000001111110000000000u32;
    pub const OPCODE: u32 = 0b10011011010000000111110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000111110000000000u32;
    pub const NAME: &str = "UMULH_64_dp_3src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UMULH_64_dp_3src {
        pub U: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UMULH_64_dp_3src {
        #[inline]
        pub const fn new(
            U: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { U, Rm, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10011011u32 << 24u32
                    | self.U.into_inner() << 23u32
                    | 0b10u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b011111u32 << 10u32
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
