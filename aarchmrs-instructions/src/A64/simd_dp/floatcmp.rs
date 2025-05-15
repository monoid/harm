/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FCMP_S_floatcmp {
    #[inline]
    pub fn FCMP_S_floatcmp(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(opc.into()) << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMP_SZ_floatcmp {
    #[inline]
    pub fn FCMP_SZ_floatcmp(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000100000001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(opc.into()) << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMPE_S_floatcmp {
    #[inline]
    pub fn FCMPE_S_floatcmp(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(opc.into()) << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMPE_SZ_floatcmp {
    #[inline]
    pub fn FCMPE_SZ_floatcmp(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111000100000001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(opc.into()) << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMP_D_floatcmp {
    #[inline]
    pub fn FCMP_D_floatcmp(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(opc.into()) << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMP_DZ_floatcmp {
    #[inline]
    pub fn FCMP_DZ_floatcmp(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001100000001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(opc.into()) << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMPE_D_floatcmp {
    #[inline]
    pub fn FCMPE_D_floatcmp(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(opc.into()) << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMPE_DZ_floatcmp {
    #[inline]
    pub fn FCMPE_DZ_floatcmp(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111001100000001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(opc.into()) << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMP_H_floatcmp {
    #[inline]
    pub fn FCMP_H_floatcmp(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(opc.into()) << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMP_HZ_floatcmp {
    #[inline]
    pub fn FCMP_HZ_floatcmp(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011100000001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(opc.into()) << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMPE_H_floatcmp {
    #[inline]
    pub fn FCMPE_H_floatcmp(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(opc.into()) << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
pub mod FCMPE_HZ_floatcmp {
    #[inline]
    pub fn FCMPE_HZ_floatcmp(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        opc: impl Into<::aarchmrs_types::BitValue<2>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001111011100000001000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(opc.into()) << 3u32
                | 0b000u32 << 0u32,
        )
    }
}
