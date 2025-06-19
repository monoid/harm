/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod movaz_z_rza_b {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct movaz_z_rza_b {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub off4: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl movaz_z_rza_b {
        #[inline]
        pub fn new(
            V: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<2>>,
            off4: impl Into<::aarchmrs_types::BitValue<4>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                V: V.into(),
                Rs: Rs.into(),
                off4: off4.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000000000010u32 << 16u32
                    | u32::from(self.V) << 15u32
                    | u32::from(self.Rs) << 13u32
                    | 0b0001u32 << 9u32
                    | u32::from(self.off4) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod movaz_z_rza_h {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct movaz_z_rza_h {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub ZAn: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl movaz_z_rza_h {
        #[inline]
        pub fn new(
            V: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<2>>,
            ZAn: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                V: V.into(),
                Rs: Rs.into(),
                ZAn: ZAn.into(),
                off3: off3.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000001000010u32 << 16u32
                    | u32::from(self.V) << 15u32
                    | u32::from(self.Rs) << 13u32
                    | 0b0001u32 << 9u32
                    | u32::from(self.ZAn) << 8u32
                    | u32::from(self.off3) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod movaz_z_rza_w {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct movaz_z_rza_w {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub ZAn: ::aarchmrs_types::BitValue<2>,
        pub off2: ::aarchmrs_types::BitValue<2>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl movaz_z_rza_w {
        #[inline]
        pub fn new(
            V: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<2>>,
            ZAn: impl Into<::aarchmrs_types::BitValue<2>>,
            off2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                V: V.into(),
                Rs: Rs.into(),
                ZAn: ZAn.into(),
                off2: off2.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000010000010u32 << 16u32
                    | u32::from(self.V) << 15u32
                    | u32::from(self.Rs) << 13u32
                    | 0b0001u32 << 9u32
                    | u32::from(self.ZAn) << 7u32
                    | u32::from(self.off2) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod movaz_z_rza_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct movaz_z_rza_d {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub ZAn: ::aarchmrs_types::BitValue<3>,
        pub o1: ::aarchmrs_types::BitValue<1>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl movaz_z_rza_d {
        #[inline]
        pub fn new(
            V: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<2>>,
            ZAn: impl Into<::aarchmrs_types::BitValue<3>>,
            o1: impl Into<::aarchmrs_types::BitValue<1>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                V: V.into(),
                Rs: Rs.into(),
                ZAn: ZAn.into(),
                o1: o1.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000011000010u32 << 16u32
                    | u32::from(self.V) << 15u32
                    | u32::from(self.Rs) << 13u32
                    | 0b0001u32 << 9u32
                    | u32::from(self.ZAn) << 6u32
                    | u32::from(self.o1) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
pub mod movaz_z_rza_q {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct movaz_z_rza_q {
        pub V: ::aarchmrs_types::BitValue<1>,
        pub Rs: ::aarchmrs_types::BitValue<2>,
        pub ZAn: ::aarchmrs_types::BitValue<4>,
        pub Zd: ::aarchmrs_types::BitValue<5>,
    }
    impl movaz_z_rza_q {
        #[inline]
        pub fn new(
            V: impl Into<::aarchmrs_types::BitValue<1>>,
            Rs: impl Into<::aarchmrs_types::BitValue<2>>,
            ZAn: impl Into<::aarchmrs_types::BitValue<4>>,
            Zd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                V: V.into(),
                Rs: Rs.into(),
                ZAn: ZAn.into(),
                Zd: Zd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1100000011000011u32 << 16u32
                    | u32::from(self.V) << 15u32
                    | u32::from(self.Rs) << 13u32
                    | 0b0001u32 << 9u32
                    | u32::from(self.ZAn) << 5u32
                    | u32::from(self.Zd) << 0u32,
            )
        }
    }
}
