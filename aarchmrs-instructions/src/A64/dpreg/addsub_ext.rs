/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADD_32_addsub_ext {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADD_32_addsub_ext {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADD_32_addsub_ext {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                imm3: imm3.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00001011001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.imm3) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ADDS_32S_addsub_ext {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADDS_32S_addsub_ext {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADDS_32S_addsub_ext {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                imm3: imm3.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00101011001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.imm3) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SUB_32_addsub_ext {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUB_32_addsub_ext {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUB_32_addsub_ext {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                imm3: imm3.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01001011001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.imm3) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SUBS_32S_addsub_ext {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUBS_32S_addsub_ext {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUBS_32S_addsub_ext {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                imm3: imm3.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01101011001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.imm3) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ADD_64_addsub_ext {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADD_64_addsub_ext {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADD_64_addsub_ext {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                imm3: imm3.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10001011001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.imm3) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ADDS_64S_addsub_ext {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADDS_64S_addsub_ext {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADDS_64S_addsub_ext {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                imm3: imm3.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10101011001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.imm3) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SUB_64_addsub_ext {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUB_64_addsub_ext {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUB_64_addsub_ext {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                imm3: imm3.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11001011001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.imm3) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod SUBS_64S_addsub_ext {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUBS_64S_addsub_ext {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub option: ::aarchmrs_types::BitValue<3>,
        pub imm3: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUBS_64S_addsub_ext {
        #[inline]
        pub fn new(
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
            option: impl Into<::aarchmrs_types::BitValue<3>>,
            imm3: impl Into<::aarchmrs_types::BitValue<3>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rm: Rm.into(),
                option: option.into(),
                imm3: imm3.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11101011001u32 << 21u32
                    | u32::from(self.Rm) << 16u32
                    | u32::from(self.option) << 13u32
                    | u32::from(self.imm3) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
