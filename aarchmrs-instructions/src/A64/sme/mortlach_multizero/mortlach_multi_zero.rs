/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod zero_za1_ri_2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct zero_za1_ri_2 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl zero_za1_ri_2 {
        #[inline]
        pub fn new(
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Rv: Rv.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000000011000u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b0000000000u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod zero_za1_ri_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct zero_za1_ri_4 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl zero_za1_ri_4 {
        #[inline]
        pub fn new(
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Rv: Rv.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000000011100u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b0000000000u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod zero_za2_ri_1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct zero_za2_ri_1 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl zero_za2_ri_1 {
        #[inline]
        pub fn new(
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Rv: Rv.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000000011001u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b0000000000u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod zero_za2_ri_2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct zero_za2_ri_2 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl zero_za2_ri_2 {
        #[inline]
        pub fn new(
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Rv: Rv.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000000011010u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b00000000000u32 << 2u32
                    | u32::from(self.off2) << 0u32,
            )
        }
    }
}
pub mod zero_za2_ri_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct zero_za2_ri_4 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl zero_za2_ri_4 {
        #[inline]
        pub fn new(
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Rv: Rv.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000000011011u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b00000000000u32 << 2u32
                    | u32::from(self.off2) << 0u32,
            )
        }
    }
}
pub mod zero_za4_ri_1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct zero_za4_ri_1 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub off2: ::aarchmrs_types::BitValue<2>,
    }
    impl zero_za4_ri_1 {
        #[inline]
        pub fn new(
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Rv: Rv.into(),
                off2: off2.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000000011101u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b00000000000u32 << 2u32
                    | u32::from(self.off2) << 0u32,
            )
        }
    }
}
pub mod zero_za4_ri_2 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct zero_za4_ri_2 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub o1: ::aarchmrs_types::BitValue<1>,
    }
    impl zero_za4_ri_2 {
        #[inline]
        pub fn new(
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                Rv: Rv.into(),
                o1: o1.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000000011110u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b000000000000u32 << 1u32
                    | u32::from(self.o1) << 0u32,
            )
        }
    }
}
pub mod zero_za4_ri_4 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct zero_za4_ri_4 {
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub o1: ::aarchmrs_types::BitValue<1>,
    }
    impl zero_za4_ri_4 {
        #[inline]
        pub fn new(
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                Rv: Rv.into(),
                o1: o1.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11000000000011111u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b000000000000u32 << 1u32
                    | u32::from(self.o1) << 0u32,
            )
        }
    }
}
