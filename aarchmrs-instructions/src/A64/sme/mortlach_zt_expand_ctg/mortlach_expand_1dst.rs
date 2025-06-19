/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod luti2_z_ztz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct luti2_z_ztz_ {
        pub i4: ::aarchmrs_types::BitValue<4>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl luti2_z_ztz_ {
        #[inline]
        pub fn new(
            i4: impl Into<::aarchmrs_types::BitValue<4>>,
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i4: i4.into(),
                size: size.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000110011u32 << 18u32
                    | u32::from(self.i4) << 14u32
                    | u32::from(self.size) << 12u32
                    | 0b00u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod luti4_z_ztz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct luti4_z_ztz_ {
        pub i3: ::aarchmrs_types::BitValue<3>,
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl luti4_z_ztz_ {
        #[inline]
        pub fn new(
            i3: impl Into<::aarchmrs_types::BitValue<3>>,
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                i3: i3.into(),
                size: size.into(),
                Zn: Zn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000001100101u32 << 17u32
                    | u32::from(self.i3) << 14u32
                    | u32::from(self.size) << 12u32
                    | 0b00u32 << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
