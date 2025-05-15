/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ST1_asisdlsop_BX1_r1b {
    #[inline]
    pub fn ST1_asisdlsop_BX1_r1b(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST3_asisdlsop_BX3_r3b {
    #[inline]
    pub fn ST3_asisdlsop_BX3_r3b(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST1_asisdlsop_HX1_r1h {
    #[inline]
    pub fn ST1_asisdlsop_HX1_r1h(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST3_asisdlsop_HX3_r3h {
    #[inline]
    pub fn ST3_asisdlsop_HX3_r3h(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST1_asisdlsop_SX1_r1s {
    #[inline]
    pub fn ST1_asisdlsop_SX1_r1s(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b100u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST1_asisdlsop_DX1_r1d {
    #[inline]
    pub fn ST1_asisdlsop_DX1_r1d(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST3_asisdlsop_SX3_r3s {
    #[inline]
    pub fn ST3_asisdlsop_SX3_r3s(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST3_asisdlsop_DX3_r3d {
    #[inline]
    pub fn ST3_asisdlsop_DX3_r3d(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b101001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST1_asisdlsop_B1_i1b {
    #[inline]
    pub fn ST1_asisdlsop_B1_i1b(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110110011111000u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST3_asisdlsop_B3_i3b {
    #[inline]
    pub fn ST3_asisdlsop_B3_i3b(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110110011111001u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST1_asisdlsop_H1_i1h {
    #[inline]
    pub fn ST1_asisdlsop_H1_i1h(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110110011111010u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST3_asisdlsop_H3_i3h {
    #[inline]
    pub fn ST3_asisdlsop_H3_i3h(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110110011111011u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST1_asisdlsop_S1_i1s {
    #[inline]
    pub fn ST1_asisdlsop_S1_i1s(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110110011111100u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST1_asisdlsop_D1_i1d {
    #[inline]
    pub fn ST1_asisdlsop_D1_i1d(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110110011111100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST3_asisdlsop_S3_i3s {
    #[inline]
    pub fn ST3_asisdlsop_S3_i3s(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110110011111101u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST3_asisdlsop_D3_i3d {
    #[inline]
    pub fn ST3_asisdlsop_D3_i3d(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110110011111101001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST2_asisdlsop_BX2_r2b {
    #[inline]
    pub fn ST2_asisdlsop_BX2_r2b(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101101u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST4_asisdlsop_BX4_r4b {
    #[inline]
    pub fn ST4_asisdlsop_BX4_r4b(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101101u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST2_asisdlsop_HX2_r2h {
    #[inline]
    pub fn ST2_asisdlsop_HX2_r2h(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101101u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST4_asisdlsop_HX4_r4h {
    #[inline]
    pub fn ST4_asisdlsop_HX4_r4h(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101101u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST2_asisdlsop_SX2_r2s {
    #[inline]
    pub fn ST2_asisdlsop_SX2_r2s(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101101u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b100u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST2_asisdlsop_DX2_r2d {
    #[inline]
    pub fn ST2_asisdlsop_DX2_r2d(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101101u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST4_asisdlsop_SX4_r4s {
    #[inline]
    pub fn ST4_asisdlsop_SX4_r4s(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101101u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST4_asisdlsop_DX4_r4d {
    #[inline]
    pub fn ST4_asisdlsop_DX4_r4d(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101101u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b101001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST2_asisdlsop_B2_i2b {
    #[inline]
    pub fn ST2_asisdlsop_B2_i2b(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110110111111000u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST4_asisdlsop_B4_i4b {
    #[inline]
    pub fn ST4_asisdlsop_B4_i4b(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110110111111001u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST2_asisdlsop_H2_i2h {
    #[inline]
    pub fn ST2_asisdlsop_H2_i2h(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110110111111010u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST4_asisdlsop_H4_i4h {
    #[inline]
    pub fn ST4_asisdlsop_H4_i4h(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110110111111011u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST2_asisdlsop_S2_i2s {
    #[inline]
    pub fn ST2_asisdlsop_S2_i2s(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110110111111100u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST2_asisdlsop_D2_i2d {
    #[inline]
    pub fn ST2_asisdlsop_D2_i2d(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110110111111100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST4_asisdlsop_S4_i4s {
    #[inline]
    pub fn ST4_asisdlsop_S4_i4s(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110110111111101u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod ST4_asisdlsop_D4_i4d {
    #[inline]
    pub fn ST4_asisdlsop_D4_i4d(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110110111111101001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD1_asisdlsop_BX1_r1b {
    #[inline]
    pub fn LD1_asisdlsop_BX1_r1b(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD3_asisdlsop_BX3_r3b {
    #[inline]
    pub fn LD3_asisdlsop_BX3_r3b(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD1_asisdlsop_HX1_r1h {
    #[inline]
    pub fn LD1_asisdlsop_HX1_r1h(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD3_asisdlsop_HX3_r3h {
    #[inline]
    pub fn LD3_asisdlsop_HX3_r3h(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD1_asisdlsop_SX1_r1s {
    #[inline]
    pub fn LD1_asisdlsop_SX1_r1s(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b100u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD1_asisdlsop_DX1_r1d {
    #[inline]
    pub fn LD1_asisdlsop_DX1_r1d(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD3_asisdlsop_SX3_r3s {
    #[inline]
    pub fn LD3_asisdlsop_SX3_r3s(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD3_asisdlsop_DX3_r3d {
    #[inline]
    pub fn LD3_asisdlsop_DX3_r3d(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b101001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD1R_asisdlsop_RX1_r {
    #[inline]
    pub fn LD1R_asisdlsop_RX1_r(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b1100u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD3R_asisdlsop_RX3_r {
    #[inline]
    pub fn LD3R_asisdlsop_RX3_r(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b1110u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD1_asisdlsop_B1_i1b {
    #[inline]
    pub fn LD1_asisdlsop_B1_i1b(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110111011111000u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD3_asisdlsop_B3_i3b {
    #[inline]
    pub fn LD3_asisdlsop_B3_i3b(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110111011111001u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD1_asisdlsop_H1_i1h {
    #[inline]
    pub fn LD1_asisdlsop_H1_i1h(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110111011111010u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD3_asisdlsop_H3_i3h {
    #[inline]
    pub fn LD3_asisdlsop_H3_i3h(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110111011111011u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD1_asisdlsop_S1_i1s {
    #[inline]
    pub fn LD1_asisdlsop_S1_i1s(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110111011111100u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD1_asisdlsop_D1_i1d {
    #[inline]
    pub fn LD1_asisdlsop_D1_i1d(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110111011111100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD3_asisdlsop_S3_i3s {
    #[inline]
    pub fn LD3_asisdlsop_S3_i3s(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110111011111101u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD3_asisdlsop_D3_i3d {
    #[inline]
    pub fn LD3_asisdlsop_D3_i3d(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110111011111101001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD1R_asisdlsop_R1_i {
    #[inline]
    pub fn LD1R_asisdlsop_R1_i(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101110111111100u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD3R_asisdlsop_R3_i {
    #[inline]
    pub fn LD3R_asisdlsop_R3_i(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101110111111110u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD2_asisdlsop_BX2_r2b {
    #[inline]
    pub fn LD2_asisdlsop_BX2_r2b(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD4_asisdlsop_BX4_r4b {
    #[inline]
    pub fn LD4_asisdlsop_BX4_r4b(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD2_asisdlsop_HX2_r2h {
    #[inline]
    pub fn LD2_asisdlsop_HX2_r2h(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD4_asisdlsop_HX4_r4h {
    #[inline]
    pub fn LD4_asisdlsop_HX4_r4h(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD2_asisdlsop_SX2_r2s {
    #[inline]
    pub fn LD2_asisdlsop_SX2_r2s(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b100u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD2_asisdlsop_DX2_r2d {
    #[inline]
    pub fn LD2_asisdlsop_DX2_r2d(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD4_asisdlsop_SX4_r4s {
    #[inline]
    pub fn LD4_asisdlsop_SX4_r4s(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD4_asisdlsop_DX4_r4d {
    #[inline]
    pub fn LD4_asisdlsop_DX4_r4d(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b101001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD2R_asisdlsop_RX2_r {
    #[inline]
    pub fn LD2R_asisdlsop_RX2_r(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b1100u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD4R_asisdlsop_RX4_r {
    #[inline]
    pub fn LD4R_asisdlsop_RX4_r(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b1110u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD2_asisdlsop_B2_i2b {
    #[inline]
    pub fn LD2_asisdlsop_B2_i2b(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110111111111000u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD4_asisdlsop_B4_i4b {
    #[inline]
    pub fn LD4_asisdlsop_B4_i4b(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110111111111001u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD2_asisdlsop_H2_i2h {
    #[inline]
    pub fn LD2_asisdlsop_H2_i2h(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110111111111010u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD4_asisdlsop_H4_i4h {
    #[inline]
    pub fn LD4_asisdlsop_H4_i4h(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110111111111011u32 << 13u32
                | u32::from(S.into()) << 12u32
                | u32::from(size.into()) << 11u32
                | 0b0u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD2_asisdlsop_S2_i2s {
    #[inline]
    pub fn LD2_asisdlsop_S2_i2s(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110111111111100u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD2_asisdlsop_D2_i2d {
    #[inline]
    pub fn LD2_asisdlsop_D2_i2d(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110111111111100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD4_asisdlsop_S4_i4s {
    #[inline]
    pub fn LD4_asisdlsop_S4_i4s(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110111111111101u32 << 13u32
                | u32::from(S.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD4_asisdlsop_D4_i4d {
    #[inline]
    pub fn LD4_asisdlsop_D4_i4d(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b00110111111111101001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD2R_asisdlsop_R2_i {
    #[inline]
    pub fn LD2R_asisdlsop_R2_i(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101111111111100u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LD4R_asisdlsop_R4_i {
    #[inline]
    pub fn LD4R_asisdlsop_R4_i(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001101111111111110u32 << 12u32
                | u32::from(size.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
