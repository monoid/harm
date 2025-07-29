/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ST4_asisdlse_R4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST4_asisdlse_R4";
    #[inline]
    pub const fn ST4_asisdlse_R4(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100000000000000u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST1_asisdlse_R4_4v {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100000000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST1_asisdlse_R4_4v";
    #[inline]
    pub const fn ST1_asisdlse_R4_4v(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100000000000010u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST3_asisdlse_R3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100000000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST3_asisdlse_R3";
    #[inline]
    pub const fn ST3_asisdlse_R3(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100000000000100u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST1_asisdlse_R3_3v {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100000000000110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST1_asisdlse_R3_3v";
    #[inline]
    pub const fn ST1_asisdlse_R3_3v(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100000000000110u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST1_asisdlse_R1_1v {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100000000000111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST1_asisdlse_R1_1v";
    #[inline]
    pub const fn ST1_asisdlse_R1_1v(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100000000000111u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST2_asisdlse_R2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100000000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST2_asisdlse_R2";
    #[inline]
    pub const fn ST2_asisdlse_R2(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100000000001000u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST1_asisdlse_R2_2v {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100000000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST1_asisdlse_R2_2v";
    #[inline]
    pub const fn ST1_asisdlse_R2_2v(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100000000001010u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD4_asisdlse_R4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD4_asisdlse_R4";
    #[inline]
    pub const fn LD4_asisdlse_R4(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100010000000000u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD1_asisdlse_R4_4v {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100010000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD1_asisdlse_R4_4v";
    #[inline]
    pub const fn LD1_asisdlse_R4_4v(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100010000000010u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD3_asisdlse_R3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100010000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD3_asisdlse_R3";
    #[inline]
    pub const fn LD3_asisdlse_R3(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100010000000100u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD1_asisdlse_R3_3v {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100010000000110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD1_asisdlse_R3_3v";
    #[inline]
    pub const fn LD1_asisdlse_R3_3v(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100010000000110u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD1_asisdlse_R1_1v {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100010000000111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD1_asisdlse_R1_1v";
    #[inline]
    pub const fn LD1_asisdlse_R1_1v(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100010000000111u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD2_asisdlse_R2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100010000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD2_asisdlse_R2";
    #[inline]
    pub const fn LD2_asisdlse_R2(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100010000001000u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD1_asisdlse_R2_2v {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100010000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD1_asisdlse_R2_2v";
    #[inline]
    pub const fn LD1_asisdlse_R2_2v(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100010000001010u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
