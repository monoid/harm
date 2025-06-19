/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MOVI_asimdimm_L_sl {
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
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            a: impl Into<::aarchmrs_types::BitValue<1>>,
            b: impl Into<::aarchmrs_types::BitValue<1>>,
            c: impl Into<::aarchmrs_types::BitValue<1>>,
            cmode: impl Into<::aarchmrs_types::BitValue<2>>,
            d: impl Into<::aarchmrs_types::BitValue<1>>,
            e: impl Into<::aarchmrs_types::BitValue<1>>,
            f: impl Into<::aarchmrs_types::BitValue<1>>,
            g: impl Into<::aarchmrs_types::BitValue<1>>,
            h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                a: a.into(),
                b: b.into(),
                c: c.into(),
                cmode: cmode.into(),
                d: d.into(),
                e: e.into(),
                f: f.into(),
                g: g.into(),
                h: h.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b00111100000u32 << 19u32
                    | u32::from(self.a) << 18u32
                    | u32::from(self.b) << 17u32
                    | u32::from(self.c) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.cmode) << 13u32
                    | 0b001u32 << 10u32
                    | u32::from(self.d) << 9u32
                    | u32::from(self.e) << 8u32
                    | u32::from(self.f) << 7u32
                    | u32::from(self.g) << 6u32
                    | u32::from(self.h) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ORR_asimdimm_L_sl {
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
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            a: impl Into<::aarchmrs_types::BitValue<1>>,
            b: impl Into<::aarchmrs_types::BitValue<1>>,
            c: impl Into<::aarchmrs_types::BitValue<1>>,
            cmode: impl Into<::aarchmrs_types::BitValue<2>>,
            d: impl Into<::aarchmrs_types::BitValue<1>>,
            e: impl Into<::aarchmrs_types::BitValue<1>>,
            f: impl Into<::aarchmrs_types::BitValue<1>>,
            g: impl Into<::aarchmrs_types::BitValue<1>>,
            h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                a: a.into(),
                b: b.into(),
                c: c.into(),
                cmode: cmode.into(),
                d: d.into(),
                e: e.into(),
                f: f.into(),
                g: g.into(),
                h: h.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b00111100000u32 << 19u32
                    | u32::from(self.a) << 18u32
                    | u32::from(self.b) << 17u32
                    | u32::from(self.c) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.cmode) << 13u32
                    | 0b101u32 << 10u32
                    | u32::from(self.d) << 9u32
                    | u32::from(self.e) << 8u32
                    | u32::from(self.f) << 7u32
                    | u32::from(self.g) << 6u32
                    | u32::from(self.h) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod MOVI_asimdimm_L_hl {
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
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            a: impl Into<::aarchmrs_types::BitValue<1>>,
            b: impl Into<::aarchmrs_types::BitValue<1>>,
            c: impl Into<::aarchmrs_types::BitValue<1>>,
            cmode: impl Into<::aarchmrs_types::BitValue<1>>,
            d: impl Into<::aarchmrs_types::BitValue<1>>,
            e: impl Into<::aarchmrs_types::BitValue<1>>,
            f: impl Into<::aarchmrs_types::BitValue<1>>,
            g: impl Into<::aarchmrs_types::BitValue<1>>,
            h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                a: a.into(),
                b: b.into(),
                c: c.into(),
                cmode: cmode.into(),
                d: d.into(),
                e: e.into(),
                f: f.into(),
                g: g.into(),
                h: h.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b00111100000u32 << 19u32
                    | u32::from(self.a) << 18u32
                    | u32::from(self.b) << 17u32
                    | u32::from(self.c) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.cmode) << 13u32
                    | 0b001u32 << 10u32
                    | u32::from(self.d) << 9u32
                    | u32::from(self.e) << 8u32
                    | u32::from(self.f) << 7u32
                    | u32::from(self.g) << 6u32
                    | u32::from(self.h) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ORR_asimdimm_L_hl {
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
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            a: impl Into<::aarchmrs_types::BitValue<1>>,
            b: impl Into<::aarchmrs_types::BitValue<1>>,
            c: impl Into<::aarchmrs_types::BitValue<1>>,
            cmode: impl Into<::aarchmrs_types::BitValue<1>>,
            d: impl Into<::aarchmrs_types::BitValue<1>>,
            e: impl Into<::aarchmrs_types::BitValue<1>>,
            f: impl Into<::aarchmrs_types::BitValue<1>>,
            g: impl Into<::aarchmrs_types::BitValue<1>>,
            h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                a: a.into(),
                b: b.into(),
                c: c.into(),
                cmode: cmode.into(),
                d: d.into(),
                e: e.into(),
                f: f.into(),
                g: g.into(),
                h: h.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b00111100000u32 << 19u32
                    | u32::from(self.a) << 18u32
                    | u32::from(self.b) << 17u32
                    | u32::from(self.c) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.cmode) << 13u32
                    | 0b101u32 << 10u32
                    | u32::from(self.d) << 9u32
                    | u32::from(self.e) << 8u32
                    | u32::from(self.f) << 7u32
                    | u32::from(self.g) << 6u32
                    | u32::from(self.h) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod MOVI_asimdimm_M_sm {
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
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            a: impl Into<::aarchmrs_types::BitValue<1>>,
            b: impl Into<::aarchmrs_types::BitValue<1>>,
            c: impl Into<::aarchmrs_types::BitValue<1>>,
            cmode: impl Into<::aarchmrs_types::BitValue<1>>,
            d: impl Into<::aarchmrs_types::BitValue<1>>,
            e: impl Into<::aarchmrs_types::BitValue<1>>,
            f: impl Into<::aarchmrs_types::BitValue<1>>,
            g: impl Into<::aarchmrs_types::BitValue<1>>,
            h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                a: a.into(),
                b: b.into(),
                c: c.into(),
                cmode: cmode.into(),
                d: d.into(),
                e: e.into(),
                f: f.into(),
                g: g.into(),
                h: h.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b00111100000u32 << 19u32
                    | u32::from(self.a) << 18u32
                    | u32::from(self.b) << 17u32
                    | u32::from(self.c) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.cmode) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.d) << 9u32
                    | u32::from(self.e) << 8u32
                    | u32::from(self.f) << 7u32
                    | u32::from(self.g) << 6u32
                    | u32::from(self.h) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod MOVI_asimdimm_N_b {
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
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            a: impl Into<::aarchmrs_types::BitValue<1>>,
            b: impl Into<::aarchmrs_types::BitValue<1>>,
            c: impl Into<::aarchmrs_types::BitValue<1>>,
            d: impl Into<::aarchmrs_types::BitValue<1>>,
            e: impl Into<::aarchmrs_types::BitValue<1>>,
            f: impl Into<::aarchmrs_types::BitValue<1>>,
            g: impl Into<::aarchmrs_types::BitValue<1>>,
            h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                a: a.into(),
                b: b.into(),
                c: c.into(),
                d: d.into(),
                e: e.into(),
                f: f.into(),
                g: g.into(),
                h: h.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b00111100000u32 << 19u32
                    | u32::from(self.a) << 18u32
                    | u32::from(self.b) << 17u32
                    | u32::from(self.c) << 16u32
                    | 0b111001u32 << 10u32
                    | u32::from(self.d) << 9u32
                    | u32::from(self.e) << 8u32
                    | u32::from(self.f) << 7u32
                    | u32::from(self.g) << 6u32
                    | u32::from(self.h) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMOV_asimdimm_S_s {
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
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            a: impl Into<::aarchmrs_types::BitValue<1>>,
            b: impl Into<::aarchmrs_types::BitValue<1>>,
            c: impl Into<::aarchmrs_types::BitValue<1>>,
            d: impl Into<::aarchmrs_types::BitValue<1>>,
            e: impl Into<::aarchmrs_types::BitValue<1>>,
            f: impl Into<::aarchmrs_types::BitValue<1>>,
            g: impl Into<::aarchmrs_types::BitValue<1>>,
            h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                a: a.into(),
                b: b.into(),
                c: c.into(),
                d: d.into(),
                e: e.into(),
                f: f.into(),
                g: g.into(),
                h: h.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b00111100000u32 << 19u32
                    | u32::from(self.a) << 18u32
                    | u32::from(self.b) << 17u32
                    | u32::from(self.c) << 16u32
                    | 0b111101u32 << 10u32
                    | u32::from(self.d) << 9u32
                    | u32::from(self.e) << 8u32
                    | u32::from(self.f) << 7u32
                    | u32::from(self.g) << 6u32
                    | u32::from(self.h) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMOV_asimdimm_H_h {
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
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            a: impl Into<::aarchmrs_types::BitValue<1>>,
            b: impl Into<::aarchmrs_types::BitValue<1>>,
            c: impl Into<::aarchmrs_types::BitValue<1>>,
            d: impl Into<::aarchmrs_types::BitValue<1>>,
            e: impl Into<::aarchmrs_types::BitValue<1>>,
            f: impl Into<::aarchmrs_types::BitValue<1>>,
            g: impl Into<::aarchmrs_types::BitValue<1>>,
            h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                a: a.into(),
                b: b.into(),
                c: c.into(),
                d: d.into(),
                e: e.into(),
                f: f.into(),
                g: g.into(),
                h: h.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b00111100000u32 << 19u32
                    | u32::from(self.a) << 18u32
                    | u32::from(self.b) << 17u32
                    | u32::from(self.c) << 16u32
                    | 0b111111u32 << 10u32
                    | u32::from(self.d) << 9u32
                    | u32::from(self.e) << 8u32
                    | u32::from(self.f) << 7u32
                    | u32::from(self.g) << 6u32
                    | u32::from(self.h) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod MVNI_asimdimm_L_sl {
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
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            a: impl Into<::aarchmrs_types::BitValue<1>>,
            b: impl Into<::aarchmrs_types::BitValue<1>>,
            c: impl Into<::aarchmrs_types::BitValue<1>>,
            cmode: impl Into<::aarchmrs_types::BitValue<2>>,
            d: impl Into<::aarchmrs_types::BitValue<1>>,
            e: impl Into<::aarchmrs_types::BitValue<1>>,
            f: impl Into<::aarchmrs_types::BitValue<1>>,
            g: impl Into<::aarchmrs_types::BitValue<1>>,
            h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                a: a.into(),
                b: b.into(),
                c: c.into(),
                cmode: cmode.into(),
                d: d.into(),
                e: e.into(),
                f: f.into(),
                g: g.into(),
                h: h.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b10111100000u32 << 19u32
                    | u32::from(self.a) << 18u32
                    | u32::from(self.b) << 17u32
                    | u32::from(self.c) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.cmode) << 13u32
                    | 0b001u32 << 10u32
                    | u32::from(self.d) << 9u32
                    | u32::from(self.e) << 8u32
                    | u32::from(self.f) << 7u32
                    | u32::from(self.g) << 6u32
                    | u32::from(self.h) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod BIC_asimdimm_L_sl {
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
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            a: impl Into<::aarchmrs_types::BitValue<1>>,
            b: impl Into<::aarchmrs_types::BitValue<1>>,
            c: impl Into<::aarchmrs_types::BitValue<1>>,
            cmode: impl Into<::aarchmrs_types::BitValue<2>>,
            d: impl Into<::aarchmrs_types::BitValue<1>>,
            e: impl Into<::aarchmrs_types::BitValue<1>>,
            f: impl Into<::aarchmrs_types::BitValue<1>>,
            g: impl Into<::aarchmrs_types::BitValue<1>>,
            h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                a: a.into(),
                b: b.into(),
                c: c.into(),
                cmode: cmode.into(),
                d: d.into(),
                e: e.into(),
                f: f.into(),
                g: g.into(),
                h: h.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b10111100000u32 << 19u32
                    | u32::from(self.a) << 18u32
                    | u32::from(self.b) << 17u32
                    | u32::from(self.c) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.cmode) << 13u32
                    | 0b101u32 << 10u32
                    | u32::from(self.d) << 9u32
                    | u32::from(self.e) << 8u32
                    | u32::from(self.f) << 7u32
                    | u32::from(self.g) << 6u32
                    | u32::from(self.h) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod MVNI_asimdimm_L_hl {
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
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            a: impl Into<::aarchmrs_types::BitValue<1>>,
            b: impl Into<::aarchmrs_types::BitValue<1>>,
            c: impl Into<::aarchmrs_types::BitValue<1>>,
            cmode: impl Into<::aarchmrs_types::BitValue<1>>,
            d: impl Into<::aarchmrs_types::BitValue<1>>,
            e: impl Into<::aarchmrs_types::BitValue<1>>,
            f: impl Into<::aarchmrs_types::BitValue<1>>,
            g: impl Into<::aarchmrs_types::BitValue<1>>,
            h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                a: a.into(),
                b: b.into(),
                c: c.into(),
                cmode: cmode.into(),
                d: d.into(),
                e: e.into(),
                f: f.into(),
                g: g.into(),
                h: h.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b10111100000u32 << 19u32
                    | u32::from(self.a) << 18u32
                    | u32::from(self.b) << 17u32
                    | u32::from(self.c) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.cmode) << 13u32
                    | 0b001u32 << 10u32
                    | u32::from(self.d) << 9u32
                    | u32::from(self.e) << 8u32
                    | u32::from(self.f) << 7u32
                    | u32::from(self.g) << 6u32
                    | u32::from(self.h) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod BIC_asimdimm_L_hl {
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
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            a: impl Into<::aarchmrs_types::BitValue<1>>,
            b: impl Into<::aarchmrs_types::BitValue<1>>,
            c: impl Into<::aarchmrs_types::BitValue<1>>,
            cmode: impl Into<::aarchmrs_types::BitValue<1>>,
            d: impl Into<::aarchmrs_types::BitValue<1>>,
            e: impl Into<::aarchmrs_types::BitValue<1>>,
            f: impl Into<::aarchmrs_types::BitValue<1>>,
            g: impl Into<::aarchmrs_types::BitValue<1>>,
            h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                a: a.into(),
                b: b.into(),
                c: c.into(),
                cmode: cmode.into(),
                d: d.into(),
                e: e.into(),
                f: f.into(),
                g: g.into(),
                h: h.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b10111100000u32 << 19u32
                    | u32::from(self.a) << 18u32
                    | u32::from(self.b) << 17u32
                    | u32::from(self.c) << 16u32
                    | 0b10u32 << 14u32
                    | u32::from(self.cmode) << 13u32
                    | 0b101u32 << 10u32
                    | u32::from(self.d) << 9u32
                    | u32::from(self.e) << 8u32
                    | u32::from(self.f) << 7u32
                    | u32::from(self.g) << 6u32
                    | u32::from(self.h) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod MVNI_asimdimm_M_sm {
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
        pub fn new(
            Q: impl Into<::aarchmrs_types::BitValue<1>>,
            a: impl Into<::aarchmrs_types::BitValue<1>>,
            b: impl Into<::aarchmrs_types::BitValue<1>>,
            c: impl Into<::aarchmrs_types::BitValue<1>>,
            cmode: impl Into<::aarchmrs_types::BitValue<1>>,
            d: impl Into<::aarchmrs_types::BitValue<1>>,
            e: impl Into<::aarchmrs_types::BitValue<1>>,
            f: impl Into<::aarchmrs_types::BitValue<1>>,
            g: impl Into<::aarchmrs_types::BitValue<1>>,
            h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Q: Q.into(),
                a: a.into(),
                b: b.into(),
                c: c.into(),
                cmode: cmode.into(),
                d: d.into(),
                e: e.into(),
                f: f.into(),
                g: g.into(),
                h: h.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0u32 << 31u32
                    | u32::from(self.Q) << 30u32
                    | 0b10111100000u32 << 19u32
                    | u32::from(self.a) << 18u32
                    | u32::from(self.b) << 17u32
                    | u32::from(self.c) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.cmode) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.d) << 9u32
                    | u32::from(self.e) << 8u32
                    | u32::from(self.f) << 7u32
                    | u32::from(self.g) << 6u32
                    | u32::from(self.h) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod MOVI_asimdimm_D_ds {
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
        pub fn new(
            a: impl Into<::aarchmrs_types::BitValue<1>>,
            b: impl Into<::aarchmrs_types::BitValue<1>>,
            c: impl Into<::aarchmrs_types::BitValue<1>>,
            d: impl Into<::aarchmrs_types::BitValue<1>>,
            e: impl Into<::aarchmrs_types::BitValue<1>>,
            f: impl Into<::aarchmrs_types::BitValue<1>>,
            g: impl Into<::aarchmrs_types::BitValue<1>>,
            h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                a: a.into(),
                b: b.into(),
                c: c.into(),
                d: d.into(),
                e: e.into(),
                f: f.into(),
                g: g.into(),
                h: h.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0010111100000u32 << 19u32
                    | u32::from(self.a) << 18u32
                    | u32::from(self.b) << 17u32
                    | u32::from(self.c) << 16u32
                    | 0b111001u32 << 10u32
                    | u32::from(self.d) << 9u32
                    | u32::from(self.e) << 8u32
                    | u32::from(self.f) << 7u32
                    | u32::from(self.g) << 6u32
                    | u32::from(self.h) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod MOVI_asimdimm_D2_d {
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
        pub fn new(
            a: impl Into<::aarchmrs_types::BitValue<1>>,
            b: impl Into<::aarchmrs_types::BitValue<1>>,
            c: impl Into<::aarchmrs_types::BitValue<1>>,
            d: impl Into<::aarchmrs_types::BitValue<1>>,
            e: impl Into<::aarchmrs_types::BitValue<1>>,
            f: impl Into<::aarchmrs_types::BitValue<1>>,
            g: impl Into<::aarchmrs_types::BitValue<1>>,
            h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                a: a.into(),
                b: b.into(),
                c: c.into(),
                d: d.into(),
                e: e.into(),
                f: f.into(),
                g: g.into(),
                h: h.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0110111100000u32 << 19u32
                    | u32::from(self.a) << 18u32
                    | u32::from(self.b) << 17u32
                    | u32::from(self.c) << 16u32
                    | 0b111001u32 << 10u32
                    | u32::from(self.d) << 9u32
                    | u32::from(self.e) << 8u32
                    | u32::from(self.f) << 7u32
                    | u32::from(self.g) << 6u32
                    | u32::from(self.h) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod FMOV_asimdimm_D2_d {
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
        pub fn new(
            a: impl Into<::aarchmrs_types::BitValue<1>>,
            b: impl Into<::aarchmrs_types::BitValue<1>>,
            c: impl Into<::aarchmrs_types::BitValue<1>>,
            d: impl Into<::aarchmrs_types::BitValue<1>>,
            e: impl Into<::aarchmrs_types::BitValue<1>>,
            f: impl Into<::aarchmrs_types::BitValue<1>>,
            g: impl Into<::aarchmrs_types::BitValue<1>>,
            h: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                a: a.into(),
                b: b.into(),
                c: c.into(),
                d: d.into(),
                e: e.into(),
                f: f.into(),
                g: g.into(),
                h: h.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0110111100000u32 << 19u32
                    | u32::from(self.a) << 18u32
                    | u32::from(self.b) << 17u32
                    | u32::from(self.c) << 16u32
                    | 0b111101u32 << 10u32
                    | u32::from(self.d) << 9u32
                    | u32::from(self.e) << 8u32
                    | u32::from(self.f) << 7u32
                    | u32::from(self.g) << 6u32
                    | u32::from(self.h) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
