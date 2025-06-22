/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SMLAL_asimdelem_L {
    pub const OPCODE_MASK: u32 = 0b10111111000000001011010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SMLAL_asimdelem_L";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SMLAL_asimdelem_L {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SMLAL_asimdelem_L {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            o2: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                size,
                L,
                M,
                Rm,
                o2,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.o2.into_inner() << 14u32
                    | 0b10u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod SQDMLAL_asimdelem_L {
    pub const OPCODE_MASK: u32 = 0b10111111000000001011010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000000011000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SQDMLAL_asimdelem_L";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQDMLAL_asimdelem_L {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQDMLAL_asimdelem_L {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            o2: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                size,
                L,
                M,
                Rm,
                o2,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.o2.into_inner() << 14u32
                    | 0b11u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod SMLSL_asimdelem_L {
    pub const OPCODE_MASK: u32 = 0b10111111000000001011010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SMLSL_asimdelem_L";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SMLSL_asimdelem_L {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SMLSL_asimdelem_L {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            o2: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                size,
                L,
                M,
                Rm,
                o2,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.o2.into_inner() << 14u32
                    | 0b10u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod SQDMLSL_asimdelem_L {
    pub const OPCODE_MASK: u32 = 0b10111111000000001011010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000000011000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SQDMLSL_asimdelem_L";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQDMLSL_asimdelem_L {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQDMLSL_asimdelem_L {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            o2: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                size,
                L,
                M,
                Rm,
                o2,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.o2.into_inner() << 14u32
                    | 0b11u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod MUL_asimdelem_R {
    pub const OPCODE_MASK: u32 = 0b10111111000000001111010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MUL_asimdelem_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MUL_asimdelem_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MUL_asimdelem_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                size,
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1000u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod SMULL_asimdelem_L {
    pub const OPCODE_MASK: u32 = 0b10111111000000001111010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SMULL_asimdelem_L";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SMULL_asimdelem_L {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SMULL_asimdelem_L {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                size,
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1010u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod SQDMULL_asimdelem_L {
    pub const OPCODE_MASK: u32 = 0b10111111000000001111010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001011000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SQDMULL_asimdelem_L";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQDMULL_asimdelem_L {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQDMULL_asimdelem_L {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                size,
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1011u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod SQDMULH_asimdelem_R {
    pub const OPCODE_MASK: u32 = 0b10111111000000001110010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SQDMULH_asimdelem_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQDMULH_asimdelem_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQDMULH_asimdelem_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            op: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                size,
                L,
                M,
                Rm,
                op,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b110u32 << 13u32
                    | self.op.into_inner() << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod SQRDMULH_asimdelem_R {
    pub const OPCODE_MASK: u32 = 0b10111111000000001110010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SQRDMULH_asimdelem_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQRDMULH_asimdelem_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQRDMULH_asimdelem_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            op: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                size,
                L,
                M,
                Rm,
                op,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b110u32 << 13u32
                    | self.op.into_inner() << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod SDOT_asimdelem_D {
    pub const OPCODE_MASK: u32 = 0b10111111000000001111010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001110000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SDOT_asimdelem_D";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SDOT_asimdelem_D {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SDOT_asimdelem_D {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                size,
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1110u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FDOT_asimdelem_D {
    pub const OPCODE_MASK: u32 = 0b10111111110000001111010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FDOT_asimdelem_D";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FDOT_asimdelem_D {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FDOT_asimdelem_D {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b00111100u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0000u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FMLA_asimdelem_RH_H {
    pub const OPCODE_MASK: u32 = 0b10111111110000001011010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000000001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMLA_asimdelem_RH_H";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLA_asimdelem_RH_H {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLA_asimdelem_RH_H {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            o2: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                L,
                M,
                Rm,
                o2,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b00111100u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.o2.into_inner() << 14u32
                    | 0b01u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FMLS_asimdelem_RH_H {
    pub const OPCODE_MASK: u32 = 0b10111111110000001011010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000000001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMLS_asimdelem_RH_H";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLS_asimdelem_RH_H {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLS_asimdelem_RH_H {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            o2: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                L,
                M,
                Rm,
                o2,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b00111100u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.o2.into_inner() << 14u32
                    | 0b01u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FMUL_asimdelem_RH_H {
    pub const OPCODE_MASK: u32 = 0b10111111110000001111010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMUL_asimdelem_RH_H";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMUL_asimdelem_RH_H {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMUL_asimdelem_RH_H {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b00111100u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1001u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod SUDOT_asimdelem_D {
    pub const OPCODE_MASK: u32 = 0b10111111010000001111010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SUDOT_asimdelem_D";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUDOT_asimdelem_D {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub US: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUDOT_asimdelem_D {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            US: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                US,
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001111u32 << 24u32
                    | self.US.into_inner() << 23u32
                    | 0b0u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1111u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FDOT_asimdelem_G {
    pub const OPCODE_MASK: u32 = 0b10111111110000001111010000000000u32;
    pub const OPCODE: u32 = 0b00001111010000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FDOT_asimdelem_G";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FDOT_asimdelem_G {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FDOT_asimdelem_G {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b00111101u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0000u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod BFDOT_asimdelem_E {
    pub const OPCODE_MASK: u32 = 0b10111111110000001111010000000000u32;
    pub const OPCODE: u32 = 0b00001111010000001111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BFDOT_asimdelem_E";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BFDOT_asimdelem_E {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl BFDOT_asimdelem_E {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b00111101u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1111u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FMLA_asimdelem_R_SD {
    pub const OPCODE_MASK: u32 = 0b10111111100000001011010000000000u32;
    pub const OPCODE: u32 = 0b00001111100000000001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMLA_asimdelem_R_SD";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLA_asimdelem_R_SD {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLA_asimdelem_R_SD {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            sz: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            o2: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                sz,
                L,
                M,
                Rm,
                o2,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b0011111u32 << 23u32
                    | self.sz.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.o2.into_inner() << 14u32
                    | 0b01u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FMLS_asimdelem_R_SD {
    pub const OPCODE_MASK: u32 = 0b10111111100000001011010000000000u32;
    pub const OPCODE: u32 = 0b00001111100000000001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMLS_asimdelem_R_SD";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLS_asimdelem_R_SD {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLS_asimdelem_R_SD {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            sz: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            o2: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                sz,
                L,
                M,
                Rm,
                o2,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b0011111u32 << 23u32
                    | self.sz.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.o2.into_inner() << 14u32
                    | 0b01u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FMUL_asimdelem_R_SD {
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    pub const OPCODE: u32 = 0b00001111100000001001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMUL_asimdelem_R_SD";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMUL_asimdelem_R_SD {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMUL_asimdelem_R_SD {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            sz: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                sz,
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b0011111u32 << 23u32
                    | self.sz.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1001u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FMLAL_asimdelem_LH {
    pub const OPCODE_MASK: u32 = 0b10111111100000001011010000000000u32;
    pub const OPCODE: u32 = 0b00001111100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMLAL_asimdelem_LH";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLAL_asimdelem_LH {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLAL_asimdelem_LH {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            sz: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            S: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                sz,
                L,
                M,
                Rm,
                S,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b0011111u32 << 23u32
                    | self.sz.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.S.into_inner() << 14u32
                    | 0b00u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FMLSL_asimdelem_LH {
    pub const OPCODE_MASK: u32 = 0b10111111100000001011010000000000u32;
    pub const OPCODE: u32 = 0b00001111100000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMLSL_asimdelem_LH";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLSL_asimdelem_LH {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLSL_asimdelem_LH {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            sz: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            S: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                sz,
                L,
                M,
                Rm,
                S,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b0011111u32 << 23u32
                    | self.sz.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.S.into_inner() << 14u32
                    | 0b00u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod USDOT_asimdelem_D {
    pub const OPCODE_MASK: u32 = 0b10111111010000001111010000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "USDOT_asimdelem_D";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct USDOT_asimdelem_D {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub US: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl USDOT_asimdelem_D {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            US: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                US,
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b001111u32 << 24u32
                    | self.US.into_inner() << 23u32
                    | 0b0u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1111u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod BFMLAL_asimdelem_F {
    pub const OPCODE_MASK: u32 = 0b10111111110000001111010000000000u32;
    pub const OPCODE: u32 = 0b00001111110000001111000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BFMLAL_asimdelem_F";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BFMLAL_asimdelem_F {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl BFMLAL_asimdelem_F {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b00111111u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1111u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod MLA_asimdelem_R {
    pub const OPCODE_MASK: u32 = 0b10111111000000001011010000000000u32;
    pub const OPCODE: u32 = 0b00101111000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MLA_asimdelem_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MLA_asimdelem_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MLA_asimdelem_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            o2: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                size,
                L,
                M,
                Rm,
                o2,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b101111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.o2.into_inner() << 14u32
                    | 0b00u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod UMLAL_asimdelem_L {
    pub const OPCODE_MASK: u32 = 0b10111111000000001011010000000000u32;
    pub const OPCODE: u32 = 0b00101111000000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "UMLAL_asimdelem_L";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UMLAL_asimdelem_L {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UMLAL_asimdelem_L {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            o2: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                size,
                L,
                M,
                Rm,
                o2,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b101111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.o2.into_inner() << 14u32
                    | 0b10u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod MLS_asimdelem_R {
    pub const OPCODE_MASK: u32 = 0b10111111000000001011010000000000u32;
    pub const OPCODE: u32 = 0b00101111000000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MLS_asimdelem_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MLS_asimdelem_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MLS_asimdelem_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            o2: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                size,
                L,
                M,
                Rm,
                o2,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b101111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.o2.into_inner() << 14u32
                    | 0b00u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod UMLSL_asimdelem_L {
    pub const OPCODE_MASK: u32 = 0b10111111000000001011010000000000u32;
    pub const OPCODE: u32 = 0b00101111000000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "UMLSL_asimdelem_L";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UMLSL_asimdelem_L {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub o2: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UMLSL_asimdelem_L {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            o2: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                size,
                L,
                M,
                Rm,
                o2,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b101111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.o2.into_inner() << 14u32
                    | 0b10u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod UMULL_asimdelem_L {
    pub const OPCODE_MASK: u32 = 0b10111111000000001111010000000000u32;
    pub const OPCODE: u32 = 0b00101111000000001010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "UMULL_asimdelem_L";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UMULL_asimdelem_L {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UMULL_asimdelem_L {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                size,
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b101111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1010u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod SQRDMLAH_asimdelem_R {
    pub const OPCODE_MASK: u32 = 0b10111111000000001101010000000000u32;
    pub const OPCODE: u32 = 0b00101111000000001101000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SQRDMLAH_asimdelem_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQRDMLAH_asimdelem_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQRDMLAH_asimdelem_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            S: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                size,
                L,
                M,
                Rm,
                S,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b101111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b11u32 << 14u32
                    | self.S.into_inner() << 13u32
                    | 0b1u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod UDOT_asimdelem_D {
    pub const OPCODE_MASK: u32 = 0b10111111000000001111010000000000u32;
    pub const OPCODE: u32 = 0b00101111000000001110000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "UDOT_asimdelem_D";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UDOT_asimdelem_D {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UDOT_asimdelem_D {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                size,
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b101111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1110u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod SQRDMLSH_asimdelem_R {
    pub const OPCODE_MASK: u32 = 0b10111111000000001101010000000000u32;
    pub const OPCODE: u32 = 0b00101111000000001101000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SQRDMLSH_asimdelem_R";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SQRDMLSH_asimdelem_R {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SQRDMLSH_asimdelem_R {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            S: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                size,
                L,
                M,
                Rm,
                S,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b101111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b11u32 << 14u32
                    | self.S.into_inner() << 13u32
                    | 0b1u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FMULX_asimdelem_RH_H {
    pub const OPCODE_MASK: u32 = 0b10111111110000001111010000000000u32;
    pub const OPCODE: u32 = 0b00101111000000001001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMULX_asimdelem_RH_H";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMULX_asimdelem_RH_H {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMULX_asimdelem_RH_H {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b10111100u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1001u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FCMLA_advsimd_elt {
    pub const OPCODE_MASK: u32 = 0b10111111000000001001010000000000u32;
    pub const OPCODE: u32 = 0b00101111000000000001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FCMLA_advsimd_elt";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMLA_advsimd_elt {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub rot: ::aarchmrs_types::BitValue<2>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCMLA_advsimd_elt {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            size: ::aarchmrs_types::BitValue<2>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            rot: ::aarchmrs_types::BitValue<2>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                size,
                L,
                M,
                Rm,
                rot,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b101111u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.rot.into_inner() << 13u32
                    | 0b1u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FMULX_asimdelem_R_SD {
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    pub const OPCODE: u32 = 0b00101111100000001001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMULX_asimdelem_R_SD";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMULX_asimdelem_R_SD {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMULX_asimdelem_R_SD {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            sz: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                sz,
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b1011111u32 << 23u32
                    | self.sz.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1001u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FMLAL2_asimdelem_LH {
    pub const OPCODE_MASK: u32 = 0b10111111100000001011010000000000u32;
    pub const OPCODE: u32 = 0b00101111100000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMLAL2_asimdelem_LH";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLAL2_asimdelem_LH {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLAL2_asimdelem_LH {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            sz: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            S: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                sz,
                L,
                M,
                Rm,
                S,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b1011111u32 << 23u32
                    | self.sz.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.S.into_inner() << 14u32
                    | 0b00u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FMLSL2_asimdelem_LH {
    pub const OPCODE_MASK: u32 = 0b10111111100000001011010000000000u32;
    pub const OPCODE: u32 = 0b00101111100000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMLSL2_asimdelem_LH";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLSL2_asimdelem_LH {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLSL2_asimdelem_LH {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            sz: ::aarchmrs_types::BitValue<1>,
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            S: ::aarchmrs_types::BitValue<1>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                sz,
                L,
                M,
                Rm,
                S,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b1011111u32 << 23u32
                    | self.sz.into_inner() << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.S.into_inner() << 14u32
                    | 0b00u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FMLALB_asimdelem_H {
    pub const OPCODE_MASK: u32 = 0b11111111110000001111010000000000u32;
    pub const OPCODE: u32 = 0b00001111110000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMLALB_asimdelem_H";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLALB_asimdelem_H {
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLALB_asimdelem_H {
        #[inline]
        pub const fn new(
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0000111111u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0000u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FMLALLBB_asimdelem_J {
    pub const OPCODE_MASK: u32 = 0b11111111110000001111010000000000u32;
    pub const OPCODE: u32 = 0b00101111000000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMLALLBB_asimdelem_J";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLALLBB_asimdelem_J {
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLALLBB_asimdelem_J {
        #[inline]
        pub const fn new(
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0010111100u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1000u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FMLALLBT_asimdelem_J {
    pub const OPCODE_MASK: u32 = 0b11111111110000001111010000000000u32;
    pub const OPCODE: u32 = 0b00101111010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMLALLBT_asimdelem_J";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLALLBT_asimdelem_J {
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLALLBT_asimdelem_J {
        #[inline]
        pub const fn new(
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0010111101u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1000u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FMLALT_asimdelem_H {
    pub const OPCODE_MASK: u32 = 0b11111111110000001111010000000000u32;
    pub const OPCODE: u32 = 0b01001111110000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMLALT_asimdelem_H";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLALT_asimdelem_H {
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLALT_asimdelem_H {
        #[inline]
        pub const fn new(
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0100111111u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b0000u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FMLALLTB_asimdelem_J {
    pub const OPCODE_MASK: u32 = 0b11111111110000001111010000000000u32;
    pub const OPCODE: u32 = 0b01101111000000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMLALLTB_asimdelem_J";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLALLTB_asimdelem_J {
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLALLTB_asimdelem_J {
        #[inline]
        pub const fn new(
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0110111100u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1000u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
pub mod FMLALLTT_asimdelem_J {
    pub const OPCODE_MASK: u32 = 0b11111111110000001111010000000000u32;
    pub const OPCODE: u32 = 0b01101111010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMLALLTT_asimdelem_J";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMLALLTT_asimdelem_J {
        pub L: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<4>,
        pub H: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMLALLTT_asimdelem_J {
        #[inline]
        pub const fn new(
            L: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<4>,
            H: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                L,
                M,
                Rm,
                H,
                Rn,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0110111101u32 << 22u32
                    | self.L.into_inner() << 21u32
                    | self.M.into_inner() << 20u32
                    | self.Rm.into_inner() << 16u32
                    | 0b1000u32 << 12u32
                    | self.H.into_inner() << 11u32
                    | 0b0u32 << 10u32
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
