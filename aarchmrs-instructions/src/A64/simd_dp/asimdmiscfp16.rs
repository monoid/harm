/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FRINTN_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111011111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110011110011000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRINTN_asimdmiscfp16_R";
    #[inline]
    pub const fn FRINTN_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | o2.into_inner() << 23u32
                | 0b1111001100u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRINTM_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111011111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110011110011000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRINTM_asimdmiscfp16_R";
    #[inline]
    pub const fn FRINTM_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | o2.into_inner() << 23u32
                | 0b1111001100u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNS_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111011111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110011110011010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNS_asimdmiscfp16_R";
    #[inline]
    pub const fn FCVTNS_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | o2.into_inner() << 23u32
                | 0b1111001101u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMS_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111011111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110011110011010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMS_asimdmiscfp16_R";
    #[inline]
    pub const fn FCVTMS_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | o2.into_inner() << 23u32
                | 0b1111001101u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAS_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110011110011100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAS_asimdmiscfp16_R";
    #[inline]
    pub const fn FCVTAS_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111001111001110010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SCVTF_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110011110011101100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_asimdmiscfp16_R";
    #[inline]
    pub const fn SCVTF_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111001111001110110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCMGT_asimdmiscfp16_FZ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110111110001100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMGT_asimdmiscfp16_FZ";
    #[inline]
    pub const fn FCMGT_asimdmiscfp16_FZ(
        Q: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111011111000110u32 << 13u32
                | op.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCMEQ_asimdmiscfp16_FZ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110111110001100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMEQ_asimdmiscfp16_FZ";
    #[inline]
    pub const fn FCMEQ_asimdmiscfp16_FZ(
        Q: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111011111000110u32 << 13u32
                | op.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCMLT_asimdmiscfp16_FZ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110111110001110100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMLT_asimdmiscfp16_FZ";
    #[inline]
    pub const fn FCMLT_asimdmiscfp16_FZ(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111011111000111010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FABS_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110111110001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FABS_asimdmiscfp16_R";
    #[inline]
    pub const fn FABS_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111011111000111110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRINTP_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111011111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110011110011000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRINTP_asimdmiscfp16_R";
    #[inline]
    pub const fn FRINTP_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | o2.into_inner() << 23u32
                | 0b1111001100u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRINTZ_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111011111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110011110011000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRINTZ_asimdmiscfp16_R";
    #[inline]
    pub const fn FRINTZ_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | o2.into_inner() << 23u32
                | 0b1111001100u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPS_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111011111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110011110011010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPS_asimdmiscfp16_R";
    #[inline]
    pub const fn FCVTPS_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | o2.into_inner() << 23u32
                | 0b1111001101u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111011111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110011110011010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_asimdmiscfp16_R";
    #[inline]
    pub const fn FCVTZS_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | o2.into_inner() << 23u32
                | 0b1111001101u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRECPE_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110111110011101100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRECPE_asimdmiscfp16_R";
    #[inline]
    pub const fn FRECPE_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111011111001110110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRINTA_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111011111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110011110011000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRINTA_asimdmiscfp16_R";
    #[inline]
    pub const fn FRINTA_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | o2.into_inner() << 23u32
                | 0b1111001100u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRINTX_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111011111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110011110011000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRINTX_asimdmiscfp16_R";
    #[inline]
    pub const fn FRINTX_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | o2.into_inner() << 23u32
                | 0b1111001100u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNU_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111011111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110011110011010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNU_asimdmiscfp16_R";
    #[inline]
    pub const fn FCVTNU_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | o2.into_inner() << 23u32
                | 0b1111001101u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMU_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111011111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110011110011010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMU_asimdmiscfp16_R";
    #[inline]
    pub const fn FCVTMU_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | o2.into_inner() << 23u32
                | 0b1111001101u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAU_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110011110011100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAU_asimdmiscfp16_R";
    #[inline]
    pub const fn FCVTAU_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111001111001110010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110011110011101100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_asimdmiscfp16_R";
    #[inline]
    pub const fn UCVTF_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111001111001110110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCMGE_asimdmiscfp16_FZ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110111110001100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMGE_asimdmiscfp16_FZ";
    #[inline]
    pub const fn FCMGE_asimdmiscfp16_FZ(
        Q: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111011111000110u32 << 13u32
                | op.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCMLE_asimdmiscfp16_FZ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110111110001100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMLE_asimdmiscfp16_FZ";
    #[inline]
    pub const fn FCMLE_asimdmiscfp16_FZ(
        Q: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111011111000110u32 << 13u32
                | op.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FNEG_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110111110001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FNEG_asimdmiscfp16_R";
    #[inline]
    pub const fn FNEG_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111011111000111110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRINTI_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111011111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110011110011000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRINTI_asimdmiscfp16_R";
    #[inline]
    pub const fn FRINTI_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | o2.into_inner() << 23u32
                | 0b1111001100u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPU_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111011111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110011110011010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPU_asimdmiscfp16_R";
    #[inline]
    pub const fn FCVTPU_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | o2.into_inner() << 23u32
                | 0b1111001101u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111011111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110011110011010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_asimdmiscfp16_R";
    #[inline]
    pub const fn FCVTZU_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | o2.into_inner() << 23u32
                | 0b1111001101u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRSQRTE_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110111110011101100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRSQRTE_asimdmiscfp16_R";
    #[inline]
    pub const fn FRSQRTE_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111011111001110110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FSQRT_asimdmiscfp16_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110111110011111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FSQRT_asimdmiscfp16_R";
    #[inline]
    pub const fn FSQRT_asimdmiscfp16_R(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111011111001111110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
