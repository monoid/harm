/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod RBIT_32_dp_1src {
    #[inline]
    pub fn RBIT_32_dp_1src(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101101011000000000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod REV16_32_dp_1src {
    #[inline]
    pub fn REV16_32_dp_1src(
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011010110000000000u32 << 12u32
                | u32::from(opc.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod REV_32_dp_1src {
    #[inline]
    pub fn REV_32_dp_1src(
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011010110000000000u32 << 12u32
                | u32::from(opc.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CLZ_32_dp_1src {
    #[inline]
    pub fn CLZ_32_dp_1src(
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010110101100000000010u32 << 11u32
                | u32::from(op.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CLS_32_dp_1src {
    #[inline]
    pub fn CLS_32_dp_1src(
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010110101100000000010u32 << 11u32
                | u32::from(op.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CTZ_32_dp_1src {
    #[inline]
    pub fn CTZ_32_dp_1src(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101101011000000000110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CNT_32_dp_1src {
    #[inline]
    pub fn CNT_32_dp_1src(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101101011000000000111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ABS_32_dp_1src {
    #[inline]
    pub fn ABS_32_dp_1src(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101101011000000001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod RBIT_64_dp_1src {
    #[inline]
    pub fn RBIT_64_dp_1src(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101101011000000000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod REV16_64_dp_1src {
    #[inline]
    pub fn REV16_64_dp_1src(
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011010110000000000u32 << 12u32
                | u32::from(opc.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod REV32_64_dp_1src {
    #[inline]
    pub fn REV32_64_dp_1src(
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011010110000000000u32 << 12u32
                | u32::from(opc.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod REV_64_dp_1src {
    #[inline]
    pub fn REV_64_dp_1src(
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011010110000000000u32 << 12u32
                | u32::from(opc.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CLZ_64_dp_1src {
    #[inline]
    pub fn CLZ_64_dp_1src(
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000000010u32 << 11u32
                | u32::from(op.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CLS_64_dp_1src {
    #[inline]
    pub fn CLS_64_dp_1src(
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000000010u32 << 11u32
                | u32::from(op.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CTZ_64_dp_1src {
    #[inline]
    pub fn CTZ_64_dp_1src(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101101011000000000110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CNT_64_dp_1src {
    #[inline]
    pub fn CNT_64_dp_1src(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101101011000000000111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ABS_64_dp_1src {
    #[inline]
    pub fn ABS_64_dp_1src(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101101011000000001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod PACIA_64P_dp_1src {
    #[inline]
    pub fn PACIA_64P_dp_1src(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | u32::from(Z.into()) << 13u32
                | 0b000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod PACIB_64P_dp_1src {
    #[inline]
    pub fn PACIB_64P_dp_1src(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | u32::from(Z.into()) << 13u32
                | 0b001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod PACDA_64P_dp_1src {
    #[inline]
    pub fn PACDA_64P_dp_1src(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | u32::from(Z.into()) << 13u32
                | 0b010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod PACDB_64P_dp_1src {
    #[inline]
    pub fn PACDB_64P_dp_1src(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | u32::from(Z.into()) << 13u32
                | 0b011u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod AUTIA_64P_dp_1src {
    #[inline]
    pub fn AUTIA_64P_dp_1src(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | u32::from(Z.into()) << 13u32
                | 0b100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod AUTIB_64P_dp_1src {
    #[inline]
    pub fn AUTIB_64P_dp_1src(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | u32::from(Z.into()) << 13u32
                | 0b101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod AUTDA_64P_dp_1src {
    #[inline]
    pub fn AUTDA_64P_dp_1src(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | u32::from(Z.into()) << 13u32
                | 0b110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod AUTDB_64P_dp_1src {
    #[inline]
    pub fn AUTDB_64P_dp_1src(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | u32::from(Z.into()) << 13u32
                | 0b111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod PACIZA_64Z_dp_1src {
    #[inline]
    pub fn PACIZA_64Z_dp_1src(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | u32::from(Z.into()) << 13u32
                | 0b00011111u32 << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod PACIZB_64Z_dp_1src {
    #[inline]
    pub fn PACIZB_64Z_dp_1src(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | u32::from(Z.into()) << 13u32
                | 0b00111111u32 << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod PACDZA_64Z_dp_1src {
    #[inline]
    pub fn PACDZA_64Z_dp_1src(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | u32::from(Z.into()) << 13u32
                | 0b01011111u32 << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod PACDZB_64Z_dp_1src {
    #[inline]
    pub fn PACDZB_64Z_dp_1src(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | u32::from(Z.into()) << 13u32
                | 0b01111111u32 << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod AUTIZA_64Z_dp_1src {
    #[inline]
    pub fn AUTIZA_64Z_dp_1src(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | u32::from(Z.into()) << 13u32
                | 0b10011111u32 << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod AUTIZB_64Z_dp_1src {
    #[inline]
    pub fn AUTIZB_64Z_dp_1src(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | u32::from(Z.into()) << 13u32
                | 0b10111111u32 << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod AUTDZA_64Z_dp_1src {
    #[inline]
    pub fn AUTDZA_64Z_dp_1src(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | u32::from(Z.into()) << 13u32
                | 0b11011111u32 << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod AUTDZB_64Z_dp_1src {
    #[inline]
    pub fn AUTDZB_64Z_dp_1src(
        Z: impl Into<::aarchmrs_types::BitValue<1>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000100u32 << 14u32
                | u32::from(Z.into()) << 13u32
                | 0b11111111u32 << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod XPACI_64Z_dp_1src {
    #[inline]
    pub fn XPACI_64Z_dp_1src(
        D: impl Into<::aarchmrs_types::BitValue<1>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000101000u32 << 11u32
                | u32::from(D.into()) << 10u32
                | 0b11111u32 << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod XPACD_64Z_dp_1src {
    #[inline]
    pub fn XPACD_64Z_dp_1src(
        D: impl Into<::aarchmrs_types::BitValue<1>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110110101100000101000u32 << 11u32
                | u32::from(D.into()) << 10u32
                | 0b11111u32 << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod PACNBIASPPC_64LR_dp_1src {
    #[inline]
    pub fn PACNBIASPPC_64LR_dp_1src() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11011010110000011000001111111110u32 << 0u32)
    }
}
pub mod PACNBIBSPPC_64LR_dp_1src {
    #[inline]
    pub fn PACNBIBSPPC_64LR_dp_1src() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11011010110000011000011111111110u32 << 0u32)
    }
}
pub mod PACIA171615_64LR_dp_1src {
    #[inline]
    pub fn PACIA171615_64LR_dp_1src() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11011010110000011000101111111110u32 << 0u32)
    }
}
pub mod PACIB171615_64LR_dp_1src {
    #[inline]
    pub fn PACIB171615_64LR_dp_1src() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11011010110000011000111111111110u32 << 0u32)
    }
}
pub mod AUTIASPPCR_64LRR_dp_1src {
    #[inline]
    pub fn AUTIASPPCR_64LRR_dp_1src(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101101011000001100100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11110u32 << 0u32,
        )
    }
}
pub mod AUTIBSPPCR_64LRR_dp_1src {
    #[inline]
    pub fn AUTIBSPPCR_64LRR_dp_1src(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101101011000001100101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | 0b11110u32 << 0u32,
        )
    }
}
pub mod PACIASPPC_64LR_dp_1src {
    #[inline]
    pub fn PACIASPPC_64LR_dp_1src() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11011010110000011010001111111110u32 << 0u32)
    }
}
pub mod PACIBSPPC_64LR_dp_1src {
    #[inline]
    pub fn PACIBSPPC_64LR_dp_1src() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11011010110000011010011111111110u32 << 0u32)
    }
}
pub mod AUTIA171615_64LR_dp_1src {
    #[inline]
    pub fn AUTIA171615_64LR_dp_1src() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11011010110000011011101111111110u32 << 0u32)
    }
}
pub mod AUTIB171615_64LR_dp_1src {
    #[inline]
    pub fn AUTIB171615_64LR_dp_1src() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11011010110000011011111111111110u32 << 0u32)
    }
}
