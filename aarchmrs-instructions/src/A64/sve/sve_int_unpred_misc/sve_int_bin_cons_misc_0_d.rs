/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod movprfx_z_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct movprfx_z_z_ {
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl movprfx_z_z_ {
        #[inline]
        pub const fn new(
            Zn: ::aarchmrs_types::BitValue<5>,
            Zd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Zn, Zd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0000010000100000101111u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zd.into_inner() << 0u32,
            )
        }
    }
}
