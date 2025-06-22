/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1w_z_p_bz_s_x32_scaled {
    pub const OPCODE_MASK: u32 = 0b11111111101000001100000000000000u32;
    pub const OPCODE: u32 = 0b10000101001000000100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ld1w_z_p_bz_s_x32_scaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1w_z_p_bz_s_x32_scaled {
        pub xs: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1w_z_p_bz_s_x32_scaled {
        #[inline]
        pub const fn new(
            xs: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<5>,
            ff: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                xs,
                Zm,
                ff,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b100001010u32 << 23u32
                    | self.xs.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.ff.into_inner() << 13u32
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
pub mod ldff1w_z_p_bz_s_x32_scaled {
    pub const OPCODE_MASK: u32 = 0b11111111101000001100000000000000u32;
    pub const OPCODE: u32 = 0b10000101001000000100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ldff1w_z_p_bz_s_x32_scaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldff1w_z_p_bz_s_x32_scaled {
        pub xs: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub ff: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ldff1w_z_p_bz_s_x32_scaled {
        #[inline]
        pub const fn new(
            xs: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<5>,
            ff: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                xs,
                Zm,
                ff,
                Pg,
                Rn,
                Zt,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b100001010u32 << 23u32
                    | self.xs.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.ff.into_inner() << 13u32
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
