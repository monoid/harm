/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ftmopa_za32_z8z8zi_b2x1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ftmopa_za32_z8z8zi_b2x1 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub K: ::aarchmrs_types::BitValue<1>,
        pub Zk: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl ftmopa_za32_z8z8zi_b2x1 {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            K: impl Into<::aarchmrs_types::BitValue<1>>,
            Zk: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            ZAda: impl Into<::aarchmrs_types::BitValue<2>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                K: K.into(),
                Zk: Zk.into(),
                Zn: Zn.into(),
                i2: i2.into(),
                ZAda: ZAda.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000011u32 << 21u32
                    | u32::from(self.Zm) << 16u32
                    | 0b000u32 << 13u32
                    | u32::from(self.K) << 12u32
                    | u32::from(self.Zk) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | u32::from(self.i2) << 4u32
                    | 0b00u32 << 2u32
                    | u32::from(self.ZAda) << 0u32,
            )
        }
    }
}
