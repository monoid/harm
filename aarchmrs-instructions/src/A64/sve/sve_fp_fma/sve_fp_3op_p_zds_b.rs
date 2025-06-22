/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmad_z_p_zzz_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001000000000000000u32;
    pub const OPCODE: u32 = 0b01100101001000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "fmad_z_p_zzz_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmad_z_p_zzz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Za: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl fmad_z_p_zzz_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Za: ::aarchmrs_types::BitValue<5>,
            N: ::aarchmrs_types::BitValue<1>,
            op: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                Za,
                N,
                op,
                Pg,
                Zm,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Za.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.N.into_inner() << 14u32
                    | self.op.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
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
pub mod fmsb_z_p_zzz_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001000000000000000u32;
    pub const OPCODE: u32 = 0b01100101001000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "fmsb_z_p_zzz_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmsb_z_p_zzz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Za: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl fmsb_z_p_zzz_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Za: ::aarchmrs_types::BitValue<5>,
            N: ::aarchmrs_types::BitValue<1>,
            op: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                Za,
                N,
                op,
                Pg,
                Zm,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Za.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.N.into_inner() << 14u32
                    | self.op.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
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
pub mod fnmad_z_p_zzz_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001000000000000000u32;
    pub const OPCODE: u32 = 0b01100101001000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "fnmad_z_p_zzz_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fnmad_z_p_zzz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Za: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl fnmad_z_p_zzz_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Za: ::aarchmrs_types::BitValue<5>,
            N: ::aarchmrs_types::BitValue<1>,
            op: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                Za,
                N,
                op,
                Pg,
                Zm,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Za.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.N.into_inner() << 14u32
                    | self.op.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
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
pub mod fnmsb_z_p_zzz_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001000000000000000u32;
    pub const OPCODE: u32 = 0b01100101001000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "fnmsb_z_p_zzz_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fnmsb_z_p_zzz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Za: ::aarchmrs_types::BitValue<5>,
        pub N: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl fnmsb_z_p_zzz_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Za: ::aarchmrs_types::BitValue<5>,
            N: ::aarchmrs_types::BitValue<1>,
            op: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zm: ::aarchmrs_types::BitValue<5>,
            Zdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                Za,
                N,
                op,
                Pg,
                Zm,
                Zdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Za.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.N.into_inner() << 14u32
                    | self.op.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zm.into_inner() << 5u32
                    | self.Zdn.into_inner() << 0u32,
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
