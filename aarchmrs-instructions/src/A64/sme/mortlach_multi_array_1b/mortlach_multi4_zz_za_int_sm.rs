/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod add_za_zzv_4x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct add_za_zzv_4x1 {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl add_za_zzv_4x1 {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            S: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                sz,
                Zm,
                Rv,
                Zn,
                S,
                off3,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010u32 << 23u32
                    | self.sz.into_inner() << 22u32
                    | 0b11u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b110u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.S.into_inner() << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
pub mod sub_za_zzv_4x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sub_za_zzv_4x1 {
        pub sz: ::aarchmrs_types::BitValue<1>,
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl sub_za_zzv_4x1 {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<1>,
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<5>,
            S: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
                sz,
                Zm,
                Rv,
                Zn,
                S,
                off3,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010u32 << 23u32
                    | self.sz.into_inner() << 22u32
                    | 0b11u32 << 20u32
                    | self.Zm.into_inner() << 16u32
                    | 0b0u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b110u32 << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b1u32 << 4u32
                    | self.S.into_inner() << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
