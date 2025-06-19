/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ldr_za_ri_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ldr_za_ri_ {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub off4: ::aarchmrs_types::BitValue<4>,
    }
    impl ldr_za_ri_ {
        #[inline]
        pub fn new(
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            off4: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Rv: Rv.into(),
                Rn: Rn.into(),
                off4: off4.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100001000000000u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.off4) << 0u32,
            )
        }
    }
}
pub mod str_za_ri_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct str_za_ri_ {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub off4: ::aarchmrs_types::BitValue<4>,
    }
    impl str_za_ri_ {
        #[inline]
        pub fn new(
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            off4: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Rv: Rv.into(),
                Rn: Rn.into(),
                off4: off4.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100001001000000u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.off4) << 0u32,
            )
        }
    }
}
