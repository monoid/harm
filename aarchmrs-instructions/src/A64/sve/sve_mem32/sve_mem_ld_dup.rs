/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1rb_z_p_bi_u8 {
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rb_z_p_bi_u8";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rb_z_p_bi_u8 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rb_z_p_bi_u8 {
        #[inline]
        pub const fn new(
            dtypeh: ::aarchmrs_types::BitValue<2>,
            imm6: ::aarchmrs_types::BitValue<6>,
            dtypel: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtypeh,
                imm6,
                dtypel,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | self.dtypeh.into_inner() << 23u32
                    | 0b1u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.dtypel.into_inner() << 13u32
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
pub mod ld1rb_z_p_bi_u16 {
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rb_z_p_bi_u16";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rb_z_p_bi_u16 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rb_z_p_bi_u16 {
        #[inline]
        pub const fn new(
            dtypeh: ::aarchmrs_types::BitValue<2>,
            imm6: ::aarchmrs_types::BitValue<6>,
            dtypel: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtypeh,
                imm6,
                dtypel,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | self.dtypeh.into_inner() << 23u32
                    | 0b1u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.dtypel.into_inner() << 13u32
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
pub mod ld1rb_z_p_bi_u32 {
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rb_z_p_bi_u32";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rb_z_p_bi_u32 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rb_z_p_bi_u32 {
        #[inline]
        pub const fn new(
            dtypeh: ::aarchmrs_types::BitValue<2>,
            imm6: ::aarchmrs_types::BitValue<6>,
            dtypel: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtypeh,
                imm6,
                dtypel,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | self.dtypeh.into_inner() << 23u32
                    | 0b1u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.dtypel.into_inner() << 13u32
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
pub mod ld1rb_z_p_bi_u64 {
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rb_z_p_bi_u64";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rb_z_p_bi_u64 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rb_z_p_bi_u64 {
        #[inline]
        pub const fn new(
            dtypeh: ::aarchmrs_types::BitValue<2>,
            imm6: ::aarchmrs_types::BitValue<6>,
            dtypel: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtypeh,
                imm6,
                dtypel,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | self.dtypeh.into_inner() << 23u32
                    | 0b1u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.dtypel.into_inner() << 13u32
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
pub mod ld1rsw_z_p_bi_s64 {
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rsw_z_p_bi_s64";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rsw_z_p_bi_s64 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rsw_z_p_bi_s64 {
        #[inline]
        pub const fn new(
            dtypeh: ::aarchmrs_types::BitValue<2>,
            imm6: ::aarchmrs_types::BitValue<6>,
            dtypel: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtypeh,
                imm6,
                dtypel,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | self.dtypeh.into_inner() << 23u32
                    | 0b1u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.dtypel.into_inner() << 13u32
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
pub mod ld1rh_z_p_bi_u16 {
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rh_z_p_bi_u16";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rh_z_p_bi_u16 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rh_z_p_bi_u16 {
        #[inline]
        pub const fn new(
            dtypeh: ::aarchmrs_types::BitValue<2>,
            imm6: ::aarchmrs_types::BitValue<6>,
            dtypel: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtypeh,
                imm6,
                dtypel,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | self.dtypeh.into_inner() << 23u32
                    | 0b1u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.dtypel.into_inner() << 13u32
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
pub mod ld1rh_z_p_bi_u32 {
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rh_z_p_bi_u32";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rh_z_p_bi_u32 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rh_z_p_bi_u32 {
        #[inline]
        pub const fn new(
            dtypeh: ::aarchmrs_types::BitValue<2>,
            imm6: ::aarchmrs_types::BitValue<6>,
            dtypel: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtypeh,
                imm6,
                dtypel,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | self.dtypeh.into_inner() << 23u32
                    | 0b1u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.dtypel.into_inner() << 13u32
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
pub mod ld1rh_z_p_bi_u64 {
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rh_z_p_bi_u64";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rh_z_p_bi_u64 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rh_z_p_bi_u64 {
        #[inline]
        pub const fn new(
            dtypeh: ::aarchmrs_types::BitValue<2>,
            imm6: ::aarchmrs_types::BitValue<6>,
            dtypel: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtypeh,
                imm6,
                dtypel,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | self.dtypeh.into_inner() << 23u32
                    | 0b1u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.dtypel.into_inner() << 13u32
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
pub mod ld1rsh_z_p_bi_s64 {
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rsh_z_p_bi_s64";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rsh_z_p_bi_s64 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rsh_z_p_bi_s64 {
        #[inline]
        pub const fn new(
            dtypeh: ::aarchmrs_types::BitValue<2>,
            imm6: ::aarchmrs_types::BitValue<6>,
            dtypel: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtypeh,
                imm6,
                dtypel,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | self.dtypeh.into_inner() << 23u32
                    | 0b1u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.dtypel.into_inner() << 13u32
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
pub mod ld1rsh_z_p_bi_s32 {
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rsh_z_p_bi_s32";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rsh_z_p_bi_s32 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rsh_z_p_bi_s32 {
        #[inline]
        pub const fn new(
            dtypeh: ::aarchmrs_types::BitValue<2>,
            imm6: ::aarchmrs_types::BitValue<6>,
            dtypel: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtypeh,
                imm6,
                dtypel,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | self.dtypeh.into_inner() << 23u32
                    | 0b1u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.dtypel.into_inner() << 13u32
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
pub mod ld1rw_z_p_bi_u32 {
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rw_z_p_bi_u32";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rw_z_p_bi_u32 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rw_z_p_bi_u32 {
        #[inline]
        pub const fn new(
            dtypeh: ::aarchmrs_types::BitValue<2>,
            imm6: ::aarchmrs_types::BitValue<6>,
            dtypel: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtypeh,
                imm6,
                dtypel,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | self.dtypeh.into_inner() << 23u32
                    | 0b1u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.dtypel.into_inner() << 13u32
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
pub mod ld1rw_z_p_bi_u64 {
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rw_z_p_bi_u64";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rw_z_p_bi_u64 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rw_z_p_bi_u64 {
        #[inline]
        pub const fn new(
            dtypeh: ::aarchmrs_types::BitValue<2>,
            imm6: ::aarchmrs_types::BitValue<6>,
            dtypel: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtypeh,
                imm6,
                dtypel,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | self.dtypeh.into_inner() << 23u32
                    | 0b1u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.dtypel.into_inner() << 13u32
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
pub mod ld1rsb_z_p_bi_s64 {
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rsb_z_p_bi_s64";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rsb_z_p_bi_s64 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rsb_z_p_bi_s64 {
        #[inline]
        pub const fn new(
            dtypeh: ::aarchmrs_types::BitValue<2>,
            imm6: ::aarchmrs_types::BitValue<6>,
            dtypel: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtypeh,
                imm6,
                dtypel,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | self.dtypeh.into_inner() << 23u32
                    | 0b1u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.dtypel.into_inner() << 13u32
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
pub mod ld1rsb_z_p_bi_s32 {
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rsb_z_p_bi_s32";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rsb_z_p_bi_s32 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rsb_z_p_bi_s32 {
        #[inline]
        pub const fn new(
            dtypeh: ::aarchmrs_types::BitValue<2>,
            imm6: ::aarchmrs_types::BitValue<6>,
            dtypel: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtypeh,
                imm6,
                dtypel,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | self.dtypeh.into_inner() << 23u32
                    | 0b1u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.dtypel.into_inner() << 13u32
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
pub mod ld1rsb_z_p_bi_s16 {
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rsb_z_p_bi_s16";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rsb_z_p_bi_s16 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rsb_z_p_bi_s16 {
        #[inline]
        pub const fn new(
            dtypeh: ::aarchmrs_types::BitValue<2>,
            imm6: ::aarchmrs_types::BitValue<6>,
            dtypel: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtypeh,
                imm6,
                dtypel,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | self.dtypeh.into_inner() << 23u32
                    | 0b1u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.dtypel.into_inner() << 13u32
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
pub mod ld1rd_z_p_bi_u64 {
    pub const OPCODE_MASK: u32 = 0b11111110010000001000000000000000u32;
    pub const OPCODE: u32 = 0b10000100010000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1rd_z_p_bi_u64";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1rd_z_p_bi_u64 {
        pub dtypeh: ::aarchmrs_types::BitValue<2>,
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub dtypel: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1rd_z_p_bi_u64 {
        #[inline]
        pub const fn new(
            dtypeh: ::aarchmrs_types::BitValue<2>,
            imm6: ::aarchmrs_types::BitValue<6>,
            dtypel: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                dtypeh,
                imm6,
                dtypel,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1000010u32 << 25u32
                    | self.dtypeh.into_inner() << 23u32
                    | 0b1u32 << 22u32
                    | self.imm6.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.dtypel.into_inner() << 13u32
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
