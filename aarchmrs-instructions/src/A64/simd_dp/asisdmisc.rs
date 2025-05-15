/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SUQADD_asisdmisc_R {
    #[inline]
    pub fn SUQADD_asisdmisc_R(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b100000001110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQABS_asisdmisc_R {
    #[inline]
    pub fn SQABS_asisdmisc_R(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b100000011110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CMGT_asisdmisc_Z {
    #[inline]
    pub fn CMGT_asisdmisc_Z(
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101111011100000100u32 << 13u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CMEQ_asisdmisc_Z {
    #[inline]
    pub fn CMEQ_asisdmisc_Z(
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101111011100000100u32 << 13u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CMLT_asisdmisc_Z {
    #[inline]
    pub fn CMLT_asisdmisc_Z(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101111011100000101010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod ABS_asisdmisc_R {
    #[inline]
    pub fn ABS_asisdmisc_R(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101111011100000101110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQXTN_asisdmisc_N {
    #[inline]
    pub fn SQXTN_asisdmisc_N(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b100001010010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCVTNS_asisdmisc_R {
    #[inline]
    pub fn FCVTNS_asisdmisc_R(
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110u32 << 24u32
                | u32::from(o2.into()) << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100001101u32 << 13u32
                | u32::from(o1.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCVTMS_asisdmisc_R {
    #[inline]
    pub fn FCVTMS_asisdmisc_R(
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110u32 << 24u32
                | u32::from(o2.into()) << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100001101u32 << 13u32
                | u32::from(o1.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCVTAS_asisdmisc_R {
    #[inline]
    pub fn FCVTAS_asisdmisc_R(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111100u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100001110010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SCVTF_asisdmisc_R {
    #[inline]
    pub fn SCVTF_asisdmisc_R(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111100u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100001110110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCMGT_asisdmisc_FZ {
    #[inline]
    pub fn FCMGT_asisdmisc_FZ(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111101u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100000110u32 << 13u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCMEQ_asisdmisc_FZ {
    #[inline]
    pub fn FCMEQ_asisdmisc_FZ(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111101u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100000110u32 << 13u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCMLT_asisdmisc_FZ {
    #[inline]
    pub fn FCMLT_asisdmisc_FZ(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111101u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100000111010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCVTPS_asisdmisc_R {
    #[inline]
    pub fn FCVTPS_asisdmisc_R(
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110u32 << 24u32
                | u32::from(o2.into()) << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100001101u32 << 13u32
                | u32::from(o1.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCVTZS_asisdmisc_R {
    #[inline]
    pub fn FCVTZS_asisdmisc_R(
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011110u32 << 24u32
                | u32::from(o2.into()) << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100001101u32 << 13u32
                | u32::from(o1.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FRECPE_asisdmisc_R {
    #[inline]
    pub fn FRECPE_asisdmisc_R(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111101u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100001110110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FRECPX_asisdmisc_R {
    #[inline]
    pub fn FRECPX_asisdmisc_R(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111101u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100001111110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod USQADD_asisdmisc_R {
    #[inline]
    pub fn USQADD_asisdmisc_R(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b100000001110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQNEG_asisdmisc_R {
    #[inline]
    pub fn SQNEG_asisdmisc_R(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b100000011110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CMGE_asisdmisc_Z {
    #[inline]
    pub fn CMGE_asisdmisc_Z(
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0111111011100000100u32 << 13u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CMLE_asisdmisc_Z {
    #[inline]
    pub fn CMLE_asisdmisc_Z(
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0111111011100000100u32 << 13u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod NEG_asisdmisc_R {
    #[inline]
    pub fn NEG_asisdmisc_R(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0111111011100000101110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQXTUN_asisdmisc_N {
    #[inline]
    pub fn SQXTUN_asisdmisc_N(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b100001001010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UQXTN_asisdmisc_N {
    #[inline]
    pub fn UQXTN_asisdmisc_N(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b100001010010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCVTXN_asisdmisc_N {
    #[inline]
    pub fn FCVTXN_asisdmisc_N(
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0111111001100001011010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCVTNU_asisdmisc_R {
    #[inline]
    pub fn FCVTNU_asisdmisc_R(
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(o2.into()) << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100001101u32 << 13u32
                | u32::from(o1.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCVTMU_asisdmisc_R {
    #[inline]
    pub fn FCVTMU_asisdmisc_R(
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(o2.into()) << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100001101u32 << 13u32
                | u32::from(o1.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCVTAU_asisdmisc_R {
    #[inline]
    pub fn FCVTAU_asisdmisc_R(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111100u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100001110010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UCVTF_asisdmisc_R {
    #[inline]
    pub fn UCVTF_asisdmisc_R(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111100u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100001110110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCMGE_asisdmisc_FZ {
    #[inline]
    pub fn FCMGE_asisdmisc_FZ(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111101u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100000110u32 << 13u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCMLE_asisdmisc_FZ {
    #[inline]
    pub fn FCMLE_asisdmisc_FZ(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111101u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100000110u32 << 13u32
                | u32::from(op.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCVTPU_asisdmisc_R {
    #[inline]
    pub fn FCVTPU_asisdmisc_R(
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(o2.into()) << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100001101u32 << 13u32
                | u32::from(o1.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCVTZU_asisdmisc_R {
    #[inline]
    pub fn FCVTZU_asisdmisc_R(
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01111110u32 << 24u32
                | u32::from(o2.into()) << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100001101u32 << 13u32
                | u32::from(o1.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FRSQRTE_asisdmisc_R {
    #[inline]
    pub fn FRSQRTE_asisdmisc_R(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111101u32 << 23u32
                | u32::from(sz.into()) << 22u32
                | 0b100001110110u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
