/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STRB_32_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STRB_32_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STRB_32_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDRB_32_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRB_32_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRB_32_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDRSB_64_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSB_64_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSB_64_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000100u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDRSB_32_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSB_32_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSB_32_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111000110u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STR_B_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_B_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_B_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDR_B_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_B_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_B_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STR_Q_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_Q_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_Q_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100100u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDR_Q_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_Q_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_Q_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00111100110u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STRH_32_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STRH_32_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STRH_32_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111000000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDRH_32_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRH_32_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRH_32_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111000010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDRSH_64_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSH_64_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSH_64_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111000100u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDRSH_32_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSH_32_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSH_32_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111000110u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STR_H_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_H_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_H_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111100000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDR_H_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_H_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_H_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01111100010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STR_32_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_32_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_32_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111000000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDR_32_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_32_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_32_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111000010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDRSW_64_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDRSW_64_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDRSW_64_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111000100u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STR_S_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_S_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_S_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111100000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDR_S_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_S_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_S_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10111100010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STR_64_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_64_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_64_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111000000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDR_64_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_64_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_64_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111000010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STR_D_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STR_D_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STR_D_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111100000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDR_D_ldst_immpre {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDR_D_ldst_immpre {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDR_D_ldst_immpre {
        #[inline]
        pub const fn new(
            imm9: ::aarchmrs_types::BitValue<9>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm9, Rn, Rt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11111100010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b11u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
