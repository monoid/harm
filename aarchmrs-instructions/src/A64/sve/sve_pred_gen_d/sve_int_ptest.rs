/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ptest__p_p_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ptest__p_p_ {
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pn: ::aarchmrs_types::BitValue<4>,
    }
    impl ptest__p_p_ {
        #[inline]
        pub fn new(
            Pg: impl Into<::aarchmrs_types::BitValue<4>>,
            Pn: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Pg: Pg.into(),
                Pn: Pn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001001010101000011u32 << 14u32
                    | u32::from(self.Pg) << 10u32
                    | 0b0u32 << 9u32
                    | u32::from(self.Pn) << 5u32
                    | 0b00000u32 << 0u32,
            )
        }
    }
}
