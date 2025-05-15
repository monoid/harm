/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SADDLV_asimdall_only {
    #[inline]
    pub fn SADDLV_asimdall_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b110000001110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SMAXV_asimdall_only {
    #[inline]
    pub fn SMAXV_asimdall_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11000u32 << 17u32
                | u32::from(op.into()) << 16u32
                | 0b101010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SMINV_asimdall_only {
    #[inline]
    pub fn SMINV_asimdall_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11000u32 << 17u32
                | u32::from(op.into()) << 16u32
                | 0b101010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ADDV_asimdall_only {
    #[inline]
    pub fn ADDV_asimdall_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b110001101110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMAXNMV_asimdall_only_H {
    #[inline]
    pub fn FMAXNMV_asimdall_only_H(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110u32 << 24u32
                | u32::from(o1.into()) << 23u32
                | 0b0110000110010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMAXV_asimdall_only_H {
    #[inline]
    pub fn FMAXV_asimdall_only_H(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110u32 << 24u32
                | u32::from(o1.into()) << 23u32
                | 0b0110000111110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMINNMV_asimdall_only_H {
    #[inline]
    pub fn FMINNMV_asimdall_only_H(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110u32 << 24u32
                | u32::from(o1.into()) << 23u32
                | 0b0110000110010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMINV_asimdall_only_H {
    #[inline]
    pub fn FMINV_asimdall_only_H(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b001110u32 << 24u32
                | u32::from(o1.into()) << 23u32
                | 0b0110000111110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UADDLV_asimdall_only {
    #[inline]
    pub fn UADDLV_asimdall_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b110000001110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UMAXV_asimdall_only {
    #[inline]
    pub fn UMAXV_asimdall_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11000u32 << 17u32
                | u32::from(op.into()) << 16u32
                | 0b101010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UMINV_asimdall_only {
    #[inline]
    pub fn UMINV_asimdall_only(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11000u32 << 17u32
                | u32::from(op.into()) << 16u32
                | 0b101010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMAXNMV_asimdall_only_SD {
    #[inline]
    pub fn FMAXNMV_asimdall_only_SD(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110u32 << 24u32
                | u32::from(o1.into()) << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b110000110010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMAXV_asimdall_only_SD {
    #[inline]
    pub fn FMAXV_asimdall_only_SD(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110u32 << 24u32
                | u32::from(o1.into()) << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b110000111110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMINNMV_asimdall_only_SD {
    #[inline]
    pub fn FMINNMV_asimdall_only_SD(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110u32 << 24u32
                | u32::from(o1.into()) << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b110000110010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FMINV_asimdall_only_SD {
    #[inline]
    pub fn FMINV_asimdall_only_SD(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b101110u32 << 24u32
                | u32::from(o1.into()) << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b110000111110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
