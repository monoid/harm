/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CBBGT_8_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBBGT_8_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBBGT_8_regs {
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
                    | 0b10u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBBGE_8_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBBGE_8_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBBGE_8_regs {
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
                    | 0b10u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBBHI_8_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBBHI_8_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBBHI_8_regs {
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
                    | 0b10u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBBHS_8_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBBHS_8_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBBHS_8_regs {
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
                    | 0b10u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBBEQ_8_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBBEQ_8_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBBEQ_8_regs {
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
                    | 0b10u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBBNE_8_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBBNE_8_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBBNE_8_regs {
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
                    | 0b10u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBHGT_16_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBHGT_16_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBHGT_16_regs {
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
                    | 0b11u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBHGE_16_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBHGE_16_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBHGE_16_regs {
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
                    | 0b11u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBHHI_16_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBHHI_16_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBHHI_16_regs {
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
                    | 0b11u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBHHS_16_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBHHS_16_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBHHS_16_regs {
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
                    | 0b11u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBHEQ_16_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBHEQ_16_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBHEQ_16_regs {
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
                    | 0b11u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBHNE_16_regs {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBHNE_16_regs {
        pub Rm: ::aarchmrs_types::BitValue<5>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBHNE_16_regs {
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
                    | 0b11u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
