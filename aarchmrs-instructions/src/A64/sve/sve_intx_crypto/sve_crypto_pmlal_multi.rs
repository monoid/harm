/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod pmlal_mz_zzzw_1x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct pmlal_mz_zzzw_1x2 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<4>,
    }
    impl pmlal_mz_zzzw_1x2 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<4>,
        ) -> Self {
            Self { Zm, Zn, Zda }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000101001u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b111111u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 1u32
                    | 0b0u32 << 0u32,
            )
        }
    }
}
