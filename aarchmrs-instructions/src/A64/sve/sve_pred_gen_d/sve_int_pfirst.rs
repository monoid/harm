/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod pfirst_p_p_p_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct pfirst_p_p_p_ {
        pub Pg: ::aarchmrs_types::BitValue<4>,
        pub Pdn: ::aarchmrs_types::BitValue<4>,
    }
    impl pfirst_p_p_p_ {
        #[inline]
        pub fn new(
            Pg: impl Into<::aarchmrs_types::BitValue<4>>,
            Pdn: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Pg: Pg.into(),
                Pdn: Pdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101010110001100000u32 << 9u32
                    | u32::from(self.Pg) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.Pdn) << 0u32,
            )
        }
    }
}
