/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod insr_z_v_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct insr_z_v_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Vm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl insr_z_v_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Vm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Vm: Vm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b110100001110u32 << 10u32
                    | u32::from(self.Vm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
