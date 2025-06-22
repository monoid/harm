/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod brkpa_p_p_pp_ {
    pub const OPCODE_MASK: u32 = 0b11111111101100001100001000000000u32;
    pub const OPCODE: u32 = 0b00100101000000001100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "brkpa_p_p_pp_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct brkpa_p_p_pp_ {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub B: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl brkpa_p_p_pp_ {
        #[inline]
        pub const fn new(
            S: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            B: ::aarchmrs_types::BitValue<1>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                S,
                Pm,
                Pg,
                Pn,
                B,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010u32 << 23u32
                    | self.S.into_inner() << 22u32
                    | 0b00u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b11u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | self.B.into_inner() << 4u32
                    | self.Pd.into_inner() << 0u32,
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
pub mod brkpas_p_p_pp_ {
    pub const OPCODE_MASK: u32 = 0b11111111101100001100001000000000u32;
    pub const OPCODE: u32 = 0b00100101000000001100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "brkpas_p_p_pp_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct brkpas_p_p_pp_ {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub B: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl brkpas_p_p_pp_ {
        #[inline]
        pub const fn new(
            S: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            B: ::aarchmrs_types::BitValue<1>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                S,
                Pm,
                Pg,
                Pn,
                B,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010u32 << 23u32
                    | self.S.into_inner() << 22u32
                    | 0b00u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b11u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | self.B.into_inner() << 4u32
                    | self.Pd.into_inner() << 0u32,
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
pub mod brkpb_p_p_pp_ {
    pub const OPCODE_MASK: u32 = 0b11111111101100001100001000000000u32;
    pub const OPCODE: u32 = 0b00100101000000001100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "brkpb_p_p_pp_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct brkpb_p_p_pp_ {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub B: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl brkpb_p_p_pp_ {
        #[inline]
        pub const fn new(
            S: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            B: ::aarchmrs_types::BitValue<1>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                S,
                Pm,
                Pg,
                Pn,
                B,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010u32 << 23u32
                    | self.S.into_inner() << 22u32
                    | 0b00u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b11u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | self.B.into_inner() << 4u32
                    | self.Pd.into_inner() << 0u32,
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
pub mod brkpbs_p_p_pp_ {
    pub const OPCODE_MASK: u32 = 0b11111111101100001100001000000000u32;
    pub const OPCODE: u32 = 0b00100101000000001100000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "brkpbs_p_p_pp_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct brkpbs_p_p_pp_ {
        pub S: ::aarchmrs_types::BitValue<1>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
        pub B: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl brkpbs_p_p_pp_ {
        #[inline]
        pub const fn new(
            S: ::aarchmrs_types::BitValue<1>,
            Pm: ::aarchmrs_types::BitValue<4>,
            Pg: ::aarchmrs_types::BitValue<4>,
            Pn: ::aarchmrs_types::BitValue<4>,
            B: ::aarchmrs_types::BitValue<1>,
            Pd: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self {
                S,
                Pm,
                Pg,
                Pn,
                B,
                Pd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010u32 << 23u32
                    | self.S.into_inner() << 22u32
                    | 0b00u32 << 20u32
                    | self.Pm.into_inner() << 16u32
                    | 0b11u32 << 14u32
                    | self.Pg.into_inner() << 10u32
                    | 0b0u32 << 9u32
                    | self.Pn.into_inner() << 5u32
                    | self.B.into_inner() << 4u32
                    | self.Pd.into_inner() << 0u32,
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
