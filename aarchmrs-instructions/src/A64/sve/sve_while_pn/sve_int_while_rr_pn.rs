/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod whilege_pn_rr_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001101100000010000u32;
    pub const OPCODE: u32 = 0b00100101001000000100000000010000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "whilege_pn_rr_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilege_pn_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub vl: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub PNd: ::aarchmrs_types::BitValue<3>,
    }
    impl whilege_pn_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            vl: ::aarchmrs_types::BitValue<1>,
            lt: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            eq: ::aarchmrs_types::BitValue<1>,
            PNd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                size,
                Rm,
                vl,
                lt,
                Rn,
                eq,
                PNd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.vl.into_inner() << 13u32
                    | 0b00u32 << 11u32
                    | self.lt.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.eq.into_inner() << 3u32
                    | self.PNd.into_inner() << 0u32,
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
pub mod whilehs_pn_rr_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001101100000010000u32;
    pub const OPCODE: u32 = 0b00100101001000000100100000010000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "whilehs_pn_rr_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilehs_pn_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub vl: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub PNd: ::aarchmrs_types::BitValue<3>,
    }
    impl whilehs_pn_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            vl: ::aarchmrs_types::BitValue<1>,
            lt: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            eq: ::aarchmrs_types::BitValue<1>,
            PNd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                size,
                Rm,
                vl,
                lt,
                Rn,
                eq,
                PNd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.vl.into_inner() << 13u32
                    | 0b01u32 << 11u32
                    | self.lt.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.eq.into_inner() << 3u32
                    | self.PNd.into_inner() << 0u32,
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
pub mod whilegt_pn_rr_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001101100000010000u32;
    pub const OPCODE: u32 = 0b00100101001000000100000000010000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "whilegt_pn_rr_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilegt_pn_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub vl: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub PNd: ::aarchmrs_types::BitValue<3>,
    }
    impl whilegt_pn_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            vl: ::aarchmrs_types::BitValue<1>,
            lt: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            eq: ::aarchmrs_types::BitValue<1>,
            PNd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                size,
                Rm,
                vl,
                lt,
                Rn,
                eq,
                PNd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.vl.into_inner() << 13u32
                    | 0b00u32 << 11u32
                    | self.lt.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.eq.into_inner() << 3u32
                    | self.PNd.into_inner() << 0u32,
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
pub mod whilehi_pn_rr_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001101100000010000u32;
    pub const OPCODE: u32 = 0b00100101001000000100100000010000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "whilehi_pn_rr_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilehi_pn_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub vl: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub PNd: ::aarchmrs_types::BitValue<3>,
    }
    impl whilehi_pn_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            vl: ::aarchmrs_types::BitValue<1>,
            lt: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            eq: ::aarchmrs_types::BitValue<1>,
            PNd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                size,
                Rm,
                vl,
                lt,
                Rn,
                eq,
                PNd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.vl.into_inner() << 13u32
                    | 0b01u32 << 11u32
                    | self.lt.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.eq.into_inner() << 3u32
                    | self.PNd.into_inner() << 0u32,
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
pub mod whilelt_pn_rr_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001101100000010000u32;
    pub const OPCODE: u32 = 0b00100101001000000100000000010000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "whilelt_pn_rr_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilelt_pn_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub vl: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub PNd: ::aarchmrs_types::BitValue<3>,
    }
    impl whilelt_pn_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            vl: ::aarchmrs_types::BitValue<1>,
            lt: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            eq: ::aarchmrs_types::BitValue<1>,
            PNd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                size,
                Rm,
                vl,
                lt,
                Rn,
                eq,
                PNd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.vl.into_inner() << 13u32
                    | 0b00u32 << 11u32
                    | self.lt.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.eq.into_inner() << 3u32
                    | self.PNd.into_inner() << 0u32,
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
pub mod whilelo_pn_rr_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001101100000010000u32;
    pub const OPCODE: u32 = 0b00100101001000000100100000010000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "whilelo_pn_rr_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilelo_pn_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub vl: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub PNd: ::aarchmrs_types::BitValue<3>,
    }
    impl whilelo_pn_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            vl: ::aarchmrs_types::BitValue<1>,
            lt: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            eq: ::aarchmrs_types::BitValue<1>,
            PNd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                size,
                Rm,
                vl,
                lt,
                Rn,
                eq,
                PNd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.vl.into_inner() << 13u32
                    | 0b01u32 << 11u32
                    | self.lt.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.eq.into_inner() << 3u32
                    | self.PNd.into_inner() << 0u32,
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
pub mod whilele_pn_rr_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001101100000010000u32;
    pub const OPCODE: u32 = 0b00100101001000000100000000010000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "whilele_pn_rr_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilele_pn_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub vl: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub PNd: ::aarchmrs_types::BitValue<3>,
    }
    impl whilele_pn_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            vl: ::aarchmrs_types::BitValue<1>,
            lt: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            eq: ::aarchmrs_types::BitValue<1>,
            PNd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                size,
                Rm,
                vl,
                lt,
                Rn,
                eq,
                PNd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.vl.into_inner() << 13u32
                    | 0b00u32 << 11u32
                    | self.lt.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.eq.into_inner() << 3u32
                    | self.PNd.into_inner() << 0u32,
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
pub mod whilels_pn_rr_ {
    pub const OPCODE_MASK: u32 = 0b11111111001000001101100000010000u32;
    pub const OPCODE: u32 = 0b00100101001000000100100000010000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "whilels_pn_rr_";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct whilels_pn_rr_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub vl: ::aarchmrs_types::BitValue<1>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub eq: ::aarchmrs_types::BitValue<1>,
        pub PNd: ::aarchmrs_types::BitValue<3>,
    }
    impl whilels_pn_rr_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            Rm: ::aarchmrs_types::BitValue<5>,
            vl: ::aarchmrs_types::BitValue<1>,
            lt: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            eq: ::aarchmrs_types::BitValue<1>,
            PNd: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                size,
                Rm,
                vl,
                lt,
                Rn,
                eq,
                PNd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b1u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b01u32 << 14u32
                    | self.vl.into_inner() << 13u32
                    | 0b01u32 << 11u32
                    | self.lt.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.eq.into_inner() << 3u32
                    | self.PNd.into_inner() << 0u32,
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
