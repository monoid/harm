/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod splice_z_p_zz_des {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct splice_z_p_zz_des {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub Pv: ::aarchmrs_types::BitValue<3>,
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Zdn: ::aarchmrs_types::BitValue<5>,
    }
    impl splice_z_p_zz_des {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            Pv: impl Into<::aarchmrs_types::BitValue<3>>,
            Zm: impl Into<::aarchmrs_types::BitValue<5>>,
            Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                size: size.into(),
                Pv: Pv.into(),
                Zm: Zm.into(),
                Zdn: Zdn.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b101100100u32 << 13u32
                    | u32::from(self.Pv) << 10u32
                    | u32::from(self.Zm) << 5u32
                    | u32::from(self.Zdn) << 0u32,
            )
        }
    }
}
