/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mad_z_p_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct mad_z_p_zzz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Za: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl mad_z_p_zzz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Za: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Zm: Zm.into(),
                Pg: Pg.into(),
                Za: Za.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b110u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Za) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
pub mod msb_z_p_zzz_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct msb_z_p_zzz_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Za: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl msb_z_p_zzz_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Za: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Zm: Zm.into(),
                Pg: Pg.into(),
                Za: Za.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b111u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Za) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
