/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod frint32z_z_p_z_m {
    #[inline]
    pub fn frint32z_z_p_z_m(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101000100u32 << 18u32
                | u32::from(sz.into()) << 17u32
                | 0b0101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod frint32x_z_p_z_m {
    #[inline]
    pub fn frint32x_z_p_z_m(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101000100u32 << 18u32
                | u32::from(sz.into()) << 17u32
                | 0b1101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod frint64z_z_p_z_m {
    #[inline]
    pub fn frint64z_z_p_z_m(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101000101u32 << 18u32
                | u32::from(sz.into()) << 17u32
                | 0b0101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod frint64x_z_p_z_m {
    #[inline]
    pub fn frint64x_z_p_z_m(
        sz: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100101000101u32 << 18u32
                | u32::from(sz.into()) << 17u32
                | 0b1101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod scvtf_z_p_z_w2s {
    #[inline]
    pub fn scvtf_z_p_z_w2s(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001011001010u32 << 17u32
                | u32::from(int_U.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod scvtf_z_p_z_w2d {
    #[inline]
    pub fn scvtf_z_p_z_w2d(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001011101000u32 << 17u32
                | u32::from(int_U.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod scvtf_z_p_z_x2s {
    #[inline]
    pub fn scvtf_z_p_z_x2s(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001011101010u32 << 17u32
                | u32::from(int_U.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod scvtf_z_p_z_x2d {
    #[inline]
    pub fn scvtf_z_p_z_x2d(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001011101011u32 << 17u32
                | u32::from(int_U.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod scvtf_z_p_z_h2fp16 {
    #[inline]
    pub fn scvtf_z_p_z_h2fp16(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001010101001u32 << 17u32
                | u32::from(int_U.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod scvtf_z_p_z_w2fp16 {
    #[inline]
    pub fn scvtf_z_p_z_w2fp16(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001010101010u32 << 17u32
                | u32::from(int_U.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod scvtf_z_p_z_x2fp16 {
    #[inline]
    pub fn scvtf_z_p_z_x2fp16(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001010101011u32 << 17u32
                | u32::from(int_U.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod ucvtf_z_p_z_w2s {
    #[inline]
    pub fn ucvtf_z_p_z_w2s(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001011001010u32 << 17u32
                | u32::from(int_U.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod ucvtf_z_p_z_w2d {
    #[inline]
    pub fn ucvtf_z_p_z_w2d(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001011101000u32 << 17u32
                | u32::from(int_U.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod ucvtf_z_p_z_x2s {
    #[inline]
    pub fn ucvtf_z_p_z_x2s(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001011101010u32 << 17u32
                | u32::from(int_U.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod ucvtf_z_p_z_x2d {
    #[inline]
    pub fn ucvtf_z_p_z_x2d(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001011101011u32 << 17u32
                | u32::from(int_U.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod ucvtf_z_p_z_h2fp16 {
    #[inline]
    pub fn ucvtf_z_p_z_h2fp16(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001010101001u32 << 17u32
                | u32::from(int_U.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod ucvtf_z_p_z_w2fp16 {
    #[inline]
    pub fn ucvtf_z_p_z_w2fp16(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001010101010u32 << 17u32
                | u32::from(int_U.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod ucvtf_z_p_z_x2fp16 {
    #[inline]
    pub fn ucvtf_z_p_z_x2fp16(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001010101011u32 << 17u32
                | u32::from(int_U.into()) << 16u32
                | 0b101u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
