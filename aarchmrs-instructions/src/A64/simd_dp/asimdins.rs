/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod DUP_asimdins_DV_v {
    #[inline]
    pub fn DUP_asimdins_DV_v(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        imm5: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110000u32 << 21u32
                | u32::from(imm5.into()) << 16u32
                | 0b000001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod DUP_asimdins_DR_r {
    #[inline]
    pub fn DUP_asimdins_DR_r(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        imm5: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110000u32 << 21u32
                | u32::from(imm5.into()) << 16u32
                | 0b000011u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SMOV_asimdins_W_w {
    #[inline]
    pub fn SMOV_asimdins_W_w(
        imm5: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00001110000u32 << 21u32
                | u32::from(imm5.into()) << 16u32
                | 0b001011u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UMOV_asimdins_W_w {
    #[inline]
    pub fn UMOV_asimdins_W_w(
        imm5: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00001110000u32 << 21u32
                | u32::from(imm5.into()) << 16u32
                | 0b001111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod INS_asimdins_IR_r {
    #[inline]
    pub fn INS_asimdins_IR_r(
        imm5: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01001110000u32 << 21u32
                | u32::from(imm5.into()) << 16u32
                | 0b000111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SMOV_asimdins_X_x {
    #[inline]
    pub fn SMOV_asimdins_X_x(
        imm5: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01001110000u32 << 21u32
                | u32::from(imm5.into()) << 16u32
                | 0b001011u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UMOV_asimdins_X_x {
    #[inline]
    pub fn UMOV_asimdins_X_x(
        imm5: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01001110000u32 << 21u32
                | u32::from(imm5.into()) << 20u32
                | 0b1000001111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod INS_asimdins_IV_v {
    #[inline]
    pub fn INS_asimdins_IV_v(
        imm5: impl Into<::aarchmrs_types::BitValue<5>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01101110000u32 << 21u32
                | u32::from(imm5.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(imm4.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
