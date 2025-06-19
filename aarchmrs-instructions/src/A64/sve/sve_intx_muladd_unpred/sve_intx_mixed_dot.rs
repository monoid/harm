/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod usdot_z_zzz_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct usdot_z_zzz_s {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl usdot_z_zzz_s {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zm, Zn, Zda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100100u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b011110u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 0u32,
            )
        }
    }
}
