/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SHA512H_QQV_cryptosha512_3 {
    #[inline]
    pub fn SHA512H_QQV_cryptosha512_3(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b100000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SHA512H2_QQV_cryptosha512_3 {
    #[inline]
    pub fn SHA512H2_QQV_cryptosha512_3(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b100001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SHA512SU1_VVV2_cryptosha512_3 {
    #[inline]
    pub fn SHA512SU1_VVV2_cryptosha512_3(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b100010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod RAX1_VVV2_cryptosha512_3 {
    #[inline]
    pub fn RAX1_VVV2_cryptosha512_3(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b100011u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SM3PARTW1_VVV4_cryptosha512_3 {
    #[inline]
    pub fn SM3PARTW1_VVV4_cryptosha512_3(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b110000u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SM3PARTW2_VVV4_cryptosha512_3 {
    #[inline]
    pub fn SM3PARTW2_VVV4_cryptosha512_3(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b110001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SM4EKEY_VVV4_cryptosha512_3 {
    #[inline]
    pub fn SM4EKEY_VVV4_cryptosha512_3(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110011u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b110010u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
