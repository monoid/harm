/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CSEL_32_condsel {
    #[inline]
    pub fn CSEL_32_condsel(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b0u32 << 11u32
                | u32::from(o2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CSINC_32_condsel {
    #[inline]
    pub fn CSINC_32_condsel(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00011010100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b0u32 << 11u32
                | u32::from(o2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CSINV_32_condsel {
    #[inline]
    pub fn CSINV_32_condsel(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011010100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b0u32 << 11u32
                | u32::from(o2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CSNEG_32_condsel {
    #[inline]
    pub fn CSNEG_32_condsel(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01011010100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b0u32 << 11u32
                | u32::from(o2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CSEL_64_condsel {
    #[inline]
    pub fn CSEL_64_condsel(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b0u32 << 11u32
                | u32::from(o2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CSINC_64_condsel {
    #[inline]
    pub fn CSINC_64_condsel(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10011010100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b0u32 << 11u32
                | u32::from(o2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CSINV_64_condsel {
    #[inline]
    pub fn CSINV_64_condsel(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011010100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b0u32 << 11u32
                | u32::from(o2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod CSNEG_64_condsel {
    #[inline]
    pub fn CSNEG_64_condsel(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
        o2: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11011010100u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | u32::from(cond.into()) << 12u32
                | 0b0u32 << 11u32
                | u32::from(o2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
