/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CBGT_32_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBGT_32_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBGT_32_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110100000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBGE_32_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBGE_32_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBGE_32_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110100001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBHI_32_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBHI_32_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBHI_32_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110100010u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBHS_32_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBHS_32_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBHS_32_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110100011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBEQ_32_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBEQ_32_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBEQ_32_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110100110u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBNE_32_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBNE_32_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBNE_32_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110100111u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBGT_64_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBGT_64_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBGT_64_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11110100000u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBGE_64_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBGE_64_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBGE_64_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11110100001u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBHI_64_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBHI_64_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBHI_64_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11110100010u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBHS_64_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBHS_64_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBHS_64_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11110100011u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBEQ_64_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBEQ_64_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBEQ_64_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11110100110u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBNE_64_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBNE_64_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBNE_64_regs {
        #[inline]
        pub const fn new(
            Rm: ::aarchmrs_types::BitValue<5>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rm, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11110100111u32 << 21u32
                    | self.Rm.into_inner() << 16u32
                    | 0b00u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
