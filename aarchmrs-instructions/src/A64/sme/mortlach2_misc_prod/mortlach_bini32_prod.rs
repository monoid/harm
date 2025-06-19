/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod bmopa_za_pp_zz_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bmopa_za_pp_zz_32 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl bmopa_za_pp_zz_32 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            Pm: ::aarchmrs_types::BitValue<3>,
            Pn: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                Pm,
                Pn,
                Zn,
                ZAda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000100u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | self.Pm.into_inner() << 13u32
                    | self.Pn.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b010u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
pub mod bmops_za_pp_zz_32 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bmops_za_pp_zz_32 {
        pub Zm: ::aarchmrs_types::BitValue<5>,
        pub Pm: ::aarchmrs_types::BitValue<3>,
        pub Pn: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ZAda: ::aarchmrs_types::BitValue<2>,
    }
    impl bmops_za_pp_zz_32 {
        #[inline]
        pub const fn new(
            Zm: ::aarchmrs_types::BitValue<5>,
            Pm: ::aarchmrs_types::BitValue<3>,
            Pn: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            ZAda: ::aarchmrs_types::BitValue<2>,
        ) -> Self {
            Self {
                Zm,
                Pm,
                Pn,
                Zn,
                ZAda,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10000000100u32 << 21u32
                    | self.Zm.into_inner() << 16u32
                    | self.Pm.into_inner() << 13u32
                    | self.Pn.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | 0b110u32 << 2u32
                    | self.ZAda.into_inner() << 0u32,
            )
        }
    }
}
