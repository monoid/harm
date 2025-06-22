/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1rqb_z_p_bi_u8 {
    pub const OPCODE_MASK: u32 = 0b11111110000100001110000000000000u32;
    pub const OPCODE: u32 = 0b10100100000000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rqb_z_p_bi_u8";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rqb_z_p_bi_u8 {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub ssz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rqb_z_p_bi_u8 {
        #[inline]
        pub const fn new(
            msz: ::aarchmrs_types::BitValue<2>,
            ssz: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                msz,
                ssz,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.msz.into_inner() << 23u32
                    | self.ssz.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
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
pub mod ld1rob_z_p_bi_u8 {
    pub const OPCODE_MASK: u32 = 0b11111110000100001110000000000000u32;
    pub const OPCODE: u32 = 0b10100100000000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rob_z_p_bi_u8";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rob_z_p_bi_u8 {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub ssz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rob_z_p_bi_u8 {
        #[inline]
        pub const fn new(
            msz: ::aarchmrs_types::BitValue<2>,
            ssz: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                msz,
                ssz,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.msz.into_inner() << 23u32
                    | self.ssz.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
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
pub mod ld1rqh_z_p_bi_u16 {
    pub const OPCODE_MASK: u32 = 0b11111110000100001110000000000000u32;
    pub const OPCODE: u32 = 0b10100100000000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rqh_z_p_bi_u16";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rqh_z_p_bi_u16 {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub ssz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rqh_z_p_bi_u16 {
        #[inline]
        pub const fn new(
            msz: ::aarchmrs_types::BitValue<2>,
            ssz: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                msz,
                ssz,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.msz.into_inner() << 23u32
                    | self.ssz.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
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
pub mod ld1roh_z_p_bi_u16 {
    pub const OPCODE_MASK: u32 = 0b11111110000100001110000000000000u32;
    pub const OPCODE: u32 = 0b10100100000000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1roh_z_p_bi_u16";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1roh_z_p_bi_u16 {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub ssz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1roh_z_p_bi_u16 {
        #[inline]
        pub const fn new(
            msz: ::aarchmrs_types::BitValue<2>,
            ssz: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                msz,
                ssz,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.msz.into_inner() << 23u32
                    | self.ssz.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
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
pub mod ld1rqw_z_p_bi_u32 {
    pub const OPCODE_MASK: u32 = 0b11111110000100001110000000000000u32;
    pub const OPCODE: u32 = 0b10100100000000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rqw_z_p_bi_u32";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rqw_z_p_bi_u32 {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub ssz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rqw_z_p_bi_u32 {
        #[inline]
        pub const fn new(
            msz: ::aarchmrs_types::BitValue<2>,
            ssz: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                msz,
                ssz,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.msz.into_inner() << 23u32
                    | self.ssz.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
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
pub mod ld1row_z_p_bi_u32 {
    pub const OPCODE_MASK: u32 = 0b11111110000100001110000000000000u32;
    pub const OPCODE: u32 = 0b10100100000000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1row_z_p_bi_u32";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1row_z_p_bi_u32 {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub ssz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1row_z_p_bi_u32 {
        #[inline]
        pub const fn new(
            msz: ::aarchmrs_types::BitValue<2>,
            ssz: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                msz,
                ssz,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.msz.into_inner() << 23u32
                    | self.ssz.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
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
pub mod ld1rqd_z_p_bi_u64 {
    pub const OPCODE_MASK: u32 = 0b11111110000100001110000000000000u32;
    pub const OPCODE: u32 = 0b10100100000000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rqd_z_p_bi_u64";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rqd_z_p_bi_u64 {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub ssz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rqd_z_p_bi_u64 {
        #[inline]
        pub const fn new(
            msz: ::aarchmrs_types::BitValue<2>,
            ssz: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                msz,
                ssz,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.msz.into_inner() << 23u32
                    | self.ssz.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
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
pub mod ld1rod_z_p_bi_u64 {
    pub const OPCODE_MASK: u32 = 0b11111110000100001110000000000000u32;
    pub const OPCODE: u32 = 0b10100100000000000010000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rod_z_p_bi_u64";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rod_z_p_bi_u64 {
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub ssz: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rod_z_p_bi_u64 {
        #[inline]
        pub const fn new(
            msz: ::aarchmrs_types::BitValue<2>,
            ssz: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                msz,
                ssz,
                imm4,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1010010u32 << 25u32
                    | self.msz.into_inner() << 23u32
                    | self.ssz.into_inner() << 21u32
                    | 0b0u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b001u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
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
