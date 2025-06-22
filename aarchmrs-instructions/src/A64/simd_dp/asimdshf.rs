/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SSHR_asimdshf_R {
    pub const OPCODE_MASK: u32 = 0b10111111100000001100110000000000u32;
    pub const OPCODE: u32 = 0b00001111000000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SSHR_asimdshf_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SSHR_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SSHR_asimdshf_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            o1: ::aarchmrs_types::BitValue<1>,
            o0: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                o1,
                o0,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b0011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.o1.into_inner() << 13u32
                    | self.o0.into_inner() << 12u32
                    | 0b01u32 << 10u32
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
pub mod SSRA_asimdshf_R {
    pub const OPCODE_MASK: u32 = 0b10111111100000001100110000000000u32;
    pub const OPCODE: u32 = 0b00001111000000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SSRA_asimdshf_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SSRA_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SSRA_asimdshf_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            o1: ::aarchmrs_types::BitValue<1>,
            o0: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                o1,
                o0,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b0011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.o1.into_inner() << 13u32
                    | self.o0.into_inner() << 12u32
                    | 0b01u32 << 10u32
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
pub mod SRSHR_asimdshf_R {
    pub const OPCODE_MASK: u32 = 0b10111111100000001100110000000000u32;
    pub const OPCODE: u32 = 0b00001111000000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SRSHR_asimdshf_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SRSHR_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SRSHR_asimdshf_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            o1: ::aarchmrs_types::BitValue<1>,
            o0: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                o1,
                o0,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b0011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.o1.into_inner() << 13u32
                    | self.o0.into_inner() << 12u32
                    | 0b01u32 << 10u32
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
pub mod SRSRA_asimdshf_R {
    pub const OPCODE_MASK: u32 = 0b10111111100000001100110000000000u32;
    pub const OPCODE: u32 = 0b00001111000000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SRSRA_asimdshf_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SRSRA_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SRSRA_asimdshf_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            o1: ::aarchmrs_types::BitValue<1>,
            o0: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                o1,
                o0,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b0011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.o1.into_inner() << 13u32
                    | self.o0.into_inner() << 12u32
                    | 0b01u32 << 10u32
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
pub mod SHL_asimdshf_R {
    pub const OPCODE_MASK: u32 = 0b10111111100000001111110000000000u32;
    pub const OPCODE: u32 = 0b00001111000000000101010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SHL_asimdshf_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SHL_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SHL_asimdshf_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b0011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b010101u32 << 10u32
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
pub mod SQSHL_asimdshf_R {
    pub const OPCODE_MASK: u32 = 0b10111111100000001110110000000000u32;
    pub const OPCODE: u32 = 0b00001111000000000110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SQSHL_asimdshf_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQSHL_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQSHL_asimdshf_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            op: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                op,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b0011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b011u32 << 13u32
                    | self.op.into_inner() << 12u32
                    | 0b01u32 << 10u32
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
pub mod SHRN_asimdshf_N {
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SHRN_asimdshf_N";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SHRN_asimdshf_N {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SHRN_asimdshf_N {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            op: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                op,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b0011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b1000u32 << 12u32
                    | self.op.into_inner() << 11u32
                    | 0b1u32 << 10u32
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
pub mod RSHRN_asimdshf_N {
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "RSHRN_asimdshf_N";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RSHRN_asimdshf_N {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl RSHRN_asimdshf_N {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            op: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                op,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b0011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b1000u32 << 12u32
                    | self.op.into_inner() << 11u32
                    | 0b1u32 << 10u32
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
pub mod SQSHRN_asimdshf_N {
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SQSHRN_asimdshf_N";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQSHRN_asimdshf_N {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQSHRN_asimdshf_N {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            op: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                op,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b0011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b1001u32 << 12u32
                    | self.op.into_inner() << 11u32
                    | 0b1u32 << 10u32
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
pub mod SQRSHRN_asimdshf_N {
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SQRSHRN_asimdshf_N";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQRSHRN_asimdshf_N {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQRSHRN_asimdshf_N {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            op: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                op,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b0011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b1001u32 << 12u32
                    | self.op.into_inner() << 11u32
                    | 0b1u32 << 10u32
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
pub mod SSHLL_asimdshf_L {
    pub const OPCODE_MASK: u32 = 0b10111111100000001111110000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001010010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SSHLL_asimdshf_L";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SSHLL_asimdshf_L {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SSHLL_asimdshf_L {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b0011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b101001u32 << 10u32
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
pub mod SCVTF_asimdshf_C {
    pub const OPCODE_MASK: u32 = 0b10111111100000001111110000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SCVTF_asimdshf_C";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SCVTF_asimdshf_C {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SCVTF_asimdshf_C {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b0011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b111001u32 << 10u32
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
pub mod FCVTZS_asimdshf_C {
    pub const OPCODE_MASK: u32 = 0b10111111100000001111110000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001111110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FCVTZS_asimdshf_C";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVTZS_asimdshf_C {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVTZS_asimdshf_C {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b0011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b111111u32 << 10u32
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
pub mod USHR_asimdshf_R {
    pub const OPCODE_MASK: u32 = 0b10111111100000001100110000000000u32;
    pub const OPCODE: u32 = 0b00101111000000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "USHR_asimdshf_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct USHR_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl USHR_asimdshf_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            o1: ::aarchmrs_types::BitValue<1>,
            o0: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                o1,
                o0,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b1011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.o1.into_inner() << 13u32
                    | self.o0.into_inner() << 12u32
                    | 0b01u32 << 10u32
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
pub mod USRA_asimdshf_R {
    pub const OPCODE_MASK: u32 = 0b10111111100000001100110000000000u32;
    pub const OPCODE: u32 = 0b00101111000000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "USRA_asimdshf_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct USRA_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl USRA_asimdshf_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            o1: ::aarchmrs_types::BitValue<1>,
            o0: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                o1,
                o0,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b1011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.o1.into_inner() << 13u32
                    | self.o0.into_inner() << 12u32
                    | 0b01u32 << 10u32
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
pub mod URSHR_asimdshf_R {
    pub const OPCODE_MASK: u32 = 0b10111111100000001100110000000000u32;
    pub const OPCODE: u32 = 0b00101111000000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "URSHR_asimdshf_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct URSHR_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl URSHR_asimdshf_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            o1: ::aarchmrs_types::BitValue<1>,
            o0: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                o1,
                o0,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b1011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.o1.into_inner() << 13u32
                    | self.o0.into_inner() << 12u32
                    | 0b01u32 << 10u32
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
pub mod URSRA_asimdshf_R {
    pub const OPCODE_MASK: u32 = 0b10111111100000001100110000000000u32;
    pub const OPCODE: u32 = 0b00101111000000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "URSRA_asimdshf_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct URSRA_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub o0: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl URSRA_asimdshf_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            o1: ::aarchmrs_types::BitValue<1>,
            o0: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                o1,
                o0,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b1011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.o1.into_inner() << 13u32
                    | self.o0.into_inner() << 12u32
                    | 0b01u32 << 10u32
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
pub mod SRI_asimdshf_R {
    pub const OPCODE_MASK: u32 = 0b10111111100000001111110000000000u32;
    pub const OPCODE: u32 = 0b00101111000000000100010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SRI_asimdshf_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SRI_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SRI_asimdshf_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b1011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b010001u32 << 10u32
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
pub mod SLI_asimdshf_R {
    pub const OPCODE_MASK: u32 = 0b10111111100000001111110000000000u32;
    pub const OPCODE: u32 = 0b00101111000000000101010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SLI_asimdshf_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SLI_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SLI_asimdshf_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b1011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b010101u32 << 10u32
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
pub mod SQSHLU_asimdshf_R {
    pub const OPCODE_MASK: u32 = 0b10111111100000001110110000000000u32;
    pub const OPCODE: u32 = 0b00101111000000000110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SQSHLU_asimdshf_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQSHLU_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQSHLU_asimdshf_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            op: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                op,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b1011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b011u32 << 13u32
                    | self.op.into_inner() << 12u32
                    | 0b01u32 << 10u32
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
pub mod UQSHL_asimdshf_R {
    pub const OPCODE_MASK: u32 = 0b10111111100000001110110000000000u32;
    pub const OPCODE: u32 = 0b00101111000000000110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "UQSHL_asimdshf_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UQSHL_asimdshf_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UQSHL_asimdshf_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            op: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                op,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b1011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b011u32 << 13u32
                    | self.op.into_inner() << 12u32
                    | 0b01u32 << 10u32
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
pub mod SQSHRUN_asimdshf_N {
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    pub const OPCODE: u32 = 0b00101111000000001000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SQSHRUN_asimdshf_N";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQSHRUN_asimdshf_N {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQSHRUN_asimdshf_N {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            op: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                op,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b1011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b1000u32 << 12u32
                    | self.op.into_inner() << 11u32
                    | 0b1u32 << 10u32
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
pub mod SQRSHRUN_asimdshf_N {
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    pub const OPCODE: u32 = 0b00101111000000001000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SQRSHRUN_asimdshf_N";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQRSHRUN_asimdshf_N {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQRSHRUN_asimdshf_N {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            op: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                op,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b1011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b1000u32 << 12u32
                    | self.op.into_inner() << 11u32
                    | 0b1u32 << 10u32
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
pub mod UQSHRN_asimdshf_N {
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    pub const OPCODE: u32 = 0b00101111000000001001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "UQSHRN_asimdshf_N";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UQSHRN_asimdshf_N {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UQSHRN_asimdshf_N {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            op: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                op,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b1011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b1001u32 << 12u32
                    | self.op.into_inner() << 11u32
                    | 0b1u32 << 10u32
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
pub mod UQRSHRN_asimdshf_N {
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    pub const OPCODE: u32 = 0b00101111000000001001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "UQRSHRN_asimdshf_N";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UQRSHRN_asimdshf_N {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UQRSHRN_asimdshf_N {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            op: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                op,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b1011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b1001u32 << 12u32
                    | self.op.into_inner() << 11u32
                    | 0b1u32 << 10u32
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
pub mod USHLL_asimdshf_L {
    pub const OPCODE_MASK: u32 = 0b10111111100000001111110000000000u32;
    pub const OPCODE: u32 = 0b00101111000000001010010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "USHLL_asimdshf_L";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct USHLL_asimdshf_L {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl USHLL_asimdshf_L {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b1011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b101001u32 << 10u32
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
pub mod UCVTF_asimdshf_C {
    pub const OPCODE_MASK: u32 = 0b10111111100000001111110000000000u32;
    pub const OPCODE: u32 = 0b00101111000000001110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "UCVTF_asimdshf_C";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UCVTF_asimdshf_C {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UCVTF_asimdshf_C {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b1011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b111001u32 << 10u32
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
pub mod FCVTZU_asimdshf_C {
    pub const OPCODE_MASK: u32 = 0b10111111100000001111110000000000u32;
    pub const OPCODE: u32 = 0b00101111000000001111110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FCVTZU_asimdshf_C";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVTZU_asimdshf_C {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub immh: ::aarchmrs_types::BitValue<4>,
        pub immb: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVTZU_asimdshf_C {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            immh: ::aarchmrs_types::BitValue<4>,
            immb: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                immh,
                immb,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b1011110u32 << 23u32
                    | self.immh.into_inner() << 19u32
                    | self.immb.into_inner() << 16u32
                    | 0b111111u32 << 10u32
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
