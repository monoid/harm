/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cntp_r_pn_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cntp_r_pn_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub vl: ::aarchmrs_types::BitValue<1>,
        pub PNn: ::aarchmrs_types::BitValue<4>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl cntp_r_pn_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            vl: impl Into<::aarchmrs_types::BitValue<1>>,
            PNn: impl Into<::aarchmrs_types::BitValue<4>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                vl: vl.into(),
                PNn: PNn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b10000010000u32 << 11u32
                    | u32::from(self.vl) << 10u32
                    | 0b1u32 << 9u32
                    | u32::from(self.PNn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
