/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod AND_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "AND_r_T1";
    #[inline]
    pub const fn AND_r_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rdn: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100000000u32 << 6u32 | Rm.into_inner() << 3u32 | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod EOR_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100000001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "EOR_r_T1";
    #[inline]
    pub const fn EOR_r_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rdn: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100000001u32 << 6u32 | Rm.into_inner() << 3u32 | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod MOV_rr_T1_ASR {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100000100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOV_rr_T1_ASR";
    #[inline]
    pub const fn MOV_rr_T1_ASR(
        Rs: ::aarchmrs_types::BitValue<3>,
        Rdm: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100000100u32 << 6u32 | Rs.into_inner() << 3u32 | Rdm.into_inner() << 0u32,
        )
    }
}
pub mod MOV_rr_T1_LSL {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOV_rr_T1_LSL";
    #[inline]
    pub const fn MOV_rr_T1_LSL(
        Rs: ::aarchmrs_types::BitValue<3>,
        Rdm: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100000010u32 << 6u32 | Rs.into_inner() << 3u32 | Rdm.into_inner() << 0u32,
        )
    }
}
pub mod MOV_rr_T1_LSR {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100000011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOV_rr_T1_LSR";
    #[inline]
    pub const fn MOV_rr_T1_LSR(
        Rs: ::aarchmrs_types::BitValue<3>,
        Rdm: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100000011u32 << 6u32 | Rs.into_inner() << 3u32 | Rdm.into_inner() << 0u32,
        )
    }
}
pub mod MOV_rr_T1_ROR {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100000111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MOV_rr_T1_ROR";
    #[inline]
    pub const fn MOV_rr_T1_ROR(
        Rs: ::aarchmrs_types::BitValue<3>,
        Rdm: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100000111u32 << 6u32 | Rs.into_inner() << 3u32 | Rdm.into_inner() << 0u32,
        )
    }
}
pub mod ADC_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100000101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ADC_r_T1";
    #[inline]
    pub const fn ADC_r_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rdn: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100000101u32 << 6u32 | Rm.into_inner() << 3u32 | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod SBC_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100000110000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SBC_r_T1";
    #[inline]
    pub const fn SBC_r_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rdn: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100000110u32 << 6u32 | Rm.into_inner() << 3u32 | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod TST_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TST_r_T1";
    #[inline]
    pub const fn TST_r_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100001000u32 << 6u32 | Rm.into_inner() << 3u32 | Rn.into_inner() << 0u32,
        )
    }
}
pub mod RSB_i_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100001001000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RSB_i_T1";
    #[inline]
    pub const fn RSB_i_T1(
        Rn: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100001001u32 << 6u32 | Rn.into_inner() << 3u32 | Rd.into_inner() << 0u32,
        )
    }
}
pub mod CMP_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100001010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMP_r_T1";
    #[inline]
    pub const fn CMP_r_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100001010u32 << 6u32 | Rm.into_inner() << 3u32 | Rn.into_inner() << 0u32,
        )
    }
}
pub mod CMN_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100001011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CMN_r_T1";
    #[inline]
    pub const fn CMN_r_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100001011u32 << 6u32 | Rm.into_inner() << 3u32 | Rn.into_inner() << 0u32,
        )
    }
}
pub mod ORR_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100001100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ORR_r_T1";
    #[inline]
    pub const fn ORR_r_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rdn: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100001100u32 << 6u32 | Rm.into_inner() << 3u32 | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod MUL_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100001101000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MUL_T1";
    #[inline]
    pub const fn MUL_T1(
        Rn: ::aarchmrs_types::BitValue<3>,
        Rdm: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100001101u32 << 6u32 | Rn.into_inner() << 3u32 | Rdm.into_inner() << 0u32,
        )
    }
}
pub mod BIC_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100001110000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BIC_r_T1";
    #[inline]
    pub const fn BIC_r_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rdn: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100001110u32 << 6u32 | Rm.into_inner() << 3u32 | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod MVN_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100001111000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MVN_r_T1";
    #[inline]
    pub const fn MVN_r_T1(
        Rm: ::aarchmrs_types::BitValue<3>,
        Rd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0100001111u32 << 6u32 | Rm.into_inner() << 3u32 | Rd.into_inner() << 0u32,
        )
    }
}
