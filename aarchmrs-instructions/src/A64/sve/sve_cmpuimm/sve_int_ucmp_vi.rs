/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cmphs_p_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cmphs_p_p_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ne: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl cmphs_p_p_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm7: impl Into<::aarchmrs_types::BitValue<7>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ne: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm7: imm7.into(),
                lt: lt.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                ne: ne.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.imm7) << 14u32
                    | u32::from(self.lt) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.ne) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod cmphi_p_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cmphi_p_p_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ne: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl cmphi_p_p_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm7: impl Into<::aarchmrs_types::BitValue<7>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ne: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm7: imm7.into(),
                lt: lt.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                ne: ne.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.imm7) << 14u32
                    | u32::from(self.lt) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.ne) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod cmplo_p_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cmplo_p_p_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ne: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl cmplo_p_p_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm7: impl Into<::aarchmrs_types::BitValue<7>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ne: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm7: imm7.into(),
                lt: lt.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                ne: ne.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.imm7) << 14u32
                    | u32::from(self.lt) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.ne) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod cmpls_p_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cmpls_p_p_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm7: ::aarchmrs_types::BitValue<7>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ne: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl cmpls_p_p_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm7: impl Into<::aarchmrs_types::BitValue<7>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ne: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm7: imm7.into(),
                lt: lt.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                ne: ne.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100100u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b1u32 << 21u32
                    | u32::from(self.imm7) << 14u32
                    | u32::from(self.lt) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.ne) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
