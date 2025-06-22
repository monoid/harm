/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MOVI_asimdimm_L_sl {
    pub const OPCODE_MASK: u32 = 0b10111111111110001001110000000000u32;
    pub const OPCODE: u32 = 0b00001111000000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MOVI_asimdimm_L_sl";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MOVI_asimdimm_L_sl {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub a: ::aarchmrs_types::BitValue<1>,
        pub b: ::aarchmrs_types::BitValue<1>,
        pub c: ::aarchmrs_types::BitValue<1>,
        pub cmode: ::aarchmrs_types::BitValue<2>,
        pub d: ::aarchmrs_types::BitValue<1>,
        pub e: ::aarchmrs_types::BitValue<1>,
        pub f: ::aarchmrs_types::BitValue<1>,
        pub g: ::aarchmrs_types::BitValue<1>,
        pub h: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MOVI_asimdimm_L_sl {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            a: ::aarchmrs_types::BitValue<1>,
            b: ::aarchmrs_types::BitValue<1>,
            c: ::aarchmrs_types::BitValue<1>,
            cmode: ::aarchmrs_types::BitValue<2>,
            d: ::aarchmrs_types::BitValue<1>,
            e: ::aarchmrs_types::BitValue<1>,
            f: ::aarchmrs_types::BitValue<1>,
            g: ::aarchmrs_types::BitValue<1>,
            h: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                a,
                b,
                c,
                cmode,
                d,
                e,
                f,
                g,
                h,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b00111100000u32 << 19u32
                    | self.a.into_inner() << 18u32
                    | self.b.into_inner() << 17u32
                    | self.c.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.cmode.into_inner() << 13u32
                    | 0b001u32 << 10u32
                    | self.d.into_inner() << 9u32
                    | self.e.into_inner() << 8u32
                    | self.f.into_inner() << 7u32
                    | self.g.into_inner() << 6u32
                    | self.h.into_inner() << 5u32
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
pub mod ORR_asimdimm_L_sl {
    pub const OPCODE_MASK: u32 = 0b10111111111110001001110000000000u32;
    pub const OPCODE: u32 = 0b00001111000000000001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ORR_asimdimm_L_sl";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ORR_asimdimm_L_sl {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub a: ::aarchmrs_types::BitValue<1>,
        pub b: ::aarchmrs_types::BitValue<1>,
        pub c: ::aarchmrs_types::BitValue<1>,
        pub cmode: ::aarchmrs_types::BitValue<2>,
        pub d: ::aarchmrs_types::BitValue<1>,
        pub e: ::aarchmrs_types::BitValue<1>,
        pub f: ::aarchmrs_types::BitValue<1>,
        pub g: ::aarchmrs_types::BitValue<1>,
        pub h: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ORR_asimdimm_L_sl {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            a: ::aarchmrs_types::BitValue<1>,
            b: ::aarchmrs_types::BitValue<1>,
            c: ::aarchmrs_types::BitValue<1>,
            cmode: ::aarchmrs_types::BitValue<2>,
            d: ::aarchmrs_types::BitValue<1>,
            e: ::aarchmrs_types::BitValue<1>,
            f: ::aarchmrs_types::BitValue<1>,
            g: ::aarchmrs_types::BitValue<1>,
            h: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                a,
                b,
                c,
                cmode,
                d,
                e,
                f,
                g,
                h,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b00111100000u32 << 19u32
                    | self.a.into_inner() << 18u32
                    | self.b.into_inner() << 17u32
                    | self.c.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.cmode.into_inner() << 13u32
                    | 0b101u32 << 10u32
                    | self.d.into_inner() << 9u32
                    | self.e.into_inner() << 8u32
                    | self.f.into_inner() << 7u32
                    | self.g.into_inner() << 6u32
                    | self.h.into_inner() << 5u32
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
pub mod MOVI_asimdimm_L_hl {
    pub const OPCODE_MASK: u32 = 0b10111111111110001101110000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MOVI_asimdimm_L_hl";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MOVI_asimdimm_L_hl {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub a: ::aarchmrs_types::BitValue<1>,
        pub b: ::aarchmrs_types::BitValue<1>,
        pub c: ::aarchmrs_types::BitValue<1>,
        pub cmode: ::aarchmrs_types::BitValue<1>,
        pub d: ::aarchmrs_types::BitValue<1>,
        pub e: ::aarchmrs_types::BitValue<1>,
        pub f: ::aarchmrs_types::BitValue<1>,
        pub g: ::aarchmrs_types::BitValue<1>,
        pub h: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MOVI_asimdimm_L_hl {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            a: ::aarchmrs_types::BitValue<1>,
            b: ::aarchmrs_types::BitValue<1>,
            c: ::aarchmrs_types::BitValue<1>,
            cmode: ::aarchmrs_types::BitValue<1>,
            d: ::aarchmrs_types::BitValue<1>,
            e: ::aarchmrs_types::BitValue<1>,
            f: ::aarchmrs_types::BitValue<1>,
            g: ::aarchmrs_types::BitValue<1>,
            h: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                a,
                b,
                c,
                cmode,
                d,
                e,
                f,
                g,
                h,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b00111100000u32 << 19u32
                    | self.a.into_inner() << 18u32
                    | self.b.into_inner() << 17u32
                    | self.c.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.cmode.into_inner() << 13u32
                    | 0b001u32 << 10u32
                    | self.d.into_inner() << 9u32
                    | self.e.into_inner() << 8u32
                    | self.f.into_inner() << 7u32
                    | self.g.into_inner() << 6u32
                    | self.h.into_inner() << 5u32
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
pub mod ORR_asimdimm_L_hl {
    pub const OPCODE_MASK: u32 = 0b10111111111110001101110000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ORR_asimdimm_L_hl";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ORR_asimdimm_L_hl {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub a: ::aarchmrs_types::BitValue<1>,
        pub b: ::aarchmrs_types::BitValue<1>,
        pub c: ::aarchmrs_types::BitValue<1>,
        pub cmode: ::aarchmrs_types::BitValue<1>,
        pub d: ::aarchmrs_types::BitValue<1>,
        pub e: ::aarchmrs_types::BitValue<1>,
        pub f: ::aarchmrs_types::BitValue<1>,
        pub g: ::aarchmrs_types::BitValue<1>,
        pub h: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ORR_asimdimm_L_hl {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            a: ::aarchmrs_types::BitValue<1>,
            b: ::aarchmrs_types::BitValue<1>,
            c: ::aarchmrs_types::BitValue<1>,
            cmode: ::aarchmrs_types::BitValue<1>,
            d: ::aarchmrs_types::BitValue<1>,
            e: ::aarchmrs_types::BitValue<1>,
            f: ::aarchmrs_types::BitValue<1>,
            g: ::aarchmrs_types::BitValue<1>,
            h: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                a,
                b,
                c,
                cmode,
                d,
                e,
                f,
                g,
                h,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b00111100000u32 << 19u32
                    | self.a.into_inner() << 18u32
                    | self.b.into_inner() << 17u32
                    | self.c.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.cmode.into_inner() << 13u32
                    | 0b101u32 << 10u32
                    | self.d.into_inner() << 9u32
                    | self.e.into_inner() << 8u32
                    | self.f.into_inner() << 7u32
                    | self.g.into_inner() << 6u32
                    | self.h.into_inner() << 5u32
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
pub mod MOVI_asimdimm_M_sm {
    pub const OPCODE_MASK: u32 = 0b10111111111110001110110000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001100010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MOVI_asimdimm_M_sm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MOVI_asimdimm_M_sm {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub a: ::aarchmrs_types::BitValue<1>,
        pub b: ::aarchmrs_types::BitValue<1>,
        pub c: ::aarchmrs_types::BitValue<1>,
        pub cmode: ::aarchmrs_types::BitValue<1>,
        pub d: ::aarchmrs_types::BitValue<1>,
        pub e: ::aarchmrs_types::BitValue<1>,
        pub f: ::aarchmrs_types::BitValue<1>,
        pub g: ::aarchmrs_types::BitValue<1>,
        pub h: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MOVI_asimdimm_M_sm {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            a: ::aarchmrs_types::BitValue<1>,
            b: ::aarchmrs_types::BitValue<1>,
            c: ::aarchmrs_types::BitValue<1>,
            cmode: ::aarchmrs_types::BitValue<1>,
            d: ::aarchmrs_types::BitValue<1>,
            e: ::aarchmrs_types::BitValue<1>,
            f: ::aarchmrs_types::BitValue<1>,
            g: ::aarchmrs_types::BitValue<1>,
            h: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                a,
                b,
                c,
                cmode,
                d,
                e,
                f,
                g,
                h,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b00111100000u32 << 19u32
                    | self.a.into_inner() << 18u32
                    | self.b.into_inner() << 17u32
                    | self.c.into_inner() << 16u32
                    | 0b110u32 << 13u32
                    | self.cmode.into_inner() << 12u32
                    | 0b01u32 << 10u32
                    | self.d.into_inner() << 9u32
                    | self.e.into_inner() << 8u32
                    | self.f.into_inner() << 7u32
                    | self.g.into_inner() << 6u32
                    | self.h.into_inner() << 5u32
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
pub mod MOVI_asimdimm_N_b {
    pub const OPCODE_MASK: u32 = 0b10111111111110001111110000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MOVI_asimdimm_N_b";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MOVI_asimdimm_N_b {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub a: ::aarchmrs_types::BitValue<1>,
        pub b: ::aarchmrs_types::BitValue<1>,
        pub c: ::aarchmrs_types::BitValue<1>,
        pub d: ::aarchmrs_types::BitValue<1>,
        pub e: ::aarchmrs_types::BitValue<1>,
        pub f: ::aarchmrs_types::BitValue<1>,
        pub g: ::aarchmrs_types::BitValue<1>,
        pub h: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MOVI_asimdimm_N_b {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            a: ::aarchmrs_types::BitValue<1>,
            b: ::aarchmrs_types::BitValue<1>,
            c: ::aarchmrs_types::BitValue<1>,
            d: ::aarchmrs_types::BitValue<1>,
            e: ::aarchmrs_types::BitValue<1>,
            f: ::aarchmrs_types::BitValue<1>,
            g: ::aarchmrs_types::BitValue<1>,
            h: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                a,
                b,
                c,
                d,
                e,
                f,
                g,
                h,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b00111100000u32 << 19u32
                    | self.a.into_inner() << 18u32
                    | self.b.into_inner() << 17u32
                    | self.c.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.d.into_inner() << 9u32
                    | self.e.into_inner() << 8u32
                    | self.f.into_inner() << 7u32
                    | self.g.into_inner() << 6u32
                    | self.h.into_inner() << 5u32
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
pub mod FMOV_asimdimm_S_s {
    pub const OPCODE_MASK: u32 = 0b10111111111110001111110000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMOV_asimdimm_S_s";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMOV_asimdimm_S_s {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub a: ::aarchmrs_types::BitValue<1>,
        pub b: ::aarchmrs_types::BitValue<1>,
        pub c: ::aarchmrs_types::BitValue<1>,
        pub d: ::aarchmrs_types::BitValue<1>,
        pub e: ::aarchmrs_types::BitValue<1>,
        pub f: ::aarchmrs_types::BitValue<1>,
        pub g: ::aarchmrs_types::BitValue<1>,
        pub h: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMOV_asimdimm_S_s {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            a: ::aarchmrs_types::BitValue<1>,
            b: ::aarchmrs_types::BitValue<1>,
            c: ::aarchmrs_types::BitValue<1>,
            d: ::aarchmrs_types::BitValue<1>,
            e: ::aarchmrs_types::BitValue<1>,
            f: ::aarchmrs_types::BitValue<1>,
            g: ::aarchmrs_types::BitValue<1>,
            h: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                a,
                b,
                c,
                d,
                e,
                f,
                g,
                h,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b00111100000u32 << 19u32
                    | self.a.into_inner() << 18u32
                    | self.b.into_inner() << 17u32
                    | self.c.into_inner() << 16u32
                    | 0b111101u32 << 10u32
                    | self.d.into_inner() << 9u32
                    | self.e.into_inner() << 8u32
                    | self.f.into_inner() << 7u32
                    | self.g.into_inner() << 6u32
                    | self.h.into_inner() << 5u32
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
pub mod FMOV_asimdimm_H_h {
    pub const OPCODE_MASK: u32 = 0b10111111111110001111110000000000u32;
    pub const OPCODE: u32 = 0b00001111000000001111110000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMOV_asimdimm_H_h";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMOV_asimdimm_H_h {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub a: ::aarchmrs_types::BitValue<1>,
        pub b: ::aarchmrs_types::BitValue<1>,
        pub c: ::aarchmrs_types::BitValue<1>,
        pub d: ::aarchmrs_types::BitValue<1>,
        pub e: ::aarchmrs_types::BitValue<1>,
        pub f: ::aarchmrs_types::BitValue<1>,
        pub g: ::aarchmrs_types::BitValue<1>,
        pub h: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMOV_asimdimm_H_h {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            a: ::aarchmrs_types::BitValue<1>,
            b: ::aarchmrs_types::BitValue<1>,
            c: ::aarchmrs_types::BitValue<1>,
            d: ::aarchmrs_types::BitValue<1>,
            e: ::aarchmrs_types::BitValue<1>,
            f: ::aarchmrs_types::BitValue<1>,
            g: ::aarchmrs_types::BitValue<1>,
            h: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                a,
                b,
                c,
                d,
                e,
                f,
                g,
                h,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b00111100000u32 << 19u32
                    | self.a.into_inner() << 18u32
                    | self.b.into_inner() << 17u32
                    | self.c.into_inner() << 16u32
                    | 0b111111u32 << 10u32
                    | self.d.into_inner() << 9u32
                    | self.e.into_inner() << 8u32
                    | self.f.into_inner() << 7u32
                    | self.g.into_inner() << 6u32
                    | self.h.into_inner() << 5u32
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
pub mod MVNI_asimdimm_L_sl {
    pub const OPCODE_MASK: u32 = 0b10111111111110001001110000000000u32;
    pub const OPCODE: u32 = 0b00101111000000000000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MVNI_asimdimm_L_sl";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MVNI_asimdimm_L_sl {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub a: ::aarchmrs_types::BitValue<1>,
        pub b: ::aarchmrs_types::BitValue<1>,
        pub c: ::aarchmrs_types::BitValue<1>,
        pub cmode: ::aarchmrs_types::BitValue<2>,
        pub d: ::aarchmrs_types::BitValue<1>,
        pub e: ::aarchmrs_types::BitValue<1>,
        pub f: ::aarchmrs_types::BitValue<1>,
        pub g: ::aarchmrs_types::BitValue<1>,
        pub h: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MVNI_asimdimm_L_sl {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            a: ::aarchmrs_types::BitValue<1>,
            b: ::aarchmrs_types::BitValue<1>,
            c: ::aarchmrs_types::BitValue<1>,
            cmode: ::aarchmrs_types::BitValue<2>,
            d: ::aarchmrs_types::BitValue<1>,
            e: ::aarchmrs_types::BitValue<1>,
            f: ::aarchmrs_types::BitValue<1>,
            g: ::aarchmrs_types::BitValue<1>,
            h: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                a,
                b,
                c,
                cmode,
                d,
                e,
                f,
                g,
                h,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b10111100000u32 << 19u32
                    | self.a.into_inner() << 18u32
                    | self.b.into_inner() << 17u32
                    | self.c.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.cmode.into_inner() << 13u32
                    | 0b001u32 << 10u32
                    | self.d.into_inner() << 9u32
                    | self.e.into_inner() << 8u32
                    | self.f.into_inner() << 7u32
                    | self.g.into_inner() << 6u32
                    | self.h.into_inner() << 5u32
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
pub mod BIC_asimdimm_L_sl {
    pub const OPCODE_MASK: u32 = 0b10111111111110001001110000000000u32;
    pub const OPCODE: u32 = 0b00101111000000000001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BIC_asimdimm_L_sl";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BIC_asimdimm_L_sl {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub a: ::aarchmrs_types::BitValue<1>,
        pub b: ::aarchmrs_types::BitValue<1>,
        pub c: ::aarchmrs_types::BitValue<1>,
        pub cmode: ::aarchmrs_types::BitValue<2>,
        pub d: ::aarchmrs_types::BitValue<1>,
        pub e: ::aarchmrs_types::BitValue<1>,
        pub f: ::aarchmrs_types::BitValue<1>,
        pub g: ::aarchmrs_types::BitValue<1>,
        pub h: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl BIC_asimdimm_L_sl {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            a: ::aarchmrs_types::BitValue<1>,
            b: ::aarchmrs_types::BitValue<1>,
            c: ::aarchmrs_types::BitValue<1>,
            cmode: ::aarchmrs_types::BitValue<2>,
            d: ::aarchmrs_types::BitValue<1>,
            e: ::aarchmrs_types::BitValue<1>,
            f: ::aarchmrs_types::BitValue<1>,
            g: ::aarchmrs_types::BitValue<1>,
            h: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                a,
                b,
                c,
                cmode,
                d,
                e,
                f,
                g,
                h,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b10111100000u32 << 19u32
                    | self.a.into_inner() << 18u32
                    | self.b.into_inner() << 17u32
                    | self.c.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.cmode.into_inner() << 13u32
                    | 0b101u32 << 10u32
                    | self.d.into_inner() << 9u32
                    | self.e.into_inner() << 8u32
                    | self.f.into_inner() << 7u32
                    | self.g.into_inner() << 6u32
                    | self.h.into_inner() << 5u32
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
pub mod MVNI_asimdimm_L_hl {
    pub const OPCODE_MASK: u32 = 0b10111111111110001101110000000000u32;
    pub const OPCODE: u32 = 0b00101111000000001000010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MVNI_asimdimm_L_hl";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MVNI_asimdimm_L_hl {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub a: ::aarchmrs_types::BitValue<1>,
        pub b: ::aarchmrs_types::BitValue<1>,
        pub c: ::aarchmrs_types::BitValue<1>,
        pub cmode: ::aarchmrs_types::BitValue<1>,
        pub d: ::aarchmrs_types::BitValue<1>,
        pub e: ::aarchmrs_types::BitValue<1>,
        pub f: ::aarchmrs_types::BitValue<1>,
        pub g: ::aarchmrs_types::BitValue<1>,
        pub h: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MVNI_asimdimm_L_hl {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            a: ::aarchmrs_types::BitValue<1>,
            b: ::aarchmrs_types::BitValue<1>,
            c: ::aarchmrs_types::BitValue<1>,
            cmode: ::aarchmrs_types::BitValue<1>,
            d: ::aarchmrs_types::BitValue<1>,
            e: ::aarchmrs_types::BitValue<1>,
            f: ::aarchmrs_types::BitValue<1>,
            g: ::aarchmrs_types::BitValue<1>,
            h: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                a,
                b,
                c,
                cmode,
                d,
                e,
                f,
                g,
                h,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b10111100000u32 << 19u32
                    | self.a.into_inner() << 18u32
                    | self.b.into_inner() << 17u32
                    | self.c.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.cmode.into_inner() << 13u32
                    | 0b001u32 << 10u32
                    | self.d.into_inner() << 9u32
                    | self.e.into_inner() << 8u32
                    | self.f.into_inner() << 7u32
                    | self.g.into_inner() << 6u32
                    | self.h.into_inner() << 5u32
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
pub mod BIC_asimdimm_L_hl {
    pub const OPCODE_MASK: u32 = 0b10111111111110001101110000000000u32;
    pub const OPCODE: u32 = 0b00101111000000001001010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BIC_asimdimm_L_hl";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BIC_asimdimm_L_hl {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub a: ::aarchmrs_types::BitValue<1>,
        pub b: ::aarchmrs_types::BitValue<1>,
        pub c: ::aarchmrs_types::BitValue<1>,
        pub cmode: ::aarchmrs_types::BitValue<1>,
        pub d: ::aarchmrs_types::BitValue<1>,
        pub e: ::aarchmrs_types::BitValue<1>,
        pub f: ::aarchmrs_types::BitValue<1>,
        pub g: ::aarchmrs_types::BitValue<1>,
        pub h: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl BIC_asimdimm_L_hl {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            a: ::aarchmrs_types::BitValue<1>,
            b: ::aarchmrs_types::BitValue<1>,
            c: ::aarchmrs_types::BitValue<1>,
            cmode: ::aarchmrs_types::BitValue<1>,
            d: ::aarchmrs_types::BitValue<1>,
            e: ::aarchmrs_types::BitValue<1>,
            f: ::aarchmrs_types::BitValue<1>,
            g: ::aarchmrs_types::BitValue<1>,
            h: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                a,
                b,
                c,
                cmode,
                d,
                e,
                f,
                g,
                h,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b10111100000u32 << 19u32
                    | self.a.into_inner() << 18u32
                    | self.b.into_inner() << 17u32
                    | self.c.into_inner() << 16u32
                    | 0b10u32 << 14u32
                    | self.cmode.into_inner() << 13u32
                    | 0b101u32 << 10u32
                    | self.d.into_inner() << 9u32
                    | self.e.into_inner() << 8u32
                    | self.f.into_inner() << 7u32
                    | self.g.into_inner() << 6u32
                    | self.h.into_inner() << 5u32
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
pub mod MVNI_asimdimm_M_sm {
    pub const OPCODE_MASK: u32 = 0b10111111111110001110110000000000u32;
    pub const OPCODE: u32 = 0b00101111000000001100010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MVNI_asimdimm_M_sm";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MVNI_asimdimm_M_sm {
        pub Q: ::aarchmrs_types::BitValue<1>,
        pub a: ::aarchmrs_types::BitValue<1>,
        pub b: ::aarchmrs_types::BitValue<1>,
        pub c: ::aarchmrs_types::BitValue<1>,
        pub cmode: ::aarchmrs_types::BitValue<1>,
        pub d: ::aarchmrs_types::BitValue<1>,
        pub e: ::aarchmrs_types::BitValue<1>,
        pub f: ::aarchmrs_types::BitValue<1>,
        pub g: ::aarchmrs_types::BitValue<1>,
        pub h: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MVNI_asimdimm_M_sm {
        #[inline]
        pub const fn new(
            Q: ::aarchmrs_types::BitValue<1>,
            a: ::aarchmrs_types::BitValue<1>,
            b: ::aarchmrs_types::BitValue<1>,
            c: ::aarchmrs_types::BitValue<1>,
            cmode: ::aarchmrs_types::BitValue<1>,
            d: ::aarchmrs_types::BitValue<1>,
            e: ::aarchmrs_types::BitValue<1>,
            f: ::aarchmrs_types::BitValue<1>,
            g: ::aarchmrs_types::BitValue<1>,
            h: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Q,
                a,
                b,
                c,
                cmode,
                d,
                e,
                f,
                g,
                h,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | self.Q.into_inner() << 30u32
                    | 0b10111100000u32 << 19u32
                    | self.a.into_inner() << 18u32
                    | self.b.into_inner() << 17u32
                    | self.c.into_inner() << 16u32
                    | 0b110u32 << 13u32
                    | self.cmode.into_inner() << 12u32
                    | 0b01u32 << 10u32
                    | self.d.into_inner() << 9u32
                    | self.e.into_inner() << 8u32
                    | self.f.into_inner() << 7u32
                    | self.g.into_inner() << 6u32
                    | self.h.into_inner() << 5u32
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
pub mod MOVI_asimdimm_D_ds {
    pub const OPCODE_MASK: u32 = 0b11111111111110001111110000000000u32;
    pub const OPCODE: u32 = 0b00101111000000001110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MOVI_asimdimm_D_ds";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MOVI_asimdimm_D_ds {
        pub a: ::aarchmrs_types::BitValue<1>,
        pub b: ::aarchmrs_types::BitValue<1>,
        pub c: ::aarchmrs_types::BitValue<1>,
        pub d: ::aarchmrs_types::BitValue<1>,
        pub e: ::aarchmrs_types::BitValue<1>,
        pub f: ::aarchmrs_types::BitValue<1>,
        pub g: ::aarchmrs_types::BitValue<1>,
        pub h: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MOVI_asimdimm_D_ds {
        #[inline]
        pub const fn new(
            a: ::aarchmrs_types::BitValue<1>,
            b: ::aarchmrs_types::BitValue<1>,
            c: ::aarchmrs_types::BitValue<1>,
            d: ::aarchmrs_types::BitValue<1>,
            e: ::aarchmrs_types::BitValue<1>,
            f: ::aarchmrs_types::BitValue<1>,
            g: ::aarchmrs_types::BitValue<1>,
            h: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                a,
                b,
                c,
                d,
                e,
                f,
                g,
                h,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0010111100000u32 << 19u32
                    | self.a.into_inner() << 18u32
                    | self.b.into_inner() << 17u32
                    | self.c.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.d.into_inner() << 9u32
                    | self.e.into_inner() << 8u32
                    | self.f.into_inner() << 7u32
                    | self.g.into_inner() << 6u32
                    | self.h.into_inner() << 5u32
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
pub mod MOVI_asimdimm_D2_d {
    pub const OPCODE_MASK: u32 = 0b11111111111110001111110000000000u32;
    pub const OPCODE: u32 = 0b01101111000000001110010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "MOVI_asimdimm_D2_d";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MOVI_asimdimm_D2_d {
        pub a: ::aarchmrs_types::BitValue<1>,
        pub b: ::aarchmrs_types::BitValue<1>,
        pub c: ::aarchmrs_types::BitValue<1>,
        pub d: ::aarchmrs_types::BitValue<1>,
        pub e: ::aarchmrs_types::BitValue<1>,
        pub f: ::aarchmrs_types::BitValue<1>,
        pub g: ::aarchmrs_types::BitValue<1>,
        pub h: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl MOVI_asimdimm_D2_d {
        #[inline]
        pub const fn new(
            a: ::aarchmrs_types::BitValue<1>,
            b: ::aarchmrs_types::BitValue<1>,
            c: ::aarchmrs_types::BitValue<1>,
            d: ::aarchmrs_types::BitValue<1>,
            e: ::aarchmrs_types::BitValue<1>,
            f: ::aarchmrs_types::BitValue<1>,
            g: ::aarchmrs_types::BitValue<1>,
            h: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                a,
                b,
                c,
                d,
                e,
                f,
                g,
                h,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0110111100000u32 << 19u32
                    | self.a.into_inner() << 18u32
                    | self.b.into_inner() << 17u32
                    | self.c.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.d.into_inner() << 9u32
                    | self.e.into_inner() << 8u32
                    | self.f.into_inner() << 7u32
                    | self.g.into_inner() << 6u32
                    | self.h.into_inner() << 5u32
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
pub mod FMOV_asimdimm_D2_d {
    pub const OPCODE_MASK: u32 = 0b11111111111110001111110000000000u32;
    pub const OPCODE: u32 = 0b01101111000000001111010000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "FMOV_asimdimm_D2_d";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMOV_asimdimm_D2_d {
        pub a: ::aarchmrs_types::BitValue<1>,
        pub b: ::aarchmrs_types::BitValue<1>,
        pub c: ::aarchmrs_types::BitValue<1>,
        pub d: ::aarchmrs_types::BitValue<1>,
        pub e: ::aarchmrs_types::BitValue<1>,
        pub f: ::aarchmrs_types::BitValue<1>,
        pub g: ::aarchmrs_types::BitValue<1>,
        pub h: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMOV_asimdimm_D2_d {
        #[inline]
        pub const fn new(
            a: ::aarchmrs_types::BitValue<1>,
            b: ::aarchmrs_types::BitValue<1>,
            c: ::aarchmrs_types::BitValue<1>,
            d: ::aarchmrs_types::BitValue<1>,
            e: ::aarchmrs_types::BitValue<1>,
            f: ::aarchmrs_types::BitValue<1>,
            g: ::aarchmrs_types::BitValue<1>,
            h: ::aarchmrs_types::BitValue<1>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                a,
                b,
                c,
                d,
                e,
                f,
                g,
                h,
                Rd,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0110111100000u32 << 19u32
                    | self.a.into_inner() << 18u32
                    | self.b.into_inner() << 17u32
                    | self.c.into_inner() << 16u32
                    | 0b111101u32 << 10u32
                    | self.d.into_inner() << 9u32
                    | self.e.into_inner() << 8u32
                    | self.f.into_inner() << 7u32
                    | self.g.into_inner() << 6u32
                    | self.h.into_inner() << 5u32
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
