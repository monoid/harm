/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ST4_asisdlsep_R4_r {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST4_asisdlsep_R4_r";
    #[inline]
    pub const fn ST4_asisdlsep_R4_r(
        Q: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100100u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST1_asisdlsep_R4_r4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST1_asisdlsep_R4_r4";
    #[inline]
    pub const fn ST1_asisdlsep_R4_r4(
        Q: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100100u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0010u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST3_asisdlsep_R3_r {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST3_asisdlsep_R3_r";
    #[inline]
    pub const fn ST3_asisdlsep_R3_r(
        Q: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100100u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0100u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST1_asisdlsep_R3_r3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100000000110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST1_asisdlsep_R3_r3";
    #[inline]
    pub const fn ST1_asisdlsep_R3_r3(
        Q: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100100u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0110u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST1_asisdlsep_R1_r1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100000000111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST1_asisdlsep_R1_r1";
    #[inline]
    pub const fn ST1_asisdlsep_R1_r1(
        Q: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100100u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0111u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST2_asisdlsep_R2_r {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST2_asisdlsep_R2_r";
    #[inline]
    pub const fn ST2_asisdlsep_R2_r(
        Q: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100100u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b1000u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST1_asisdlsep_R2_r2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST1_asisdlsep_R2_r2";
    #[inline]
    pub const fn ST1_asisdlsep_R2_r2(
        Q: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100100u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b1010u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST4_asisdlsep_I4_i {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST4_asisdlsep_I4_i";
    #[inline]
    pub const fn ST4_asisdlsep_I4_i(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100100111110000u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST1_asisdlsep_I4_i4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100111110010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST1_asisdlsep_I4_i4";
    #[inline]
    pub const fn ST1_asisdlsep_I4_i4(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100100111110010u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST3_asisdlsep_I3_i {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100111110100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST3_asisdlsep_I3_i";
    #[inline]
    pub const fn ST3_asisdlsep_I3_i(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100100111110100u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST1_asisdlsep_I3_i3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100111110110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST1_asisdlsep_I3_i3";
    #[inline]
    pub const fn ST1_asisdlsep_I3_i3(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100100111110110u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST1_asisdlsep_I1_i1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100111110111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST1_asisdlsep_I1_i1";
    #[inline]
    pub const fn ST1_asisdlsep_I1_i1(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100100111110111u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST2_asisdlsep_I2_i {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST2_asisdlsep_I2_i";
    #[inline]
    pub const fn ST2_asisdlsep_I2_i(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100100111111000u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod ST1_asisdlsep_I2_i2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100100111111010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ST1_asisdlsep_I2_i2";
    #[inline]
    pub const fn ST1_asisdlsep_I2_i2(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100100111111010u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD4_asisdlsep_R4_r {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD4_asisdlsep_R4_r";
    #[inline]
    pub const fn LD4_asisdlsep_R4_r(
        Q: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100110u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0000u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD1_asisdlsep_R4_r4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100110000000010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD1_asisdlsep_R4_r4";
    #[inline]
    pub const fn LD1_asisdlsep_R4_r4(
        Q: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100110u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0010u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD3_asisdlsep_R3_r {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100110000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD3_asisdlsep_R3_r";
    #[inline]
    pub const fn LD3_asisdlsep_R3_r(
        Q: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100110u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0100u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD1_asisdlsep_R3_r3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100110000000110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD1_asisdlsep_R3_r3";
    #[inline]
    pub const fn LD1_asisdlsep_R3_r3(
        Q: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100110u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0110u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD1_asisdlsep_R1_r1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100110000000111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD1_asisdlsep_R1_r1";
    #[inline]
    pub const fn LD1_asisdlsep_R1_r1(
        Q: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100110u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b0111u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD2_asisdlsep_R2_r {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100110000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD2_asisdlsep_R2_r";
    #[inline]
    pub const fn LD2_asisdlsep_R2_r(
        Q: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100110u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b1000u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD1_asisdlsep_R2_r2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100110000001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD1_asisdlsep_R2_r2";
    #[inline]
    pub const fn LD1_asisdlsep_R2_r2(
        Q: ::aarchmrs_types::BitValue<1>,
        Rm: ::aarchmrs_types::BitValue<5>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100110u32 << 21u32
                | Rm.into_inner() << 16u32
                | 0b1010u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD4_asisdlsep_I4_i {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100110111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD4_asisdlsep_I4_i";
    #[inline]
    pub const fn LD4_asisdlsep_I4_i(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100110111110000u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD1_asisdlsep_I4_i4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100110111110010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD1_asisdlsep_I4_i4";
    #[inline]
    pub const fn LD1_asisdlsep_I4_i4(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100110111110010u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD3_asisdlsep_I3_i {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100110111110100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD3_asisdlsep_I3_i";
    #[inline]
    pub const fn LD3_asisdlsep_I3_i(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100110111110100u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD1_asisdlsep_I3_i3 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100110111110110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD1_asisdlsep_I3_i3";
    #[inline]
    pub const fn LD1_asisdlsep_I3_i3(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100110111110110u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD1_asisdlsep_I1_i1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100110111110111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD1_asisdlsep_I1_i1";
    #[inline]
    pub const fn LD1_asisdlsep_I1_i1(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100110111110111u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD2_asisdlsep_I2_i {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100110111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD2_asisdlsep_I2_i";
    #[inline]
    pub const fn LD2_asisdlsep_I2_i(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100110111111000u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LD1_asisdlsep_I2_i2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111111111111111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001100110111111010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LD1_asisdlsep_I2_i2";
    #[inline]
    pub const fn LD1_asisdlsep_I2_i2(
        Q: ::aarchmrs_types::BitValue<1>,
        size: ::aarchmrs_types::BitValue<2>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b001100110111111010u32 << 12u32
                | size.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
