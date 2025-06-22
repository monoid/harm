/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod RBIT_32_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    pub const OPCODE: u32 = 0b01011010110000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "RBIT_32_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RBIT_32_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl RBIT_32_dp_1src {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101101011000000000000u32 << 10u32
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
pub mod REV16_32_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000000000000u32;
    pub const OPCODE: u32 = 0b01011010110000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "REV16_32_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct REV16_32_dp_1src {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl REV16_32_dp_1src {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011010110000000000u32 << 12u32
                    | self.opc.into_inner() << 10u32
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
pub mod REV_32_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000000000000u32;
    pub const OPCODE: u32 = 0b01011010110000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "REV_32_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct REV_32_dp_1src {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl REV_32_dp_1src {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011010110000000000u32 << 12u32
                    | self.opc.into_inner() << 10u32
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
pub mod CLZ_32_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111100000000000u32;
    pub const OPCODE: u32 = 0b01011010110000000001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CLZ_32_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CLZ_32_dp_1src {
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CLZ_32_dp_1src {
        #[inline]
        pub const fn new(
            op: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { op, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010110101100000000010u32 << 11u32
                    | self.op.into_inner() << 10u32
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
pub mod CLS_32_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111100000000000u32;
    pub const OPCODE: u32 = 0b01011010110000000001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CLS_32_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CLS_32_dp_1src {
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CLS_32_dp_1src {
        #[inline]
        pub const fn new(
            op: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { op, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010110101100000000010u32 << 11u32
                    | self.op.into_inner() << 10u32
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
pub mod CTZ_32_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    pub const OPCODE: u32 = 0b01011010110000000001100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CTZ_32_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CTZ_32_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CTZ_32_dp_1src {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101101011000000000110u32 << 10u32
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
pub mod CNT_32_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    pub const OPCODE: u32 = 0b01011010110000000001110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CNT_32_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CNT_32_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CNT_32_dp_1src {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101101011000000000111u32 << 10u32
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
pub mod ABS_32_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    pub const OPCODE: u32 = 0b01011010110000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ABS_32_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ABS_32_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ABS_32_dp_1src {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101101011000000001000u32 << 10u32
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
pub mod RBIT_64_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    pub const OPCODE: u32 = 0b11011010110000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "RBIT_64_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RBIT_64_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl RBIT_64_dp_1src {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101101011000000000000u32 << 10u32
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
pub mod REV16_64_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000000000000u32;
    pub const OPCODE: u32 = 0b11011010110000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "REV16_64_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct REV16_64_dp_1src {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl REV16_64_dp_1src {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000000000u32 << 12u32
                    | self.opc.into_inner() << 10u32
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
pub mod REV32_64_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000000000000u32;
    pub const OPCODE: u32 = 0b11011010110000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "REV32_64_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct REV32_64_dp_1src {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl REV32_64_dp_1src {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000000000u32 << 12u32
                    | self.opc.into_inner() << 10u32
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
pub mod REV_64_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111000000000000u32;
    pub const OPCODE: u32 = 0b11011010110000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "REV_64_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct REV_64_dp_1src {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl REV_64_dp_1src {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000000000u32 << 12u32
                    | self.opc.into_inner() << 10u32
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
pub mod CLZ_64_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111100000000000u32;
    pub const OPCODE: u32 = 0b11011010110000000001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CLZ_64_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CLZ_64_dp_1src {
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CLZ_64_dp_1src {
        #[inline]
        pub const fn new(
            op: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { op, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000000010u32 << 11u32
                    | self.op.into_inner() << 10u32
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
pub mod CLS_64_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111100000000000u32;
    pub const OPCODE: u32 = 0b11011010110000000001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CLS_64_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CLS_64_dp_1src {
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CLS_64_dp_1src {
        #[inline]
        pub const fn new(
            op: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { op, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000000010u32 << 11u32
                    | self.op.into_inner() << 10u32
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
pub mod CTZ_64_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    pub const OPCODE: u32 = 0b11011010110000000001100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CTZ_64_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CTZ_64_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CTZ_64_dp_1src {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101101011000000000110u32 << 10u32
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
pub mod CNT_64_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    pub const OPCODE: u32 = 0b11011010110000000001110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CNT_64_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CNT_64_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CNT_64_dp_1src {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101101011000000000111u32 << 10u32
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
pub mod ABS_64_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000000u32;
    pub const OPCODE: u32 = 0b11011010110000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ABS_64_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ABS_64_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ABS_64_dp_1src {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101101011000000001000u32 << 10u32
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
pub mod PACIA_64P_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111101110000000000u32;
    pub const OPCODE: u32 = 0b11011010110000010000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "PACIA_64P_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIA_64P_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl PACIA_64P_dp_1src {
        #[inline]
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Z, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | self.Z.into_inner() << 13u32
                    | 0b000u32 << 10u32
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
pub mod PACIB_64P_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111101110000000000u32;
    pub const OPCODE: u32 = 0b11011010110000010000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "PACIB_64P_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIB_64P_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl PACIB_64P_dp_1src {
        #[inline]
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Z, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | self.Z.into_inner() << 13u32
                    | 0b001u32 << 10u32
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
pub mod PACDA_64P_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111101110000000000u32;
    pub const OPCODE: u32 = 0b11011010110000010000100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "PACDA_64P_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACDA_64P_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl PACDA_64P_dp_1src {
        #[inline]
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Z, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | self.Z.into_inner() << 13u32
                    | 0b010u32 << 10u32
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
pub mod PACDB_64P_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111101110000000000u32;
    pub const OPCODE: u32 = 0b11011010110000010000110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "PACDB_64P_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACDB_64P_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl PACDB_64P_dp_1src {
        #[inline]
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Z, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | self.Z.into_inner() << 13u32
                    | 0b011u32 << 10u32
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
pub mod AUTIA_64P_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111101110000000000u32;
    pub const OPCODE: u32 = 0b11011010110000010001000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "AUTIA_64P_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIA_64P_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTIA_64P_dp_1src {
        #[inline]
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Z, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | self.Z.into_inner() << 13u32
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
pub mod AUTIB_64P_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111101110000000000u32;
    pub const OPCODE: u32 = 0b11011010110000010001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "AUTIB_64P_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIB_64P_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTIB_64P_dp_1src {
        #[inline]
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Z, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | self.Z.into_inner() << 13u32
                    | 0b101u32 << 10u32
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
pub mod AUTDA_64P_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111101110000000000u32;
    pub const OPCODE: u32 = 0b11011010110000010001100000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "AUTDA_64P_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTDA_64P_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTDA_64P_dp_1src {
        #[inline]
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Z, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | self.Z.into_inner() << 13u32
                    | 0b110u32 << 10u32
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
pub mod AUTDB_64P_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111101110000000000u32;
    pub const OPCODE: u32 = 0b11011010110000010001110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "AUTDB_64P_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTDB_64P_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTDB_64P_dp_1src {
        #[inline]
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Z, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | self.Z.into_inner() << 13u32
                    | 0b111u32 << 10u32
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
pub mod PACIZA_64Z_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111101111111100000u32;
    pub const OPCODE: u32 = 0b11011010110000010000001111100000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "PACIZA_64Z_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIZA_64Z_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl PACIZA_64Z_dp_1src {
        #[inline]
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Z, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | self.Z.into_inner() << 13u32
                    | 0b00011111u32 << 5u32
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
pub mod PACIZB_64Z_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111101111111100000u32;
    pub const OPCODE: u32 = 0b11011010110000010000011111100000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "PACIZB_64Z_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIZB_64Z_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl PACIZB_64Z_dp_1src {
        #[inline]
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Z, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | self.Z.into_inner() << 13u32
                    | 0b00111111u32 << 5u32
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
pub mod PACDZA_64Z_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111101111111100000u32;
    pub const OPCODE: u32 = 0b11011010110000010000101111100000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "PACDZA_64Z_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACDZA_64Z_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl PACDZA_64Z_dp_1src {
        #[inline]
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Z, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | self.Z.into_inner() << 13u32
                    | 0b01011111u32 << 5u32
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
pub mod PACDZB_64Z_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111101111111100000u32;
    pub const OPCODE: u32 = 0b11011010110000010000111111100000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "PACDZB_64Z_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACDZB_64Z_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl PACDZB_64Z_dp_1src {
        #[inline]
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Z, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | self.Z.into_inner() << 13u32
                    | 0b01111111u32 << 5u32
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
pub mod AUTIZA_64Z_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111101111111100000u32;
    pub const OPCODE: u32 = 0b11011010110000010001001111100000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "AUTIZA_64Z_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIZA_64Z_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTIZA_64Z_dp_1src {
        #[inline]
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Z, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | self.Z.into_inner() << 13u32
                    | 0b10011111u32 << 5u32
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
pub mod AUTIZB_64Z_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111101111111100000u32;
    pub const OPCODE: u32 = 0b11011010110000010001011111100000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "AUTIZB_64Z_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIZB_64Z_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTIZB_64Z_dp_1src {
        #[inline]
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Z, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | self.Z.into_inner() << 13u32
                    | 0b10111111u32 << 5u32
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
pub mod AUTDZA_64Z_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111101111111100000u32;
    pub const OPCODE: u32 = 0b11011010110000010001101111100000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "AUTDZA_64Z_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTDZA_64Z_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTDZA_64Z_dp_1src {
        #[inline]
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Z, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | self.Z.into_inner() << 13u32
                    | 0b11011111u32 << 5u32
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
pub mod AUTDZB_64Z_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111101111111100000u32;
    pub const OPCODE: u32 = 0b11011010110000010001111111100000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "AUTDZB_64Z_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTDZB_64Z_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTDZB_64Z_dp_1src {
        #[inline]
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Z, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | self.Z.into_inner() << 13u32
                    | 0b11111111u32 << 5u32
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
pub mod XPACI_64Z_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111101111100000u32;
    pub const OPCODE: u32 = 0b11011010110000010100001111100000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "XPACI_64Z_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct XPACI_64Z_dp_1src {
        pub D: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl XPACI_64Z_dp_1src {
        #[inline]
        pub const fn new(
            D: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { D, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000101000u32 << 11u32
                    | self.D.into_inner() << 10u32
                    | 0b11111u32 << 5u32
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
pub mod XPACD_64Z_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111101111100000u32;
    pub const OPCODE: u32 = 0b11011010110000010100001111100000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "XPACD_64Z_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct XPACD_64Z_dp_1src {
        pub D: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl XPACD_64Z_dp_1src {
        #[inline]
        pub const fn new(
            D: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { D, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000101000u32 << 11u32
                    | self.D.into_inner() << 10u32
                    | 0b11111u32 << 5u32
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
pub mod PACNBIASPPC_64LR_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    pub const OPCODE: u32 = 0b11011010110000011000001111111110u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "PACNBIASPPC_64LR_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACNBIASPPC_64LR_dp_1src {}
    impl PACNBIASPPC_64LR_dp_1src {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000011000001111111110u32 << 0u32,
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
pub mod PACNBIBSPPC_64LR_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    pub const OPCODE: u32 = 0b11011010110000011000011111111110u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "PACNBIBSPPC_64LR_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACNBIBSPPC_64LR_dp_1src {}
    impl PACNBIBSPPC_64LR_dp_1src {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000011000011111111110u32 << 0u32,
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
pub mod PACIA171615_64LR_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    pub const OPCODE: u32 = 0b11011010110000011000101111111110u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "PACIA171615_64LR_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIA171615_64LR_dp_1src {}
    impl PACIA171615_64LR_dp_1src {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000011000101111111110u32 << 0u32,
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
pub mod PACIB171615_64LR_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    pub const OPCODE: u32 = 0b11011010110000011000111111111110u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "PACIB171615_64LR_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIB171615_64LR_dp_1src {}
    impl PACIB171615_64LR_dp_1src {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000011000111111111110u32 << 0u32,
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
pub mod AUTIASPPCR_64LRR_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000011111u32;
    pub const OPCODE: u32 = 0b11011010110000011001000000011110u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "AUTIASPPCR_64LRR_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIASPPCR_64LRR_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTIASPPCR_64LRR_dp_1src {
        #[inline]
        pub const fn new(Rn: ::aarchmrs_types::BitValue<5>) -> Self {
            Self { Rn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101101011000001100100u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b11110u32 << 0u32,
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
pub mod AUTIBSPPCR_64LRR_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000011111u32;
    pub const OPCODE: u32 = 0b11011010110000011001010000011110u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "AUTIBSPPCR_64LRR_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIBSPPCR_64LRR_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTIBSPPCR_64LRR_dp_1src {
        #[inline]
        pub const fn new(Rn: ::aarchmrs_types::BitValue<5>) -> Self {
            Self { Rn }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101101011000001100101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b11110u32 << 0u32,
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
pub mod PACIASPPC_64LR_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    pub const OPCODE: u32 = 0b11011010110000011010001111111110u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "PACIASPPC_64LR_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIASPPC_64LR_dp_1src {}
    impl PACIASPPC_64LR_dp_1src {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000011010001111111110u32 << 0u32,
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
pub mod PACIBSPPC_64LR_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    pub const OPCODE: u32 = 0b11011010110000011010011111111110u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "PACIBSPPC_64LR_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIBSPPC_64LR_dp_1src {}
    impl PACIBSPPC_64LR_dp_1src {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000011010011111111110u32 << 0u32,
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
pub mod AUTIA171615_64LR_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    pub const OPCODE: u32 = 0b11011010110000011011101111111110u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "AUTIA171615_64LR_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIA171615_64LR_dp_1src {}
    impl AUTIA171615_64LR_dp_1src {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000011011101111111110u32 << 0u32,
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
pub mod AUTIB171615_64LR_dp_1src {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    pub const OPCODE: u32 = 0b11011010110000011011111111111110u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "AUTIB171615_64LR_dp_1src";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIB171615_64LR_dp_1src {}
    impl AUTIB171615_64LR_dp_1src {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000011011111111111110u32 << 0u32,
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
