/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod REV64_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "REV64_asimdmisc_R";
    #[inline]
    pub const fn REV64_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        o0: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100000000u32 << 13u32
                | o0.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod REV16_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "REV16_asimdmisc_R";
    #[inline]
    pub const fn REV16_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        o0: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100000000u32 << 13u32
                | o0.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SADDLP_asimdmisc_P {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111011110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000000010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SADDLP_asimdmisc_P";
    #[inline]
    pub const fn SADDLP_asimdmisc_P(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1000000u32 << 15u32
                | op.into_inner() << 14u32
                | 0b1010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SUQADD_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000000011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUQADD_asimdmisc_R";
    #[inline]
    pub const fn SUQADD_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100000001110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CLS_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000000100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CLS_asimdmisc_R";
    #[inline]
    pub const fn CLS_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100000010010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CNT_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000000101100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CNT_asimdmisc_R";
    #[inline]
    pub const fn CNT_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100000010110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SADALP_asimdmisc_P {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111011110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000000010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SADALP_asimdmisc_P";
    #[inline]
    pub const fn SADALP_asimdmisc_P(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1000000u32 << 15u32
                | op.into_inner() << 14u32
                | 0b1010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQABS_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000000111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQABS_asimdmisc_R";
    #[inline]
    pub const fn SQABS_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100000011110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CMGT_asimdmisc_Z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000001000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMGT_asimdmisc_Z";
    #[inline]
    pub const fn CMGT_asimdmisc_Z(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100000100u32 << 13u32
                | op.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CMEQ_asimdmisc_Z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000001000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMEQ_asimdmisc_Z";
    #[inline]
    pub const fn CMEQ_asimdmisc_Z(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100000100u32 << 13u32
                | op.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CMLT_asimdmisc_Z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000001010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMLT_asimdmisc_Z";
    #[inline]
    pub const fn CMLT_asimdmisc_Z(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100000101010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod ABS_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000001011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ABS_asimdmisc_R";
    #[inline]
    pub const fn ABS_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100000101110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod XTN_asimdmisc_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000010010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "XTN_asimdmisc_N";
    #[inline]
    pub const fn XTN_asimdmisc_N(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100001001010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQXTN_asimdmisc_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000010100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQXTN_asimdmisc_N";
    #[inline]
    pub const fn SQXTN_asimdmisc_N(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100001010010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTN_asimdmisc_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000010110100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTN_asimdmisc_N";
    #[inline]
    pub const fn FCVTN_asimdmisc_N(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011100u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100001011010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTL_asimdmisc_L {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000010111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTL_asimdmisc_L";
    #[inline]
    pub const fn FCVTL_asimdmisc_L(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011100u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100001011110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRINTN_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000011000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRINTN_asimdmisc_R";
    #[inline]
    pub const fn FRINTN_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | o2.into_inner() << 23u32
                | sz.into_inner() << 22u32
                | 0b100001100u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRINTM_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000011000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRINTM_asimdmisc_R";
    #[inline]
    pub const fn FRINTM_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | o2.into_inner() << 23u32
                | sz.into_inner() << 22u32
                | 0b100001100u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNS_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000011010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNS_asimdmisc_R";
    #[inline]
    pub const fn FCVTNS_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | o2.into_inner() << 23u32
                | sz.into_inner() << 22u32
                | 0b100001101u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMS_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000011010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMS_asimdmisc_R";
    #[inline]
    pub const fn FCVTMS_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | o2.into_inner() << 23u32
                | sz.into_inner() << 22u32
                | 0b100001101u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAS_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000011100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAS_asimdmisc_R";
    #[inline]
    pub const fn FCVTAS_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011100u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100001110010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SCVTF_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000011101100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_asimdmisc_R";
    #[inline]
    pub const fn SCVTF_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011100u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100001110110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRINT32Z_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000011110100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRINT32Z_asimdmisc_R";
    #[inline]
    pub const fn FRINT32Z_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011100u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100001111u32 << 13u32
                | op.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRINT64Z_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000011110100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRINT64Z_asimdmisc_R";
    #[inline]
    pub const fn FRINT64Z_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011100u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100001111u32 << 13u32
                | op.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCMGT_asimdmisc_FZ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101000001100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMGT_asimdmisc_FZ";
    #[inline]
    pub const fn FCMGT_asimdmisc_FZ(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011101u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100000110u32 << 13u32
                | op.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCMEQ_asimdmisc_FZ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101000001100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMEQ_asimdmisc_FZ";
    #[inline]
    pub const fn FCMEQ_asimdmisc_FZ(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011101u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100000110u32 << 13u32
                | op.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCMLT_asimdmisc_FZ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101000001110100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMLT_asimdmisc_FZ";
    #[inline]
    pub const fn FCMLT_asimdmisc_FZ(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011101u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100000111010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FABS_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FABS_asimdmisc_R";
    #[inline]
    pub const fn FABS_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011101u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100000111110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRINTP_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000011000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRINTP_asimdmisc_R";
    #[inline]
    pub const fn FRINTP_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | o2.into_inner() << 23u32
                | sz.into_inner() << 22u32
                | 0b100001100u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRINTZ_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000011000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRINTZ_asimdmisc_R";
    #[inline]
    pub const fn FRINTZ_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | o2.into_inner() << 23u32
                | sz.into_inner() << 22u32
                | 0b100001100u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPS_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000011010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPS_asimdmisc_R";
    #[inline]
    pub const fn FCVTPS_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | o2.into_inner() << 23u32
                | sz.into_inner() << 22u32
                | 0b100001101u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110001000011010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_asimdmisc_R";
    #[inline]
    pub const fn FCVTZS_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001110u32 << 24u32
                | o2.into_inner() << 23u32
                | sz.into_inner() << 22u32
                | 0b100001101u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod URECPE_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101000011100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "URECPE_asimdmisc_R";
    #[inline]
    pub const fn URECPE_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011101u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100001110010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRECPE_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101000011101100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRECPE_asimdmisc_R";
    #[inline]
    pub const fn FRECPE_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011101u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100001110110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod BFCVTN_asimdmisc_4S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001110101000010110100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BFCVTN_asimdmisc_4S";
    #[inline]
    pub const fn BFCVTN_asimdmisc_4S(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b00111010100001011010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod REV32_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "REV32_asimdmisc_R";
    #[inline]
    pub const fn REV32_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        o0: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100000000u32 << 13u32
                | o0.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UADDLP_asimdmisc_P {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111011110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000000010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UADDLP_asimdmisc_P";
    #[inline]
    pub const fn UADDLP_asimdmisc_P(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1000000u32 << 15u32
                | op.into_inner() << 14u32
                | 0b1010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod USQADD_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000000011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "USQADD_asimdmisc_R";
    #[inline]
    pub const fn USQADD_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100000001110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CLZ_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000000100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CLZ_asimdmisc_R";
    #[inline]
    pub const fn CLZ_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100000010010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UADALP_asimdmisc_P {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111011110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000000010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UADALP_asimdmisc_P";
    #[inline]
    pub const fn UADALP_asimdmisc_P(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1000000u32 << 15u32
                | op.into_inner() << 14u32
                | 0b1010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQNEG_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000000111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQNEG_asimdmisc_R";
    #[inline]
    pub const fn SQNEG_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100000011110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CMGE_asimdmisc_Z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000001000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMGE_asimdmisc_Z";
    #[inline]
    pub const fn CMGE_asimdmisc_Z(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100000100u32 << 13u32
                | op.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CMLE_asimdmisc_Z {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000001000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMLE_asimdmisc_Z";
    #[inline]
    pub const fn CMLE_asimdmisc_Z(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100000100u32 << 13u32
                | op.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod NEG_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000001011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "NEG_asimdmisc_R";
    #[inline]
    pub const fn NEG_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100000101110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQXTUN_asimdmisc_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000010010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQXTUN_asimdmisc_N";
    #[inline]
    pub const fn SQXTUN_asimdmisc_N(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100001001010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SHLL_asimdmisc_S {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000010011100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHLL_asimdmisc_S";
    #[inline]
    pub const fn SHLL_asimdmisc_S(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100001001110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UQXTN_asimdmisc_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000010100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UQXTN_asimdmisc_N";
    #[inline]
    pub const fn UQXTN_asimdmisc_N(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | size.into_inner() << 22u32
                | 0b100001010010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTXN_asimdmisc_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110011000010110100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTXN_asimdmisc_N";
    #[inline]
    pub const fn FCVTXN_asimdmisc_N(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111001100001011010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRINTA_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000011000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRINTA_asimdmisc_R";
    #[inline]
    pub const fn FRINTA_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | o2.into_inner() << 23u32
                | sz.into_inner() << 22u32
                | 0b100001100u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRINTX_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000011000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRINTX_asimdmisc_R";
    #[inline]
    pub const fn FRINTX_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | o2.into_inner() << 23u32
                | sz.into_inner() << 22u32
                | 0b100001100u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTNU_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000011010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTNU_asimdmisc_R";
    #[inline]
    pub const fn FCVTNU_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | o2.into_inner() << 23u32
                | sz.into_inner() << 22u32
                | 0b100001101u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTMU_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000011010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTMU_asimdmisc_R";
    #[inline]
    pub const fn FCVTMU_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | o2.into_inner() << 23u32
                | sz.into_inner() << 22u32
                | 0b100001101u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTAU_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000011100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTAU_asimdmisc_R";
    #[inline]
    pub const fn FCVTAU_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011100u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100001110010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000011101100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_asimdmisc_R";
    #[inline]
    pub const fn UCVTF_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011100u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100001110110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRINT32X_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000011110100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRINT32X_asimdmisc_R";
    #[inline]
    pub const fn FRINT32X_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011100u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100001111u32 << 13u32
                | op.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRINT64X_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000011110100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRINT64X_asimdmisc_R";
    #[inline]
    pub const fn FRINT64X_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011100u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100001111u32 << 13u32
                | op.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod NOT_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000000101100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "NOT_asimdmisc_R";
    #[inline]
    pub const fn NOT_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111000100000010110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod F1CVTL_asimdmisc_V {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000010111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "F1CVTL_asimdmisc_V";
    #[inline]
    pub const fn F1CVTL_asimdmisc_V(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111000100001011110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod RBIT_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110011000000101100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RBIT_asimdmisc_R";
    #[inline]
    pub const fn RBIT_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111001100000010110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod F2CVTL_asimdmisc_V {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110011000010111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "F2CVTL_asimdmisc_V";
    #[inline]
    pub const fn F2CVTL_asimdmisc_V(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111001100001011110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCMGE_asimdmisc_FZ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110101000001100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMGE_asimdmisc_FZ";
    #[inline]
    pub const fn FCMGE_asimdmisc_FZ(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011101u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100000110u32 << 13u32
                | op.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCMLE_asimdmisc_FZ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110101000001100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCMLE_asimdmisc_FZ";
    #[inline]
    pub const fn FCMLE_asimdmisc_FZ(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011101u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100000110u32 << 13u32
                | op.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FNEG_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110101000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FNEG_asimdmisc_R";
    #[inline]
    pub const fn FNEG_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011101u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100000111110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRINTI_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000011000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRINTI_asimdmisc_R";
    #[inline]
    pub const fn FRINTI_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | o2.into_inner() << 23u32
                | sz.into_inner() << 22u32
                | 0b100001100u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTPU_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000011010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTPU_asimdmisc_R";
    #[inline]
    pub const fn FCVTPU_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | o2.into_inner() << 23u32
                | sz.into_inner() << 22u32
                | 0b100001101u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111001111111110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110001000011010100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_asimdmisc_R";
    #[inline]
    pub const fn FCVTZU_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        o2: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        o1: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b101110u32 << 24u32
                | o2.into_inner() << 23u32
                | sz.into_inner() << 22u32
                | 0b100001101u32 << 13u32
                | o1.into_inner() << 12u32
                | 0b10u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod URSQRTE_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110101000011100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "URSQRTE_asimdmisc_R";
    #[inline]
    pub const fn URSQRTE_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011101u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100001110010u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FRSQRTE_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110101000011101100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FRSQRTE_asimdmisc_R";
    #[inline]
    pub const fn FRSQRTE_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011101u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100001110110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FSQRT_asimdmisc_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111101111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110101000011111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FSQRT_asimdmisc_R";
    #[inline]
    pub const fn FSQRT_asimdmisc_R(
        Q: ::aarchmrs_types::BitValue<1>,
        sz: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011101u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b100001111110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod BF1CVTL_asimdmisc_V {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110101000010111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BF1CVTL_asimdmisc_V";
    #[inline]
    pub const fn BF1CVTL_asimdmisc_V(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111010100001011110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod BF2CVTL_asimdmisc_V {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101110111000010111100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BF2CVTL_asimdmisc_V";
    #[inline]
    pub const fn BF2CVTL_asimdmisc_V(
        Q: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b10111011100001011110u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
