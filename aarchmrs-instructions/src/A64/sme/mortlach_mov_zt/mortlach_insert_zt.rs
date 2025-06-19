/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod movt_zt_r_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct movt_zt_r_ {
        pub off3: ::aarchmrs_types::BitValue<3>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl movt_zt_r_ {
        #[inline]
        pub fn new(
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
            Rt: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                off3: off3.into(),
                Rt: Rt.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000010011100u32 << 15u32
                    | u32::from(self.off3) << 12u32
                    | 0b0011111u32 << 5u32
                    | u32::from(self.Rt) << 0u32,
            )
        }
    }
}
