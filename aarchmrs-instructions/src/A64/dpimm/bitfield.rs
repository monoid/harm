/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SBFM_32M_bitfield {
    #[inline]
    pub fn SBFM_32M_bitfield(
        immr: impl Into<::aarchmrs_types::BitValue<6>>,
        imms: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0001001100u32 << 22u32
                | u32::from(immr.into()) << 16u32
                | u32::from(imms.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod BFM_32M_bitfield {
    #[inline]
    pub fn BFM_32M_bitfield(
        immr: impl Into<::aarchmrs_types::BitValue<6>>,
        imms: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0011001100u32 << 22u32
                | u32::from(immr.into()) << 16u32
                | u32::from(imms.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UBFM_32M_bitfield {
    #[inline]
    pub fn UBFM_32M_bitfield(
        immr: impl Into<::aarchmrs_types::BitValue<6>>,
        imms: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0101001100u32 << 22u32
                | u32::from(immr.into()) << 16u32
                | u32::from(imms.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SBFM_64M_bitfield {
    #[inline]
    pub fn SBFM_64M_bitfield(
        immr: impl Into<::aarchmrs_types::BitValue<6>>,
        imms: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1001001101u32 << 22u32
                | u32::from(immr.into()) << 16u32
                | u32::from(imms.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod BFM_64M_bitfield {
    #[inline]
    pub fn BFM_64M_bitfield(
        immr: impl Into<::aarchmrs_types::BitValue<6>>,
        imms: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1011001101u32 << 22u32
                | u32::from(immr.into()) << 16u32
                | u32::from(imms.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UBFM_64M_bitfield {
    #[inline]
    pub fn UBFM_64M_bitfield(
        immr: impl Into<::aarchmrs_types::BitValue<6>>,
        imms: impl Into<::aarchmrs_types::BitValue<6>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1101001101u32 << 22u32
                | u32::from(immr.into()) << 16u32
                | u32::from(imms.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
