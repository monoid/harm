/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ST4_asisdlsep_R4_r {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    pub const OPCODE: u32 = 0b00001100100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ST4_asisdlsep_R4_r";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ST4_asisdlsep_R4_r {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl ST4_asisdlsep_R4_r {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                Rm,
                size,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100100u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0000u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod ST1_asisdlsep_R4_r4 {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    pub const OPCODE: u32 = 0b00001100100000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ST1_asisdlsep_R4_r4";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ST1_asisdlsep_R4_r4 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl ST1_asisdlsep_R4_r4 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                Rm,
                size,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100100u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0010u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod ST3_asisdlsep_R3_r {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    pub const OPCODE: u32 = 0b00001100100000000100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ST3_asisdlsep_R3_r";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ST3_asisdlsep_R3_r {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl ST3_asisdlsep_R3_r {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                Rm,
                size,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100100u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0100u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod ST1_asisdlsep_R3_r3 {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    pub const OPCODE: u32 = 0b00001100100000000110000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ST1_asisdlsep_R3_r3";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ST1_asisdlsep_R3_r3 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl ST1_asisdlsep_R3_r3 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                Rm,
                size,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100100u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0110u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod ST1_asisdlsep_R1_r1 {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    pub const OPCODE: u32 = 0b00001100100000000111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ST1_asisdlsep_R1_r1";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ST1_asisdlsep_R1_r1 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl ST1_asisdlsep_R1_r1 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                Rm,
                size,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100100u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0111u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod ST2_asisdlsep_R2_r {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    pub const OPCODE: u32 = 0b00001100100000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ST2_asisdlsep_R2_r";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ST2_asisdlsep_R2_r {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl ST2_asisdlsep_R2_r {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                Rm,
                size,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100100u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1000u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod ST1_asisdlsep_R2_r2 {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    pub const OPCODE: u32 = 0b00001100100000001010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ST1_asisdlsep_R2_r2";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ST1_asisdlsep_R2_r2 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl ST1_asisdlsep_R2_r2 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                Rm,
                size,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100100u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1010u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod ST4_asisdlsep_I4_i {
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    pub const OPCODE: u32 = 0b00001100100111110000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ST4_asisdlsep_I4_i";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ST4_asisdlsep_I4_i {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl ST4_asisdlsep_I4_i {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, size, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100100111110000u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod ST1_asisdlsep_I4_i4 {
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    pub const OPCODE: u32 = 0b00001100100111110010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ST1_asisdlsep_I4_i4";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ST1_asisdlsep_I4_i4 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl ST1_asisdlsep_I4_i4 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, size, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100100111110010u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod ST3_asisdlsep_I3_i {
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    pub const OPCODE: u32 = 0b00001100100111110100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ST3_asisdlsep_I3_i";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ST3_asisdlsep_I3_i {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl ST3_asisdlsep_I3_i {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, size, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100100111110100u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod ST1_asisdlsep_I3_i3 {
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    pub const OPCODE: u32 = 0b00001100100111110110000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ST1_asisdlsep_I3_i3";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ST1_asisdlsep_I3_i3 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl ST1_asisdlsep_I3_i3 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, size, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100100111110110u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod ST1_asisdlsep_I1_i1 {
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    pub const OPCODE: u32 = 0b00001100100111110111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ST1_asisdlsep_I1_i1";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ST1_asisdlsep_I1_i1 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl ST1_asisdlsep_I1_i1 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, size, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100100111110111u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod ST2_asisdlsep_I2_i {
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    pub const OPCODE: u32 = 0b00001100100111111000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ST2_asisdlsep_I2_i";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ST2_asisdlsep_I2_i {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl ST2_asisdlsep_I2_i {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, size, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100100111111000u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod ST1_asisdlsep_I2_i2 {
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    pub const OPCODE: u32 = 0b00001100100111111010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ST1_asisdlsep_I2_i2";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ST1_asisdlsep_I2_i2 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl ST1_asisdlsep_I2_i2 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, size, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100100111111010u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod LD4_asisdlsep_R4_r {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    pub const OPCODE: u32 = 0b00001100110000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LD4_asisdlsep_R4_r";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LD4_asisdlsep_R4_r {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LD4_asisdlsep_R4_r {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                Rm,
                size,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100110u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0000u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod LD1_asisdlsep_R4_r4 {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    pub const OPCODE: u32 = 0b00001100110000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LD1_asisdlsep_R4_r4";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LD1_asisdlsep_R4_r4 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LD1_asisdlsep_R4_r4 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                Rm,
                size,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100110u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0010u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod LD3_asisdlsep_R3_r {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    pub const OPCODE: u32 = 0b00001100110000000100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LD3_asisdlsep_R3_r";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LD3_asisdlsep_R3_r {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LD3_asisdlsep_R3_r {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                Rm,
                size,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100110u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0100u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod LD1_asisdlsep_R3_r3 {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    pub const OPCODE: u32 = 0b00001100110000000110000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LD1_asisdlsep_R3_r3";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LD1_asisdlsep_R3_r3 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LD1_asisdlsep_R3_r3 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                Rm,
                size,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100110u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0110u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod LD1_asisdlsep_R1_r1 {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    pub const OPCODE: u32 = 0b00001100110000000111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LD1_asisdlsep_R1_r1";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LD1_asisdlsep_R1_r1 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LD1_asisdlsep_R1_r1 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                Rm,
                size,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100110u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0111u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod LD2_asisdlsep_R2_r {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    pub const OPCODE: u32 = 0b00001100110000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LD2_asisdlsep_R2_r";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LD2_asisdlsep_R2_r {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LD2_asisdlsep_R2_r {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                Rm,
                size,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100110u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1000u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod LD1_asisdlsep_R2_r2 {
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    pub const OPCODE: u32 = 0b00001100110000001010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LD1_asisdlsep_R2_r2";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LD1_asisdlsep_R2_r2 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LD1_asisdlsep_R2_r2 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                Rm,
                size,
                Rn,
                Rt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100110u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1010u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod LD4_asisdlsep_I4_i {
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    pub const OPCODE: u32 = 0b00001100110111110000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LD4_asisdlsep_I4_i";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LD4_asisdlsep_I4_i {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LD4_asisdlsep_I4_i {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, size, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100110111110000u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod LD1_asisdlsep_I4_i4 {
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    pub const OPCODE: u32 = 0b00001100110111110010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LD1_asisdlsep_I4_i4";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LD1_asisdlsep_I4_i4 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LD1_asisdlsep_I4_i4 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, size, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100110111110010u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod LD3_asisdlsep_I3_i {
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    pub const OPCODE: u32 = 0b00001100110111110100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LD3_asisdlsep_I3_i";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LD3_asisdlsep_I3_i {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LD3_asisdlsep_I3_i {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, size, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100110111110100u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod LD1_asisdlsep_I3_i3 {
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    pub const OPCODE: u32 = 0b00001100110111110110000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LD1_asisdlsep_I3_i3";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LD1_asisdlsep_I3_i3 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LD1_asisdlsep_I3_i3 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, size, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100110111110110u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod LD1_asisdlsep_I1_i1 {
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    pub const OPCODE: u32 = 0b00001100110111110111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LD1_asisdlsep_I1_i1";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LD1_asisdlsep_I1_i1 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LD1_asisdlsep_I1_i1 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, size, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100110111110111u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod LD2_asisdlsep_I2_i {
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    pub const OPCODE: u32 = 0b00001100110111111000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LD2_asisdlsep_I2_i";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LD2_asisdlsep_I2_i {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LD2_asisdlsep_I2_i {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, size, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100110111111000u32 << 12u32
                    | self.size.into_inner() << 10u32
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
pub mod LD1_asisdlsep_I2_i2 {
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    pub const OPCODE: u32 = 0b00001100110111111010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "LD1_asisdlsep_I2_i2";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LD1_asisdlsep_I2_i2 {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LD1_asisdlsep_I2_i2 {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Q, size, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001100110111111010u32 << 12u32
                    | self.size.into_inner() << 10u32
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
