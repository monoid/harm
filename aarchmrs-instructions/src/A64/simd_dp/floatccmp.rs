/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FCCMP_S_floatccmp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCCMP_S_floatccmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl FCCMP_S_floatccmp {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                Rn: Rn.into(),
                nzcv: nzcv.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.nzcv) << 0u32,
            )
        }
    }
}
pub mod FCCMPE_S_floatccmp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCCMPE_S_floatccmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl FCCMPE_S_floatccmp {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                Rn: Rn.into(),
                nzcv: nzcv.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.nzcv) << 0u32,
            )
        }
    }
}
pub mod FCCMP_D_floatccmp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCCMP_D_floatccmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl FCCMP_D_floatccmp {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                Rn: Rn.into(),
                nzcv: nzcv.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110011u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.nzcv) << 0u32,
            )
        }
    }
}
pub mod FCCMPE_D_floatccmp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCCMPE_D_floatccmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl FCCMPE_D_floatccmp {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                Rn: Rn.into(),
                nzcv: nzcv.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110011u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.nzcv) << 0u32,
            )
        }
    }
}
pub mod FCCMP_H_floatccmp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCCMP_H_floatccmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl FCCMP_H_floatccmp {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                Rn: Rn.into(),
                nzcv: nzcv.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110111u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b0u32 << 4u32
                    | u32::from(self.nzcv) << 0u32,
            )
        }
    }
}
pub mod FCCMPE_H_floatccmp {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCCMPE_H_floatccmp {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub cond: ::aarchmrs_types::BitValue<4>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub nzcv: ::aarchmrs_types::BitValue<4>,
    }
    impl FCCMPE_H_floatccmp {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            cond: impl Into<::aarchmrs_types::BitValue<4>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            nzcv: impl Into<::aarchmrs_types::BitValue<4>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                cond: cond.into(),
                Rn: Rn.into(),
                nzcv: nzcv.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110111u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.cond) << 12u32
                    | 0b01u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b1u32 << 4u32
                    | u32::from(self.nzcv) << 0u32,
            )
        }
    }
}
