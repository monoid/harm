/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1b_mz_p_bi_4 {
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000011u32;
    pub const OPCODE: u32 = 0b10100000010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1b_mz_p_bi_4";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1b_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ld1b_mz_p_bi_4 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<3>,
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
                0b101000000100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 2u32
                    | 0b00u32 << 0u32,
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
pub mod ldnt1b_mz_p_bi_4 {
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000011u32;
    pub const OPCODE: u32 = 0b10100000010000001000000000000001u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ldnt1b_mz_p_bi_4";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1b_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ldnt1b_mz_p_bi_4 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<3>,
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
                0b101000000100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 2u32
                    | 0b01u32 << 0u32,
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
pub mod ld1h_mz_p_bi_4 {
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000011u32;
    pub const OPCODE: u32 = 0b10100000010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1h_mz_p_bi_4";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1h_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ld1h_mz_p_bi_4 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<3>,
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
                0b101000000100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 2u32
                    | 0b00u32 << 0u32,
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
pub mod ldnt1h_mz_p_bi_4 {
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000011u32;
    pub const OPCODE: u32 = 0b10100000010000001000000000000001u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ldnt1h_mz_p_bi_4";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1h_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ldnt1h_mz_p_bi_4 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<3>,
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
                0b101000000100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 2u32
                    | 0b01u32 << 0u32,
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
pub mod ld1w_mz_p_bi_4 {
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000011u32;
    pub const OPCODE: u32 = 0b10100000010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1w_mz_p_bi_4";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1w_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ld1w_mz_p_bi_4 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<3>,
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
                0b101000000100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 2u32
                    | 0b00u32 << 0u32,
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
pub mod ldnt1w_mz_p_bi_4 {
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000011u32;
    pub const OPCODE: u32 = 0b10100000010000001000000000000001u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ldnt1w_mz_p_bi_4";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1w_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ldnt1w_mz_p_bi_4 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<3>,
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
                0b101000000100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 2u32
                    | 0b01u32 << 0u32,
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
pub mod ld1d_mz_p_bi_4 {
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000011u32;
    pub const OPCODE: u32 = 0b10100000010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1d_mz_p_bi_4";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1d_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ld1d_mz_p_bi_4 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<3>,
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
                0b101000000100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 2u32
                    | 0b00u32 << 0u32,
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
pub mod ldnt1d_mz_p_bi_4 {
    pub const OPCODE_MASK: u32 = 0b11111111111100001000000000000011u32;
    pub const OPCODE: u32 = 0b10100000010000001000000000000001u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ldnt1d_mz_p_bi_4";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldnt1d_mz_p_bi_4 {
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub PNg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<3>,
    }
    impl ldnt1d_mz_p_bi_4 {
        #[inline]
        pub const fn new(
            imm4: ::aarchmrs_types::BitValue<4>,
            msz: ::aarchmrs_types::BitValue<2>,
            PNg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<3>,
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
                0b101000000100u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.PNg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 2u32
                    | 0b01u32 << 0u32,
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
