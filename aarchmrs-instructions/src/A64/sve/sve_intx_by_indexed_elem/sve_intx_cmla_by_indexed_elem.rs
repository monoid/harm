/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cmla_z_zzzi_h {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cmla_z_zzzi_h {
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub rot: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl cmla_z_zzzi_h {
        #[inline]
        pub const fn new(
            i2: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<3>,
            rot: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                i2,
                Zm,
                rot,
                Zn,
                Zda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100101u32 << 21u32
                    | self.i2.into_inner() << 19u32
                    | self.Zm.into_inner() << 16u32
                    | 0b0110u32 << 12u32
                    | self.rot.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 0u32,
            )
        }
    }
}
pub mod cmla_z_zzzi_s {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cmla_z_zzzi_s {
        pub i1: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub rot: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zda: ::aarchmrs_types::BitValue<5>,
    }
    impl cmla_z_zzzi_s {
        #[inline]
        pub const fn new(
            i1: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<4>,
            rot: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zda: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                i1,
                Zm,
                rot,
                Zn,
                Zda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01000100111u32 << 21u32
                    | self.i1.into_inner() << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b0110u32 << 12u32
                    | self.rot.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zda.into_inner() << 0u32,
            )
        }
    }
}
