/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SMLABB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001000000000000000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLABB_A1";
    #[inline]
    pub const fn SMLABB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Ra: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010000u32 << 20u32
                | Rd.into_inner() << 16u32
                | Ra.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1000u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SMLABT_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001000000000000000011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLABT_A1";
    #[inline]
    pub const fn SMLABT_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Ra: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010000u32 << 20u32
                | Rd.into_inner() << 16u32
                | Ra.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1100u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SMLATB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001000000000000000010100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLATB_A1";
    #[inline]
    pub const fn SMLATB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Ra: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010000u32 << 20u32
                | Rd.into_inner() << 16u32
                | Ra.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1010u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SMLATT_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001000000000000000011100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLATT_A1";
    #[inline]
    pub const fn SMLATT_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Ra: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010000u32 << 20u32
                | Rd.into_inner() << 16u32
                | Ra.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1110u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SMLAWB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001001000000000000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLAWB_A1";
    #[inline]
    pub const fn SMLAWB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Ra: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010010u32 << 20u32
                | Rd.into_inner() << 16u32
                | Ra.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1000u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SMLAWT_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001001000000000000011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLAWT_A1";
    #[inline]
    pub const fn SMLAWT_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Ra: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010010u32 << 20u32
                | Rd.into_inner() << 16u32
                | Ra.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1100u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SMULWB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001001000000000000010100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMULWB_A1";
    #[inline]
    pub const fn SMULWB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010010u32 << 20u32
                | Rd.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rm.into_inner() << 8u32
                | 0b1010u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SMULWT_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001001000000000000011100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMULWT_A1";
    #[inline]
    pub const fn SMULWT_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010010u32 << 20u32
                | Rd.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rm.into_inner() << 8u32
                | 0b1110u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SMLALBB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001010000000000000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLALBB_A1";
    #[inline]
    pub const fn SMLALBB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010100u32 << 20u32
                | RdHi.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1000u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SMLALBT_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001010000000000000011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLALBT_A1";
    #[inline]
    pub const fn SMLALBT_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010100u32 << 20u32
                | RdHi.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1100u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SMLALTB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001010000000000000010100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLALTB_A1";
    #[inline]
    pub const fn SMLALTB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010100u32 << 20u32
                | RdHi.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1010u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SMLALTT_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001010000000000000011100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMLALTT_A1";
    #[inline]
    pub const fn SMLALTT_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        RdHi: ::aarchmrs_types::BitValue<4>,
        RdLo: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010100u32 << 20u32
                | RdHi.into_inner() << 16u32
                | RdLo.into_inner() << 12u32
                | Rm.into_inner() << 8u32
                | 0b1110u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SMULBB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001011000000000000010000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMULBB_A1";
    #[inline]
    pub const fn SMULBB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010110u32 << 20u32
                | Rd.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rm.into_inner() << 8u32
                | 0b1000u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SMULBT_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001011000000000000011000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMULBT_A1";
    #[inline]
    pub const fn SMULBT_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010110u32 << 20u32
                | Rd.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rm.into_inner() << 8u32
                | 0b1100u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SMULTB_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001011000000000000010100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMULTB_A1";
    #[inline]
    pub const fn SMULTB_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010110u32 << 20u32
                | Rd.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rm.into_inner() << 8u32
                | 0b1010u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
pub mod SMULTT_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001011000000000000011100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMULTT_A1";
    #[inline]
    pub const fn SMULTT_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
        Rn: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b00010110u32 << 20u32
                | Rd.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | Rm.into_inner() << 8u32
                | 0b1110u32 << 4u32
                | Rn.into_inner() << 0u32,
        )
    }
}
