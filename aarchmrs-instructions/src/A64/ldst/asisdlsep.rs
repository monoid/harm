/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ST4_asisdlsep_R4_r {
    #[inline]
    pub fn ST4_asisdlsep_R4_r(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0000u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST1_asisdlsep_R4_r4 {
    #[inline]
    pub fn ST1_asisdlsep_R4_r4(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0010u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST3_asisdlsep_R3_r {
    #[inline]
    pub fn ST3_asisdlsep_R3_r(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0100u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST1_asisdlsep_R3_r3 {
    #[inline]
    pub fn ST1_asisdlsep_R3_r3(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0110u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST1_asisdlsep_R1_r1 {
    #[inline]
    pub fn ST1_asisdlsep_R1_r1(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0111u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST2_asisdlsep_R2_r {
    #[inline]
    pub fn ST2_asisdlsep_R2_r(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b1000u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST1_asisdlsep_R2_r2 {
    #[inline]
    pub fn ST1_asisdlsep_R2_r2(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b1010u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST4_asisdlsep_I4_i {
    #[inline]
    pub fn ST4_asisdlsep_I4_i(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100100111110000u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST1_asisdlsep_I4_i4 {
    #[inline]
    pub fn ST1_asisdlsep_I4_i4(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100100111110010u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST3_asisdlsep_I3_i {
    #[inline]
    pub fn ST3_asisdlsep_I3_i(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100100111110100u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST1_asisdlsep_I3_i3 {
    #[inline]
    pub fn ST1_asisdlsep_I3_i3(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100100111110110u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST1_asisdlsep_I1_i1 {
    #[inline]
    pub fn ST1_asisdlsep_I1_i1(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100100111110111u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST2_asisdlsep_I2_i {
    #[inline]
    pub fn ST2_asisdlsep_I2_i(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100100111111000u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST1_asisdlsep_I2_i2 {
    #[inline]
    pub fn ST1_asisdlsep_I2_i2(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100100111111010u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD4_asisdlsep_R4_r {
    #[inline]
    pub fn LD4_asisdlsep_R4_r(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0000u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD1_asisdlsep_R4_r4 {
    #[inline]
    pub fn LD1_asisdlsep_R4_r4(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0010u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD3_asisdlsep_R3_r {
    #[inline]
    pub fn LD3_asisdlsep_R3_r(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0100u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD1_asisdlsep_R3_r3 {
    #[inline]
    pub fn LD1_asisdlsep_R3_r3(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0110u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD1_asisdlsep_R1_r1 {
    #[inline]
    pub fn LD1_asisdlsep_R1_r1(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0111u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD2_asisdlsep_R2_r {
    #[inline]
    pub fn LD2_asisdlsep_R2_r(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b1000u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD1_asisdlsep_R2_r2 {
    #[inline]
    pub fn LD1_asisdlsep_R2_r2(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b1010u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD4_asisdlsep_I4_i {
    #[inline]
    pub fn LD4_asisdlsep_I4_i(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100110111110000u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD1_asisdlsep_I4_i4 {
    #[inline]
    pub fn LD1_asisdlsep_I4_i4(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100110111110010u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD3_asisdlsep_I3_i {
    #[inline]
    pub fn LD3_asisdlsep_I3_i(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100110111110100u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD1_asisdlsep_I3_i3 {
    #[inline]
    pub fn LD1_asisdlsep_I3_i3(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100110111110110u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD1_asisdlsep_I1_i1 {
    #[inline]
    pub fn LD1_asisdlsep_I1_i1(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100110111110111u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD2_asisdlsep_I2_i {
    #[inline]
    pub fn LD2_asisdlsep_I2_i(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100110111111000u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD1_asisdlsep_I2_i2 {
    #[inline]
    pub fn LD1_asisdlsep_I2_i2(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001100110111111010u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
