/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CBGT_32_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBGT_32_imm {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBGT_32_imm {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm6, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110101000u32 << 21u32
                    | self.imm6.into_inner() << 15u32
                    | 0b0u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBLT_32_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBLT_32_imm {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBLT_32_imm {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm6, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110101001u32 << 21u32
                    | self.imm6.into_inner() << 15u32
                    | 0b0u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBHI_32_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBHI_32_imm {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBHI_32_imm {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm6, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110101010u32 << 21u32
                    | self.imm6.into_inner() << 15u32
                    | 0b0u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBLO_32_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBLO_32_imm {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBLO_32_imm {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm6, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110101011u32 << 21u32
                    | self.imm6.into_inner() << 15u32
                    | 0b0u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBEQ_32_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBEQ_32_imm {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBEQ_32_imm {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm6, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110101110u32 << 21u32
                    | self.imm6.into_inner() << 15u32
                    | 0b0u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBNE_32_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBNE_32_imm {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBNE_32_imm {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm6, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01110101111u32 << 21u32
                    | self.imm6.into_inner() << 15u32
                    | 0b0u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBGT_64_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBGT_64_imm {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBGT_64_imm {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm6, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11110101000u32 << 21u32
                    | self.imm6.into_inner() << 15u32
                    | 0b0u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBLT_64_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBLT_64_imm {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBLT_64_imm {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm6, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11110101001u32 << 21u32
                    | self.imm6.into_inner() << 15u32
                    | 0b0u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBHI_64_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBHI_64_imm {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBHI_64_imm {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm6, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11110101010u32 << 21u32
                    | self.imm6.into_inner() << 15u32
                    | 0b0u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBLO_64_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBLO_64_imm {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBLO_64_imm {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm6, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11110101011u32 << 21u32
                    | self.imm6.into_inner() << 15u32
                    | 0b0u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBEQ_64_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBEQ_64_imm {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBEQ_64_imm {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm6, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11110101110u32 << 21u32
                    | self.imm6.into_inner() << 15u32
                    | 0b0u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod CBNE_64_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CBNE_64_imm {
        pub imm6: ::aarchmrs_types::BitValue<6>,
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl CBNE_64_imm {
        #[inline]
        pub const fn new(
            imm6: ::aarchmrs_types::BitValue<6>,
            imm9: ::aarchmrs_types::BitValue<9>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm6, imm9, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11110101111u32 << 21u32
                    | self.imm6.into_inner() << 15u32
                    | 0b0u32 << 14u32
                    | self.imm9.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
