/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod incp_z_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct incp_z_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl incp_z_p_z_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pm: impl Into<::aarchmrs_types::BitValue<4>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pm: Pm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1011001000000u32 << 9u32
                    | u32::from(self.Pm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod decp_z_p_z_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct decp_z_p_z_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pm: ::aarchmrs_types::BitValue<4>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl decp_z_p_z_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pm: impl Into<::aarchmrs_types::BitValue<4>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pm: Pm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1011011000000u32 << 9u32
                    | u32::from(self.Pm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
