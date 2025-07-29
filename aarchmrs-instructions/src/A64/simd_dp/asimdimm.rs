/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MOVI_asimdimm_L_sl {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111110001001110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOVI_asimdimm_L_sl";
    #[inline]
    pub const fn MOVI_asimdimm_L_sl(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111100000u32 << 19u32
                | a.into_inner() << 18u32
                | b.into_inner() << 17u32
                | c.into_inner() << 16u32
                | 0b0u32 << 15u32
                | cmode.into_inner() << 13u32
                | 0b001u32 << 10u32
                | d.into_inner() << 9u32
                | e.into_inner() << 8u32
                | f.into_inner() << 7u32
                | g.into_inner() << 6u32
                | h.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ORR_asimdimm_L_sl {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111110001001110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000000001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORR_asimdimm_L_sl";
    #[inline]
    pub const fn ORR_asimdimm_L_sl(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111100000u32 << 19u32
                | a.into_inner() << 18u32
                | b.into_inner() << 17u32
                | c.into_inner() << 16u32
                | 0b0u32 << 15u32
                | cmode.into_inner() << 13u32
                | 0b101u32 << 10u32
                | d.into_inner() << 9u32
                | e.into_inner() << 8u32
                | f.into_inner() << 7u32
                | g.into_inner() << 6u32
                | h.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MOVI_asimdimm_L_hl {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111110001101110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOVI_asimdimm_L_hl";
    #[inline]
    pub const fn MOVI_asimdimm_L_hl(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111100000u32 << 19u32
                | a.into_inner() << 18u32
                | b.into_inner() << 17u32
                | c.into_inner() << 16u32
                | 0b10u32 << 14u32
                | cmode.into_inner() << 13u32
                | 0b001u32 << 10u32
                | d.into_inner() << 9u32
                | e.into_inner() << 8u32
                | f.into_inner() << 7u32
                | g.into_inner() << 6u32
                | h.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ORR_asimdimm_L_hl {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111110001101110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORR_asimdimm_L_hl";
    #[inline]
    pub const fn ORR_asimdimm_L_hl(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111100000u32 << 19u32
                | a.into_inner() << 18u32
                | b.into_inner() << 17u32
                | c.into_inner() << 16u32
                | 0b10u32 << 14u32
                | cmode.into_inner() << 13u32
                | 0b101u32 << 10u32
                | d.into_inner() << 9u32
                | e.into_inner() << 8u32
                | f.into_inner() << 7u32
                | g.into_inner() << 6u32
                | h.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MOVI_asimdimm_M_sm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111110001110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOVI_asimdimm_M_sm";
    #[inline]
    pub const fn MOVI_asimdimm_M_sm(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111100000u32 << 19u32
                | a.into_inner() << 18u32
                | b.into_inner() << 17u32
                | c.into_inner() << 16u32
                | 0b110u32 << 13u32
                | cmode.into_inner() << 12u32
                | 0b01u32 << 10u32
                | d.into_inner() << 9u32
                | e.into_inner() << 8u32
                | f.into_inner() << 7u32
                | g.into_inner() << 6u32
                | h.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MOVI_asimdimm_N_b {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111110001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOVI_asimdimm_N_b";
    #[inline]
    pub const fn MOVI_asimdimm_N_b(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111100000u32 << 19u32
                | a.into_inner() << 18u32
                | b.into_inner() << 17u32
                | c.into_inner() << 16u32
                | 0b111001u32 << 10u32
                | d.into_inner() << 9u32
                | e.into_inner() << 8u32
                | f.into_inner() << 7u32
                | g.into_inner() << 6u32
                | h.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMOV_asimdimm_S_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111110001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMOV_asimdimm_S_s";
    #[inline]
    pub const fn FMOV_asimdimm_S_s(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111100000u32 << 19u32
                | a.into_inner() << 18u32
                | b.into_inner() << 17u32
                | c.into_inner() << 16u32
                | 0b111101u32 << 10u32
                | d.into_inner() << 9u32
                | e.into_inner() << 8u32
                | f.into_inner() << 7u32
                | g.into_inner() << 6u32
                | h.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMOV_asimdimm_H_h {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111110001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMOV_asimdimm_H_h";
    #[inline]
    pub const fn FMOV_asimdimm_H_h(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111100000u32 << 19u32
                | a.into_inner() << 18u32
                | b.into_inner() << 17u32
                | c.into_inner() << 16u32
                | 0b111111u32 << 10u32
                | d.into_inner() << 9u32
                | e.into_inner() << 8u32
                | f.into_inner() << 7u32
                | g.into_inner() << 6u32
                | h.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MVNI_asimdimm_L_sl {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111110001001110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MVNI_asimdimm_L_sl";
    #[inline]
    pub const fn MVNI_asimdimm_L_sl(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111100000u32 << 19u32
                | a.into_inner() << 18u32
                | b.into_inner() << 17u32
                | c.into_inner() << 16u32
                | 0b0u32 << 15u32
                | cmode.into_inner() << 13u32
                | 0b001u32 << 10u32
                | d.into_inner() << 9u32
                | e.into_inner() << 8u32
                | f.into_inner() << 7u32
                | g.into_inner() << 6u32
                | h.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod BIC_asimdimm_L_sl {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111110001001110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000000001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BIC_asimdimm_L_sl";
    #[inline]
    pub const fn BIC_asimdimm_L_sl(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111100000u32 << 19u32
                | a.into_inner() << 18u32
                | b.into_inner() << 17u32
                | c.into_inner() << 16u32
                | 0b0u32 << 15u32
                | cmode.into_inner() << 13u32
                | 0b101u32 << 10u32
                | d.into_inner() << 9u32
                | e.into_inner() << 8u32
                | f.into_inner() << 7u32
                | g.into_inner() << 6u32
                | h.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MVNI_asimdimm_L_hl {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111110001101110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MVNI_asimdimm_L_hl";
    #[inline]
    pub const fn MVNI_asimdimm_L_hl(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111100000u32 << 19u32
                | a.into_inner() << 18u32
                | b.into_inner() << 17u32
                | c.into_inner() << 16u32
                | 0b10u32 << 14u32
                | cmode.into_inner() << 13u32
                | 0b001u32 << 10u32
                | d.into_inner() << 9u32
                | e.into_inner() << 8u32
                | f.into_inner() << 7u32
                | g.into_inner() << 6u32
                | h.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod BIC_asimdimm_L_hl {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111110001101110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BIC_asimdimm_L_hl";
    #[inline]
    pub const fn BIC_asimdimm_L_hl(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111100000u32 << 19u32
                | a.into_inner() << 18u32
                | b.into_inner() << 17u32
                | c.into_inner() << 16u32
                | 0b10u32 << 14u32
                | cmode.into_inner() << 13u32
                | 0b101u32 << 10u32
                | d.into_inner() << 9u32
                | e.into_inner() << 8u32
                | f.into_inner() << 7u32
                | g.into_inner() << 6u32
                | h.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MVNI_asimdimm_M_sm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111110001110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000001100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MVNI_asimdimm_M_sm";
    #[inline]
    pub const fn MVNI_asimdimm_M_sm(
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
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111100000u32 << 19u32
                | a.into_inner() << 18u32
                | b.into_inner() << 17u32
                | c.into_inner() << 16u32
                | 0b110u32 << 13u32
                | cmode.into_inner() << 12u32
                | 0b01u32 << 10u32
                | d.into_inner() << 9u32
                | e.into_inner() << 8u32
                | f.into_inner() << 7u32
                | g.into_inner() << 6u32
                | h.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MOVI_asimdimm_D_ds {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111110001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOVI_asimdimm_D_ds";
    #[inline]
    pub const fn MOVI_asimdimm_D_ds(
        a: ::aarchmrs_types::BitValue<1>,
        b: ::aarchmrs_types::BitValue<1>,
        c: ::aarchmrs_types::BitValue<1>,
        d: ::aarchmrs_types::BitValue<1>,
        e: ::aarchmrs_types::BitValue<1>,
        f: ::aarchmrs_types::BitValue<1>,
        g: ::aarchmrs_types::BitValue<1>,
        h: ::aarchmrs_types::BitValue<1>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0010111100000u32 << 19u32
                | a.into_inner() << 18u32
                | b.into_inner() << 17u32
                | c.into_inner() << 16u32
                | 0b111001u32 << 10u32
                | d.into_inner() << 9u32
                | e.into_inner() << 8u32
                | f.into_inner() << 7u32
                | g.into_inner() << 6u32
                | h.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod MOVI_asimdimm_D2_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111110001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01101111000000001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOVI_asimdimm_D2_d";
    #[inline]
    pub const fn MOVI_asimdimm_D2_d(
        a: ::aarchmrs_types::BitValue<1>,
        b: ::aarchmrs_types::BitValue<1>,
        c: ::aarchmrs_types::BitValue<1>,
        d: ::aarchmrs_types::BitValue<1>,
        e: ::aarchmrs_types::BitValue<1>,
        f: ::aarchmrs_types::BitValue<1>,
        g: ::aarchmrs_types::BitValue<1>,
        h: ::aarchmrs_types::BitValue<1>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110111100000u32 << 19u32
                | a.into_inner() << 18u32
                | b.into_inner() << 17u32
                | c.into_inner() << 16u32
                | 0b111001u32 << 10u32
                | d.into_inner() << 9u32
                | e.into_inner() << 8u32
                | f.into_inner() << 7u32
                | g.into_inner() << 6u32
                | h.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FMOV_asimdimm_D2_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111110001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01101111000000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FMOV_asimdimm_D2_d";
    #[inline]
    pub const fn FMOV_asimdimm_D2_d(
        a: ::aarchmrs_types::BitValue<1>,
        b: ::aarchmrs_types::BitValue<1>,
        c: ::aarchmrs_types::BitValue<1>,
        d: ::aarchmrs_types::BitValue<1>,
        e: ::aarchmrs_types::BitValue<1>,
        f: ::aarchmrs_types::BitValue<1>,
        g: ::aarchmrs_types::BitValue<1>,
        h: ::aarchmrs_types::BitValue<1>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0110111100000u32 << 19u32
                | a.into_inner() << 18u32
                | b.into_inner() << 17u32
                | c.into_inner() << 16u32
                | 0b111101u32 << 10u32
                | d.into_inner() << 9u32
                | e.into_inner() << 8u32
                | f.into_inner() << 7u32
                | g.into_inner() << 6u32
                | h.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
