/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod prfb_i_p_bz_d_64_scaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000010000u32;
    pub const OPCODE: u32 = 0b11000100011000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "prfb_i_p_bz_d_64_scaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfb_i_p_bz_d_64_scaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfb_i_p_bz_d_64_scaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            prfop: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                Zm,
                msz,
                Pg,
                Rn,
                prfop,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100011u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.prfop.into_inner() << 0u32,
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
pub mod prfh_i_p_bz_d_64_scaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000010000u32;
    pub const OPCODE: u32 = 0b11000100011000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "prfh_i_p_bz_d_64_scaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfh_i_p_bz_d_64_scaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfh_i_p_bz_d_64_scaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            prfop: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                Zm,
                msz,
                Pg,
                Rn,
                prfop,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100011u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.prfop.into_inner() << 0u32,
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
pub mod prfw_i_p_bz_d_64_scaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000010000u32;
    pub const OPCODE: u32 = 0b11000100011000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "prfw_i_p_bz_d_64_scaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfw_i_p_bz_d_64_scaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfw_i_p_bz_d_64_scaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            prfop: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                Zm,
                msz,
                Pg,
                Rn,
                prfop,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100011u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.prfop.into_inner() << 0u32,
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
pub mod prfd_i_p_bz_d_64_scaled {
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000010000u32;
    pub const OPCODE: u32 = 0b11000100011000001000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "prfd_i_p_bz_d_64_scaled";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct prfd_i_p_bz_d_64_scaled {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub msz: ::aarchmrs_types::BitValue<2>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub prfop: ::aarchmrs_types::BitValue<4>,
    }
    impl prfd_i_p_bz_d_64_scaled {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            msz: ::aarchmrs_types::BitValue<2>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            prfop: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                Zm,
                msz,
                Pg,
                Rn,
                prfop,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100011u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b1u32 << 15u32
                    | self.msz.into_inner() << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b0u32 << 4u32
                    | self.prfop.into_inner() << 0u32,
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
