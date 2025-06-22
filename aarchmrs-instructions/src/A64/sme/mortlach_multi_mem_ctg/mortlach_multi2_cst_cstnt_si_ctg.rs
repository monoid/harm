/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod st1b_mz_p_bi_2 {
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000001u32;
    pub const OPCODE: u32 = 0b10100000011000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "st1b_mz_p_bi_2";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1b_mz_p_bi_2 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<4>,
    }
    impl st1b_mz_p_bi_2 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm4,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000110u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 1u32
                    | 0b0u32 << 0u32,
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
pub mod stnt1b_mz_p_bi_2 {
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000001u32;
    pub const OPCODE: u32 = 0b10100000011000000000000000000001u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "stnt1b_mz_p_bi_2";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct stnt1b_mz_p_bi_2 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<4>,
    }
    impl stnt1b_mz_p_bi_2 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm4,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000110u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 1u32
                    | 0b1u32 << 0u32,
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
pub mod st1h_mz_p_bi_2 {
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000001u32;
    pub const OPCODE: u32 = 0b10100000011000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "st1h_mz_p_bi_2";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1h_mz_p_bi_2 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<4>,
    }
    impl st1h_mz_p_bi_2 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm4,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000110u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 1u32
                    | 0b0u32 << 0u32,
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
pub mod stnt1h_mz_p_bi_2 {
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000001u32;
    pub const OPCODE: u32 = 0b10100000011000000000000000000001u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "stnt1h_mz_p_bi_2";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct stnt1h_mz_p_bi_2 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<4>,
    }
    impl stnt1h_mz_p_bi_2 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm4,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000110u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 1u32
                    | 0b1u32 << 0u32,
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
pub mod st1w_mz_p_bi_2 {
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000001u32;
    pub const OPCODE: u32 = 0b10100000011000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "st1w_mz_p_bi_2";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1w_mz_p_bi_2 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<4>,
    }
    impl st1w_mz_p_bi_2 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm4,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000110u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 1u32
                    | 0b0u32 << 0u32,
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
pub mod stnt1w_mz_p_bi_2 {
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000001u32;
    pub const OPCODE: u32 = 0b10100000011000000000000000000001u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "stnt1w_mz_p_bi_2";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct stnt1w_mz_p_bi_2 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<4>,
    }
    impl stnt1w_mz_p_bi_2 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm4,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000110u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 1u32
                    | 0b1u32 << 0u32,
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
pub mod st1d_mz_p_bi_2 {
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000001u32;
    pub const OPCODE: u32 = 0b10100000011000000000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "st1d_mz_p_bi_2";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1d_mz_p_bi_2 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<4>,
    }
    impl st1d_mz_p_bi_2 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm4,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000110u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 1u32
                    | 0b0u32 << 0u32,
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
pub mod stnt1d_mz_p_bi_2 {
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000001u32;
    pub const OPCODE: u32 = 0b10100000011000000000000000000001u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "stnt1d_mz_p_bi_2";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct stnt1d_mz_p_bi_2 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<4>,
    }
    impl stnt1d_mz_p_bi_2 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                imm4,
                msz,
                PNg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101000000110u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 1u32
                    | 0b1u32 << 0u32,
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
