/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fdot_za_zzw_2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fdot_za_zzw_2x2 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fdot_za_zzw_2x2 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<4>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { Zm, Rv, Zn, off3 }
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
                    | 0b000u32 << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
pub mod bfdot_za_zzw_2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfdot_za_zzw_2x2 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl bfdot_za_zzw_2x2 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<4>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { Zm, Rv, Zn, off3 }
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
                    | 0b010u32 << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
pub mod fdot_za_z8z8w_2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fdot_za_z8z8w_2x2 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fdot_za_z8z8w_2x2 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<4>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { Zm, Rv, Zn, off3 }
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
                    | 0b100u32 << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
pub mod fdot_za32_z8z8w_2x2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fdot_za32_z8z8w_2x2 {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fdot_za32_z8z8w_2x2 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<4>,
            Rv: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<4>,
            off3: ::aarchmrs_types::BitValue<3>,
        ) -> Self {
            Self { Zm, Rv, Zn, off3 }
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
                    | 0b110u32 << 3u32
                    | self.off3.into_inner() << 0u32,
            )
        }
    }
}
