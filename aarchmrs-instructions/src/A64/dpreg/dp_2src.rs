/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod UDIV_32_dp_2src {
    #[inline]
    pub fn UDIV_32_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b00001u32 << 11u32
                | u32::from(o1.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SDIV_32_dp_2src {
    #[inline]
    pub fn SDIV_32_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b00001u32 << 11u32
                | u32::from(o1.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod LSLV_32_dp_2src {
    #[inline]
    pub fn LSLV_32_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op2: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0010u32 << 12u32
                | u32::from(op2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod LSRV_32_dp_2src {
    #[inline]
    pub fn LSRV_32_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op2: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0010u32 << 12u32
                | u32::from(op2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ASRV_32_dp_2src {
    #[inline]
    pub fn ASRV_32_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op2: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0010u32 << 12u32
                | u32::from(op2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod RORV_32_dp_2src {
    #[inline]
    pub fn RORV_32_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op2: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0010u32 << 12u32
                | u32::from(op2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CRC32B_32C_dp_2src {
    #[inline]
    pub fn CRC32B_32C_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        C: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(C.into()) << 12u32
                | u32::from(sz.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CRC32H_32C_dp_2src {
    #[inline]
    pub fn CRC32H_32C_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        C: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(C.into()) << 12u32
                | u32::from(sz.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CRC32W_32C_dp_2src {
    #[inline]
    pub fn CRC32W_32C_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        C: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(C.into()) << 12u32
                | u32::from(sz.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CRC32CB_32C_dp_2src {
    #[inline]
    pub fn CRC32CB_32C_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        C: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(C.into()) << 12u32
                | u32::from(sz.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CRC32CH_32C_dp_2src {
    #[inline]
    pub fn CRC32CH_32C_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        C: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(C.into()) << 12u32
                | u32::from(sz.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CRC32CW_32C_dp_2src {
    #[inline]
    pub fn CRC32CW_32C_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        C: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(C.into()) << 12u32
                | u32::from(sz.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SMAX_32_dp_2src {
    #[inline]
    pub fn SMAX_32_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UMAX_32_dp_2src {
    #[inline]
    pub fn UMAX_32_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SMIN_32_dp_2src {
    #[inline]
    pub fn SMIN_32_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UMIN_32_dp_2src {
    #[inline]
    pub fn UMIN_32_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011011u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SUBP_64S_dp_2src {
    #[inline]
    pub fn SUBP_64S_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UDIV_64_dp_2src {
    #[inline]
    pub fn UDIV_64_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b00001u32 << 11u32
                | u32::from(o1.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SDIV_64_dp_2src {
    #[inline]
    pub fn SDIV_64_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b00001u32 << 11u32
                | u32::from(o1.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod IRG_64I_dp_2src {
    #[inline]
    pub fn IRG_64I_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod GMI_64G_dp_2src {
    #[inline]
    pub fn GMI_64G_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod LSLV_64_dp_2src {
    #[inline]
    pub fn LSLV_64_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op2: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0010u32 << 12u32
                | u32::from(op2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod LSRV_64_dp_2src {
    #[inline]
    pub fn LSRV_64_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op2: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0010u32 << 12u32
                | u32::from(op2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ASRV_64_dp_2src {
    #[inline]
    pub fn ASRV_64_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op2: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0010u32 << 12u32
                | u32::from(op2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod RORV_64_dp_2src {
    #[inline]
    pub fn RORV_64_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        op2: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0010u32 << 12u32
                | u32::from(op2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod PACGA_64P_dp_2src {
    #[inline]
    pub fn PACGA_64P_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b001100u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CRC32X_64C_dp_2src {
    #[inline]
    pub fn CRC32X_64C_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        C: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(C.into()) << 12u32
                | u32::from(sz.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CRC32CX_64C_dp_2src {
    #[inline]
    pub fn CRC32CX_64C_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        C: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b010u32 << 13u32
                | u32::from(C.into()) << 12u32
                | u32::from(sz.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SMAX_64_dp_2src {
    #[inline]
    pub fn SMAX_64_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UMAX_64_dp_2src {
    #[inline]
    pub fn UMAX_64_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SMIN_64_dp_2src {
    #[inline]
    pub fn SMIN_64_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UMIN_64_dp_2src {
    #[inline]
    pub fn UMIN_64_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b011011u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SUBPS_64S_dp_2src {
    #[inline]
    pub fn SUBPS_64S_dp_2src(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111010110u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b000000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
