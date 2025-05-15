/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FMUL_S_floatdp2 {
    #[inline]
    pub fn FMUL_S_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(op.into()) << 15u32
                | 0b00010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FDIV_S_floatdp2 {
    #[inline]
    pub fn FDIV_S_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FADD_S_floatdp2 {
    #[inline]
    pub fn FADD_S_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001u32 << 13u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FSUB_S_floatdp2 {
    #[inline]
    pub fn FSUB_S_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001u32 << 13u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMAX_S_floatdp2 {
    #[inline]
    pub fn FMAX_S_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMIN_S_floatdp2 {
    #[inline]
    pub fn FMIN_S_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMAXNM_S_floatdp2 {
    #[inline]
    pub fn FMAXNM_S_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMINNM_S_floatdp2 {
    #[inline]
    pub fn FMINNM_S_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FNMUL_S_floatdp2 {
    #[inline]
    pub fn FNMUL_S_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(op.into()) << 15u32
                | 0b00010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMUL_D_floatdp2 {
    #[inline]
    pub fn FMUL_D_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(op.into()) << 15u32
                | 0b00010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FDIV_D_floatdp2 {
    #[inline]
    pub fn FDIV_D_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FADD_D_floatdp2 {
    #[inline]
    pub fn FADD_D_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001u32 << 13u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FSUB_D_floatdp2 {
    #[inline]
    pub fn FSUB_D_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001u32 << 13u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMAX_D_floatdp2 {
    #[inline]
    pub fn FMAX_D_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMIN_D_floatdp2 {
    #[inline]
    pub fn FMIN_D_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMAXNM_D_floatdp2 {
    #[inline]
    pub fn FMAXNM_D_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMINNM_D_floatdp2 {
    #[inline]
    pub fn FMINNM_D_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FNMUL_D_floatdp2 {
    #[inline]
    pub fn FNMUL_D_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(op.into()) << 15u32
                | 0b00010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMUL_H_floatdp2 {
    #[inline]
    pub fn FMUL_H_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(op.into()) << 15u32
                | 0b00010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FDIV_H_floatdp2 {
    #[inline]
    pub fn FDIV_H_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FADD_H_floatdp2 {
    #[inline]
    pub fn FADD_H_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001u32 << 13u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FSUB_H_floatdp2 {
    #[inline]
    pub fn FSUB_H_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001u32 << 13u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMAX_H_floatdp2 {
    #[inline]
    pub fn FMAX_H_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMIN_H_floatdp2 {
    #[inline]
    pub fn FMIN_H_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMAXNM_H_floatdp2 {
    #[inline]
    pub fn FMAXNM_H_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMINNM_H_floatdp2 {
    #[inline]
    pub fn FMINNM_H_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b01u32 << 14u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FNMUL_H_floatdp2 {
    #[inline]
    pub fn FNMUL_H_floatdp2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011110111u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(op.into()) << 15u32
                | 0b00010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
