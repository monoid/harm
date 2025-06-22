/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FCMP_S_floatcmp {
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000111u32;
    pub const OPCODE: u32 = 0b00011110001000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FCMP_S_floatcmp";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMP_S_floatcmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMP_S_floatcmp {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rm, Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
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
pub mod FCMP_SZ_floatcmp {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000111u32;
    pub const OPCODE: u32 = 0b00011110001000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    pub const NAME: &str = "FCMP_SZ_floatcmp";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMP_SZ_floatcmp {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMP_SZ_floatcmp {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0001111000100000001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
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
pub mod FCMPE_S_floatcmp {
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000111u32;
    pub const OPCODE: u32 = 0b00011110001000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FCMPE_S_floatcmp";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMPE_S_floatcmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMPE_S_floatcmp {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rm, Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
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
pub mod FCMPE_SZ_floatcmp {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000111u32;
    pub const OPCODE: u32 = 0b00011110001000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    pub const NAME: &str = "FCMPE_SZ_floatcmp";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMPE_SZ_floatcmp {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMPE_SZ_floatcmp {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0001111000100000001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
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
pub mod FCMP_D_floatcmp {
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000111u32;
    pub const OPCODE: u32 = 0b00011110011000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FCMP_D_floatcmp";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMP_D_floatcmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMP_D_floatcmp {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rm, Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
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
pub mod FCMP_DZ_floatcmp {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000111u32;
    pub const OPCODE: u32 = 0b00011110011000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    pub const NAME: &str = "FCMP_DZ_floatcmp";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMP_DZ_floatcmp {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMP_DZ_floatcmp {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0001111001100000001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
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
pub mod FCMPE_D_floatcmp {
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000111u32;
    pub const OPCODE: u32 = 0b00011110011000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FCMPE_D_floatcmp";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMPE_D_floatcmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMPE_D_floatcmp {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rm, Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
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
pub mod FCMPE_DZ_floatcmp {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000111u32;
    pub const OPCODE: u32 = 0b00011110011000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    pub const NAME: &str = "FCMPE_DZ_floatcmp";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMPE_DZ_floatcmp {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMPE_DZ_floatcmp {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0001111001100000001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
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
pub mod FCMP_H_floatcmp {
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000111u32;
    pub const OPCODE: u32 = 0b00011110111000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FCMP_H_floatcmp";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMP_H_floatcmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMP_H_floatcmp {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rm, Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110111u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
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
pub mod FCMP_HZ_floatcmp {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000111u32;
    pub const OPCODE: u32 = 0b00011110111000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    pub const NAME: &str = "FCMP_HZ_floatcmp";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMP_HZ_floatcmp {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMP_HZ_floatcmp {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0001111011100000001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
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
pub mod FCMPE_H_floatcmp {
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000111u32;
    pub const OPCODE: u32 = 0b00011110111000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FCMPE_H_floatcmp";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMPE_H_floatcmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMPE_H_floatcmp {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rm, Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110111u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
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
pub mod FCMPE_HZ_floatcmp {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000111u32;
    pub const OPCODE: u32 = 0b00011110111000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    pub const NAME: &str = "FCMPE_HZ_floatcmp";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCMPE_HZ_floatcmp {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub opc: ::aarchmrs_types::BitValue<2>,
    }
    impl FCMPE_HZ_floatcmp {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            opc: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self { Rn, opc }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0001111011100000001000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.opc.into_inner() << 3u32
                    | 0b000u32 << 0u32,
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
