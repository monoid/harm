/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod st1h_z_p_bz_s_x32_scaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000001010000000000000u32;
    pub const OPCODE: u32 = 0b11100100111000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "st1h_z_p_bz_s_x32_scaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1h_z_p_bz_s_x32_scaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub xs: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st1h_z_p_bz_s_x32_scaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            xs: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, xs, Pg, Rn, Zt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100100111u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.xs.into_inner() << 14u32
                    | 0b0u32 << 13u32
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
pub mod st1w_z_p_bz_s_x32_scaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000001010000000000000u32;
    pub const OPCODE: u32 = 0b11100101011000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "st1w_z_p_bz_s_x32_scaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1w_z_p_bz_s_x32_scaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub xs: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st1w_z_p_bz_s_x32_scaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            xs: ::aarchmrs_types::BitValue<1>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, xs, Pg, Rn, Zt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100101011u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.xs.into_inner() << 14u32
                    | 0b0u32 << 13u32
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
