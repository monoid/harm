/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MOVI_asimdimm_L_sl {
    #[inline]
    pub fn MOVI_asimdimm_L_sl(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00111100000u32 << 19u32
                | u32::from(a.into()) << 18u32
                | u32::from(b.into()) << 17u32
                | u32::from(c.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(cmode.into()) << 13u32
                | 0b001u32 << 10u32
                | u32::from(d.into()) << 9u32
                | u32::from(e.into()) << 8u32
                | u32::from(f.into()) << 7u32
                | u32::from(g.into()) << 6u32
                | u32::from(h.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ORR_asimdimm_L_sl {
    #[inline]
    pub fn ORR_asimdimm_L_sl(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00111100000u32 << 19u32
                | u32::from(a.into()) << 18u32
                | u32::from(b.into()) << 17u32
                | u32::from(c.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(cmode.into()) << 13u32
                | 0b101u32 << 10u32
                | u32::from(d.into()) << 9u32
                | u32::from(e.into()) << 8u32
                | u32::from(f.into()) << 7u32
                | u32::from(g.into()) << 6u32
                | u32::from(h.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod MOVI_asimdimm_L_hl {
    #[inline]
    pub fn MOVI_asimdimm_L_hl(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00111100000u32 << 19u32
                | u32::from(a.into()) << 18u32
                | u32::from(b.into()) << 17u32
                | u32::from(c.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(cmode.into()) << 13u32
                | 0b001u32 << 10u32
                | u32::from(d.into()) << 9u32
                | u32::from(e.into()) << 8u32
                | u32::from(f.into()) << 7u32
                | u32::from(g.into()) << 6u32
                | u32::from(h.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ORR_asimdimm_L_hl {
    #[inline]
    pub fn ORR_asimdimm_L_hl(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00111100000u32 << 19u32
                | u32::from(a.into()) << 18u32
                | u32::from(b.into()) << 17u32
                | u32::from(c.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(cmode.into()) << 13u32
                | 0b101u32 << 10u32
                | u32::from(d.into()) << 9u32
                | u32::from(e.into()) << 8u32
                | u32::from(f.into()) << 7u32
                | u32::from(g.into()) << 6u32
                | u32::from(h.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod MOVI_asimdimm_M_sm {
    #[inline]
    pub fn MOVI_asimdimm_M_sm(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00111100000u32 << 19u32
                | u32::from(a.into()) << 18u32
                | u32::from(b.into()) << 17u32
                | u32::from(c.into()) << 16u32
                | 0b110u32 << 13u32
                | u32::from(cmode.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(d.into()) << 9u32
                | u32::from(e.into()) << 8u32
                | u32::from(f.into()) << 7u32
                | u32::from(g.into()) << 6u32
                | u32::from(h.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod MOVI_asimdimm_N_b {
    #[inline]
    pub fn MOVI_asimdimm_N_b(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00111100000u32 << 19u32
                | u32::from(a.into()) << 18u32
                | u32::from(b.into()) << 17u32
                | u32::from(c.into()) << 16u32
                | 0b111001u32 << 10u32
                | u32::from(d.into()) << 9u32
                | u32::from(e.into()) << 8u32
                | u32::from(f.into()) << 7u32
                | u32::from(g.into()) << 6u32
                | u32::from(h.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMOV_asimdimm_S_s {
    #[inline]
    pub fn FMOV_asimdimm_S_s(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00111100000u32 << 19u32
                | u32::from(a.into()) << 18u32
                | u32::from(b.into()) << 17u32
                | u32::from(c.into()) << 16u32
                | 0b111101u32 << 10u32
                | u32::from(d.into()) << 9u32
                | u32::from(e.into()) << 8u32
                | u32::from(f.into()) << 7u32
                | u32::from(g.into()) << 6u32
                | u32::from(h.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMOV_asimdimm_H_h {
    #[inline]
    pub fn FMOV_asimdimm_H_h(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00111100000u32 << 19u32
                | u32::from(a.into()) << 18u32
                | u32::from(b.into()) << 17u32
                | u32::from(c.into()) << 16u32
                | 0b111111u32 << 10u32
                | u32::from(d.into()) << 9u32
                | u32::from(e.into()) << 8u32
                | u32::from(f.into()) << 7u32
                | u32::from(g.into()) << 6u32
                | u32::from(h.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod MVNI_asimdimm_L_sl {
    #[inline]
    pub fn MVNI_asimdimm_L_sl(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b10111100000u32 << 19u32
                | u32::from(a.into()) << 18u32
                | u32::from(b.into()) << 17u32
                | u32::from(c.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(cmode.into()) << 13u32
                | 0b001u32 << 10u32
                | u32::from(d.into()) << 9u32
                | u32::from(e.into()) << 8u32
                | u32::from(f.into()) << 7u32
                | u32::from(g.into()) << 6u32
                | u32::from(h.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod BIC_asimdimm_L_sl {
    #[inline]
    pub fn BIC_asimdimm_L_sl(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b10111100000u32 << 19u32
                | u32::from(a.into()) << 18u32
                | u32::from(b.into()) << 17u32
                | u32::from(c.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(cmode.into()) << 13u32
                | 0b101u32 << 10u32
                | u32::from(d.into()) << 9u32
                | u32::from(e.into()) << 8u32
                | u32::from(f.into()) << 7u32
                | u32::from(g.into()) << 6u32
                | u32::from(h.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod MVNI_asimdimm_L_hl {
    #[inline]
    pub fn MVNI_asimdimm_L_hl(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b10111100000u32 << 19u32
                | u32::from(a.into()) << 18u32
                | u32::from(b.into()) << 17u32
                | u32::from(c.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(cmode.into()) << 13u32
                | 0b001u32 << 10u32
                | u32::from(d.into()) << 9u32
                | u32::from(e.into()) << 8u32
                | u32::from(f.into()) << 7u32
                | u32::from(g.into()) << 6u32
                | u32::from(h.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod BIC_asimdimm_L_hl {
    #[inline]
    pub fn BIC_asimdimm_L_hl(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b10111100000u32 << 19u32
                | u32::from(a.into()) << 18u32
                | u32::from(b.into()) << 17u32
                | u32::from(c.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(cmode.into()) << 13u32
                | 0b101u32 << 10u32
                | u32::from(d.into()) << 9u32
                | u32::from(e.into()) << 8u32
                | u32::from(f.into()) << 7u32
                | u32::from(g.into()) << 6u32
                | u32::from(h.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod MVNI_asimdimm_M_sm {
    #[inline]
    pub fn MVNI_asimdimm_M_sm(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b10111100000u32 << 19u32
                | u32::from(a.into()) << 18u32
                | u32::from(b.into()) << 17u32
                | u32::from(c.into()) << 16u32
                | 0b110u32 << 13u32
                | u32::from(cmode.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(d.into()) << 9u32
                | u32::from(e.into()) << 8u32
                | u32::from(f.into()) << 7u32
                | u32::from(g.into()) << 6u32
                | u32::from(h.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod MOVI_asimdimm_D_ds {
    #[inline]
    pub fn MOVI_asimdimm_D_ds(
        a: impl Into<::aarchmrs_types::BitValue<1>>,
        b: impl Into<::aarchmrs_types::BitValue<1>>,
        c: impl Into<::aarchmrs_types::BitValue<1>>,
        d: impl Into<::aarchmrs_types::BitValue<1>>,
        e: impl Into<::aarchmrs_types::BitValue<1>>,
        f: impl Into<::aarchmrs_types::BitValue<1>>,
        g: impl Into<::aarchmrs_types::BitValue<1>>,
        h: impl Into<::aarchmrs_types::BitValue<1>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010111100000u32 << 19u32
                | u32::from(a.into()) << 18u32
                | u32::from(b.into()) << 17u32
                | u32::from(c.into()) << 16u32
                | 0b111001u32 << 10u32
                | u32::from(d.into()) << 9u32
                | u32::from(e.into()) << 8u32
                | u32::from(f.into()) << 7u32
                | u32::from(g.into()) << 6u32
                | u32::from(h.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod MOVI_asimdimm_D2_d {
    #[inline]
    pub fn MOVI_asimdimm_D2_d(
        a: impl Into<::aarchmrs_types::BitValue<1>>,
        b: impl Into<::aarchmrs_types::BitValue<1>>,
        c: impl Into<::aarchmrs_types::BitValue<1>>,
        d: impl Into<::aarchmrs_types::BitValue<1>>,
        e: impl Into<::aarchmrs_types::BitValue<1>>,
        f: impl Into<::aarchmrs_types::BitValue<1>>,
        g: impl Into<::aarchmrs_types::BitValue<1>>,
        h: impl Into<::aarchmrs_types::BitValue<1>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110111100000u32 << 19u32
                | u32::from(a.into()) << 18u32
                | u32::from(b.into()) << 17u32
                | u32::from(c.into()) << 16u32
                | 0b111001u32 << 10u32
                | u32::from(d.into()) << 9u32
                | u32::from(e.into()) << 8u32
                | u32::from(f.into()) << 7u32
                | u32::from(g.into()) << 6u32
                | u32::from(h.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMOV_asimdimm_D2_d {
    #[inline]
    pub fn FMOV_asimdimm_D2_d(
        a: impl Into<::aarchmrs_types::BitValue<1>>,
        b: impl Into<::aarchmrs_types::BitValue<1>>,
        c: impl Into<::aarchmrs_types::BitValue<1>>,
        d: impl Into<::aarchmrs_types::BitValue<1>>,
        e: impl Into<::aarchmrs_types::BitValue<1>>,
        f: impl Into<::aarchmrs_types::BitValue<1>>,
        g: impl Into<::aarchmrs_types::BitValue<1>>,
        h: impl Into<::aarchmrs_types::BitValue<1>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110111100000u32 << 19u32
                | u32::from(a.into()) << 18u32
                | u32::from(b.into()) << 17u32
                | u32::from(c.into()) << 16u32
                | 0b111101u32 << 10u32
                | u32::from(d.into()) << 9u32
                | u32::from(e.into()) << 8u32
                | u32::from(f.into()) << 7u32
                | u32::from(g.into()) << 6u32
                | u32::from(h.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
