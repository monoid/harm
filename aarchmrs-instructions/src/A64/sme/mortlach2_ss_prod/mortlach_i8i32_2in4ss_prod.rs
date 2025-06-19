/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod stmopa_za_zzzi_b2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct stmopa_za_zzzi_b2x1 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub K: ::aarchmrs_types::BitValue<1>,
        pub Zk: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl stmopa_za_zzzi_b2x1 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            K: ::aarchmrs_types::BitValue<1>,
            Zk: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<4>,
            i2: ::aarchmrs_types::BitValue<2>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                K,
                Zk,
                Zn,
                i2,
                ZAda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000010u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b100u32 << 13u32
                    | self.K.into_inner() << 12u32
                    | self.Zk.into_inner() << 10u32
                    | self.Zn.into_inner() << 6u32
                    | self.i2.into_inner() << 4u32
                    | 0b00u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod sutmopa_za_zzzi_b2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sutmopa_za_zzzi_b2x1 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub K: ::aarchmrs_types::BitValue<1>,
        pub Zk: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl sutmopa_za_zzzi_b2x1 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            K: ::aarchmrs_types::BitValue<1>,
            Zk: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<4>,
            i2: ::aarchmrs_types::BitValue<2>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                K,
                Zk,
                Zn,
                i2,
                ZAda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000011u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b100u32 << 13u32
                    | self.K.into_inner() << 12u32
                    | self.Zk.into_inner() << 10u32
                    | self.Zn.into_inner() << 6u32
                    | self.i2.into_inner() << 4u32
                    | 0b00u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod ustmopa_za_zzzi_b2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ustmopa_za_zzzi_b2x1 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub K: ::aarchmrs_types::BitValue<1>,
        pub Zk: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl ustmopa_za_zzzi_b2x1 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            K: ::aarchmrs_types::BitValue<1>,
            Zk: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<4>,
            i2: ::aarchmrs_types::BitValue<2>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                K,
                Zk,
                Zn,
                i2,
                ZAda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000001010u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b100u32 << 13u32
                    | self.K.into_inner() << 12u32
                    | self.Zk.into_inner() << 10u32
                    | self.Zn.into_inner() << 6u32
                    | self.i2.into_inner() << 4u32
                    | 0b00u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod utmopa_za_zzzi_b2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct utmopa_za_zzzi_b2x1 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub K: ::aarchmrs_types::BitValue<1>,
        pub Zk: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl utmopa_za_zzzi_b2x1 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            K: ::aarchmrs_types::BitValue<1>,
            Zk: ::aarchmrs_types::BitValue<2>,
            Zn: ::aarchmrs_types::BitValue<4>,
            i2: ::aarchmrs_types::BitValue<2>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                K,
                Zk,
                Zn,
                i2,
                ZAda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000001011u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | 0b100u32 << 13u32
                    | self.K.into_inner() << 12u32
                    | self.Zk.into_inner() << 10u32
                    | self.Zn.into_inner() << 6u32
                    | self.i2.into_inner() << 4u32
                    | 0b00u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
