/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fadd_za_zw_4x4_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fadd_za_zw_4x4_16 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fadd_za_zw_4x4_16 {
        #[inline]
        pub const fn new(
            Rv: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { Rv, Zm, S, off3 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001101001010u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b111u32 << 10u32
                    | self.Zm.into_inner() << 7u32
                    | 0b000u32 << 4u32
                    | self.S.into_inner() << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
pub mod bfadd_za_zw_4x4_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfadd_za_zw_4x4_16 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl bfadd_za_zw_4x4_16 {
        #[inline]
        pub const fn new(
            Rv: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { Rv, Zm, S, off3 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001111001010u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b111u32 << 10u32
                    | self.Zm.into_inner() << 7u32
                    | 0b000u32 << 4u32
                    | self.S.into_inner() << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
pub mod fsub_za_zw_4x4_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fsub_za_zw_4x4_16 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fsub_za_zw_4x4_16 {
        #[inline]
        pub const fn new(
            Rv: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { Rv, Zm, S, off3 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001101001010u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b111u32 << 10u32
                    | self.Zm.into_inner() << 7u32
                    | 0b000u32 << 4u32
                    | self.S.into_inner() << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
pub mod bfsub_za_zw_4x4_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfsub_za_zw_4x4_16 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zm: ::aarchmrs_types::BitValue<3>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl bfsub_za_zw_4x4_16 {
        #[inline]
        pub const fn new(
            Rv: ::aarchmrs_types::BitValue<2>,
            Zm: ::aarchmrs_types::BitValue<3>,
            S: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { Rv, Zm, S, off3 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000001111001010u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b111u32 << 10u32
                    | self.Zm.into_inner() << 7u32
                    | 0b000u32 << 4u32
                    | self.S.into_inner() << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
