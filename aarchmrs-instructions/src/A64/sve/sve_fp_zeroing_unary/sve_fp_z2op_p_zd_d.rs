/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod flogb_z_p_z_z {
    #[inline]
    pub fn flogb_z_p_z_z(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01100100000111101u32 << 15u32
                | u32::from(size.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod fcvtzs_z_p_z_s2wz {
    #[inline]
    pub fn fcvtzs_z_p_z_s2wz(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001001001111110u32 << 14u32
                | u32::from(int_U.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod fcvtzs_z_p_z_d2wz {
    #[inline]
    pub fn fcvtzs_z_p_z_d2wz(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001001101111010u32 << 14u32
                | u32::from(int_U.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod fcvtzs_z_p_z_s2xz {
    #[inline]
    pub fn fcvtzs_z_p_z_s2xz(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001001101111110u32 << 14u32
                | u32::from(int_U.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod fcvtzs_z_p_z_d2xz {
    #[inline]
    pub fn fcvtzs_z_p_z_d2xz(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001001101111111u32 << 14u32
                | u32::from(int_U.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod fcvtzs_z_p_z_fp162hz {
    #[inline]
    pub fn fcvtzs_z_p_z_fp162hz(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001000101111011u32 << 14u32
                | u32::from(int_U.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod fcvtzs_z_p_z_fp162wz {
    #[inline]
    pub fn fcvtzs_z_p_z_fp162wz(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001000101111110u32 << 14u32
                | u32::from(int_U.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod fcvtzs_z_p_z_fp162xz {
    #[inline]
    pub fn fcvtzs_z_p_z_fp162xz(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001000101111111u32 << 14u32
                | u32::from(int_U.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod fcvtzu_z_p_z_s2wz {
    #[inline]
    pub fn fcvtzu_z_p_z_s2wz(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001001001111110u32 << 14u32
                | u32::from(int_U.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod fcvtzu_z_p_z_d2wz {
    #[inline]
    pub fn fcvtzu_z_p_z_d2wz(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001001101111010u32 << 14u32
                | u32::from(int_U.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod fcvtzu_z_p_z_s2xz {
    #[inline]
    pub fn fcvtzu_z_p_z_s2xz(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001001101111110u32 << 14u32
                | u32::from(int_U.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod fcvtzu_z_p_z_d2xz {
    #[inline]
    pub fn fcvtzu_z_p_z_d2xz(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001001101111111u32 << 14u32
                | u32::from(int_U.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod fcvtzu_z_p_z_fp162hz {
    #[inline]
    pub fn fcvtzu_z_p_z_fp162hz(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001000101111011u32 << 14u32
                | u32::from(int_U.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod fcvtzu_z_p_z_fp162wz {
    #[inline]
    pub fn fcvtzu_z_p_z_fp162wz(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001000101111110u32 << 14u32
                | u32::from(int_U.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod fcvtzu_z_p_z_fp162xz {
    #[inline]
    pub fn fcvtzu_z_p_z_fp162xz(
        int_U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011001000101111111u32 << 14u32
                | u32::from(int_U.into()) << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
