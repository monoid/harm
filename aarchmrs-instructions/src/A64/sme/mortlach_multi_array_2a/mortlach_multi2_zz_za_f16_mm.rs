/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmla_za_zzw_2x2_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmla_za_zzw_2x2_16 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fmla_za_zzw_2x2_16 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<4>,
            S: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
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
                0b11000001101u32 << 21u32
                    | self.Zm.into_inner() << 17u32
                    | 0b00u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b100u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0u32 << 5u32
                    | self.S.into_inner() << 4u32
                    | 0b1u32 << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
pub mod bfmla_za_zzw_2x2_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmla_za_zzw_2x2_16 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl bfmla_za_zzw_2x2_16 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<4>,
            S: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
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
                0b11000001111u32 << 21u32
                    | self.Zm.into_inner() << 17u32
                    | 0b00u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b100u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0u32 << 5u32
                    | self.S.into_inner() << 4u32
                    | 0b1u32 << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
pub mod fmls_za_zzw_2x2_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmls_za_zzw_2x2_16 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fmls_za_zzw_2x2_16 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<4>,
            S: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
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
                0b11000001101u32 << 21u32
                    | self.Zm.into_inner() << 17u32
                    | 0b00u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b100u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0u32 << 5u32
                    | self.S.into_inner() << 4u32
                    | 0b1u32 << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
pub mod bfmls_za_zzw_2x2_16 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfmls_za_zzw_2x2_16 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl bfmls_za_zzw_2x2_16 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<4>,
            S: ::aarchmrs_types::BitValue<1>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self {
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
                0b11000001111u32 << 21u32
                    | self.Zm.into_inner() << 17u32
                    | 0b00u32 << 15u32
                    | self.Rv.into_inner() << 13u32
                    | 0b100u32 << 10u32
                    | self.Zn.into_inner() << 6u32
                    | 0b0u32 << 5u32
                    | self.S.into_inner() << 4u32
                    | 0b1u32 << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
