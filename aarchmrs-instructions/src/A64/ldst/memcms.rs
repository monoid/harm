/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CPYFP_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001000000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFP_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFP_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFP_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFPWT_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001000000000001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFPWT_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPWT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPWT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFPRT_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001000000000010010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFPRT_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPRT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPRT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFPT_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001000000000011010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFPT_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFPWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001000000000100010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFPWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFPWTWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001000000000101010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFPWTWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPWTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPWTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFPRTWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001000000000110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFPRTWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPRTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPRTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFPTWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001000000000111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFPTWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFPRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001000000001000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFPRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFPWTRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001000000001001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFPWTRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPWTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPWTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFPRTRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001000000001010010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFPRTRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPRTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPRTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFPTRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001000000001011010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFPTRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFPN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001000000001100010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFPN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFPWTN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001000000001101010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFPWTN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPWTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPWTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFPRTN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001000000001110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFPRTN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPRTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPRTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFPTN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001000000001111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFPTN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFM_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001010000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFM_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFM_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFM_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFMWT_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001010000000001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFMWT_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMWT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMWT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFMRT_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001010000000010010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFMRT_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMRT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMRT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFMT_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001010000000011010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFMT_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFMWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001010000000100010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFMWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFMWTWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001010000000101010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFMWTWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMWTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMWTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFMRTWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001010000000110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFMRTWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMRTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMRTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFMTWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001010000000111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFMTWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFMRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001010000001000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFMRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFMWTRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001010000001001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFMWTRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMWTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMWTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFMRTRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001010000001010010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFMRTRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMRTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMRTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFMTRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001010000001011010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFMTRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFMN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001010000001100010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFMN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFMWTN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001010000001101010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFMWTN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMWTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMWTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFMRTN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001010000001110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFMRTN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMRTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMRTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFMTN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001010000001111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFMTN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFE_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001100000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFE_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFE_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFE_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFEWT_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001100000000001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFEWT_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFEWT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFEWT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFERT_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001100000000010010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFERT_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFERT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFERT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFET_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001100000000011010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFET_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFET_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFET_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFEWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001100000000100010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFEWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFEWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFEWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFEWTWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001100000000101010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFEWTWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFEWTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFEWTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFERTWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001100000000110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFERTWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFERTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFERTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFETWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001100000000111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFETWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFETWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFETWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFERN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001100000001000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFERN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFERN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFERN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFEWTRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001100000001001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFEWTRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFEWTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFEWTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFERTRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001100000001010010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFERTRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFERTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFERTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFETRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001100000001011010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFETRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFETRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFETRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFEN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001100000001100010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFEN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFEN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFEN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFEWTN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001100000001101010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFEWTN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFEWTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFEWTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFERTN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001100000001110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFERTN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFERTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFERTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYFETN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001100000001111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYFETN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFETN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFETN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETP_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001110000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETP_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETP_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETP_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETPT_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001110000000001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETPT_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETPT_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETPT_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETPN_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001110000000010010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETPN_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETPN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETPN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETPTN_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001110000000011010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETPTN_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETPTN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETPTN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETM_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001110000000100010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETM_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETM_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETM_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETMT_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001110000000101010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETMT_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETMT_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETMT_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETMN_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001110000000110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETMN_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETMN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETMN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETMTN_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001110000000111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETMTN_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETMTN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETMTN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETE_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001110000001000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETE_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETE_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETE_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETET_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001110000001001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETET_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETET_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETET_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETEN_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001110000001010010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETEN_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETEN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETEN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETETN_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011001110000001011010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETETN_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETETN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETETN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYP_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101000000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYP_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYP_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYP_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYPWT_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101000000000001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYPWT_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPWT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPWT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYPRT_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101000000000010010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYPRT_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPRT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPRT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYPT_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101000000000011010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYPT_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYPWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101000000000100010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYPWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYPWTWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101000000000101010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYPWTWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPWTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPWTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYPRTWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101000000000110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYPRTWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPRTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPRTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYPTWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101000000000111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYPTWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYPRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101000000001000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYPRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYPWTRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101000000001001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYPWTRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPWTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPWTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYPRTRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101000000001010010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYPRTRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPRTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPRTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYPTRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101000000001011010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYPTRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYPN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101000000001100010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYPN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYPWTN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101000000001101010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYPWTN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPWTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPWTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYPRTN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101000000001110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYPRTN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPRTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPRTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYPTN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101000000001111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYPTN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYM_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101010000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYM_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYM_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYM_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYMWT_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101010000000001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYMWT_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMWT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMWT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYMRT_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101010000000010010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYMRT_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMRT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMRT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYMT_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101010000000011010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYMT_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYMWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101010000000100010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYMWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYMWTWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101010000000101010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYMWTWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMWTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMWTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYMRTWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101010000000110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYMRTWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMRTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMRTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYMTWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101010000000111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYMTWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYMRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101010000001000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYMRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYMWTRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101010000001001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYMWTRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMWTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMWTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYMRTRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101010000001010010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYMRTRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMRTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMRTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYMTRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101010000001011010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYMTRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYMN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101010000001100010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYMN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYMWTN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101010000001101010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYMWTN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMWTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMWTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYMRTN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101010000001110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYMRTN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMRTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMRTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYMTN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101010000001111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYMTN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYE_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101100000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYE_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYE_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYE_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYEWT_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101100000000001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYEWT_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYEWT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYEWT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYERT_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101100000000010010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYERT_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYERT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYERT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYET_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101100000000011010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYET_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYET_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYET_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYEWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101100000000100010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYEWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYEWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYEWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYEWTWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101100000000101010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYEWTWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYEWTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYEWTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYERTWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101100000000110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYERTWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYERTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYERTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYETWN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101100000000111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYETWN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYETWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYETWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYERN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101100000001000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYERN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYERN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYERN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYEWTRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101100000001001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYEWTRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYEWTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYEWTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYERTRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101100000001010010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYERTRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYERTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYERTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYETRN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101100000001011010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYETRN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYETRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYETRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYEN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101100000001100010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYEN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYEN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYEN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYEWTN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101100000001101010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYEWTN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYEWTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYEWTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYERTN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101100000001110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYERTN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYERTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYERTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod CPYETN_CPY_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101100000001111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "CPYETN_CPY_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYETN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYETN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETGP_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101110000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETGP_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGP_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGP_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETGPT_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101110000000001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETGPT_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGPT_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGPT_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETGPN_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101110000000010010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETGPN_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGPN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGPN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETGPTN_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101110000000011010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETGPTN_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGPTN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGPTN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETGM_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101110000000100010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETGM_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGM_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGM_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETGMT_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101110000000101010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETGMT_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGMT_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGMT_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETGMN_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101110000000110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETGMN_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGMN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGMN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETGMTN_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101110000000111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETGMTN_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGMTN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGMTN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETGE_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101110000001000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETGE_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGE_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGE_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETGET_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101110000001001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETGET_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGET_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGET_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETGEN_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101110000001010010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETGEN_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGEN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGEN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
pub mod SETGETN_SET_memcms {
    pub const OPCODE_MASK: u32 = 0b00111111111000001111110000000000u32;
    pub const OPCODE: u32 = 0b00011101110000001011010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "SETGETN_SET_memcms";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGETN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGETN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
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
