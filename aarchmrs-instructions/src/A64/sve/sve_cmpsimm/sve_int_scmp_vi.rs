/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cmpge_p_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cmpge_p_p_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ne: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl cmpge_p_p_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ne: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm5: imm5.into(),
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
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b00u32 << 14u32
                    | u32::from(self.lt) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.ne) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod cmpeq_p_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cmpeq_p_p_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ne: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl cmpeq_p_p_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ne: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm5: imm5.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                ne: ne.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.ne) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod cmplt_p_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cmplt_p_p_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ne: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl cmplt_p_p_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ne: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm5: imm5.into(),
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
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b00u32 << 14u32
                    | u32::from(self.lt) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.ne) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod cmpgt_p_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cmpgt_p_p_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ne: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl cmpgt_p_p_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ne: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm5: imm5.into(),
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
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b00u32 << 14u32
                    | u32::from(self.lt) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.ne) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod cmpne_p_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cmpne_p_p_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ne: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl cmpne_p_p_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ne: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm5: imm5.into(),
                Pg: Pg.into(),
                Zn: Zn.into(),
                ne: ne.into(),
                Pd: Pd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b100u32 << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.ne) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
pub mod cmple_p_p_zi_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct cmple_p_p_zi_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub lt: ::aarchmrs_types::BitValue<1>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub ne: ::aarchmrs_types::BitValue<1>,
        pub Pd: ::aarchmrs_types::BitValue<4>,
    }
    impl cmple_p_p_zi_ {
        #[inline]
        pub fn new(
            size: impl Into<::aarchmrs_types::BitValue<2>>,
            imm5: impl Into<::aarchmrs_types::BitValue<5>>,
            lt: impl Into<::aarchmrs_types::BitValue<1>>,
            Pg: impl Into<::aarchmrs_types::BitValue<3>>,
            Zn: impl Into<::aarchmrs_types::BitValue<5>>,
            ne: impl Into<::aarchmrs_types::BitValue<1>>,
            Pd: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                size: size.into(),
                imm5: imm5.into(),
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
                0b00100101u32 << 24u32
                    | u32::from(self.size) << 22u32
                    | 0b0u32 << 21u32
                    | u32::from(self.imm5) << 16u32
                    | 0b00u32 << 14u32
                    | u32::from(self.lt) << 13u32
                    | u32::from(self.Pg) << 10u32
                    | u32::from(self.Zn) << 5u32
                    | u32::from(self.ne) << 4u32
                    | u32::from(self.Pd) << 0u32,
            )
        }
    }
}
