/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod incb_r_rs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct incb_r_rs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl incb_r_rs_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b11u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111000u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod decb_r_rs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct decb_r_rs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl decb_r_rs_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b11u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod inch_r_rs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct inch_r_rs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl inch_r_rs_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b11u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111000u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod dech_r_rs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct dech_r_rs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl dech_r_rs_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b11u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod incw_r_rs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct incw_r_rs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl incw_r_rs_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b11u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111000u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod decw_r_rs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct decw_r_rs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl decw_r_rs_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b11u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod incd_r_rs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct incd_r_rs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl incd_r_rs_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b11u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111000u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
            )
        }
    }
}
pub mod decd_r_rs_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct decd_r_rs_ {
        pub size: ::aarchmrs_types::BitValue<2>,
        pub imm4: ::aarchmrs_types::BitValue<4>,
        pub pattern: ::aarchmrs_types::BitValue<5>,
        pub Rdn: ::aarchmrs_types::BitValue<5>,
    }
    impl decd_r_rs_ {
        #[inline]
        pub const fn new(
            size: ::aarchmrs_types::BitValue<2>,
            imm4: ::aarchmrs_types::BitValue<4>,
            pattern: ::aarchmrs_types::BitValue<5>,
            Rdn: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                size,
                imm4,
                pattern,
                Rdn,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00000100u32 << 24u32
                    | self.size.into_inner() << 22u32
                    | 0b11u32 << 20u32
                    | self.imm4.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.pattern.into_inner() << 5u32
                    | self.Rdn.into_inner() << 0u32,
            )
        }
    }
}
