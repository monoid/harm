/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ld1q_z_p_ar_d_64_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ld1q_z_p_ar_d_64_unscaled {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl ld1q_z_p_ar_d_64_unscaled {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                Zt: Zt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000100000u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | 0b101u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zt) << 0u32,
            )
        }
    }
}
