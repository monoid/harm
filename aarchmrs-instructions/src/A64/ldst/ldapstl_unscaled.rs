/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STLURB_32_ldapstl_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLURB_32_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLURB_32_ldapstl_unscaled {
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
                0b00011001000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDAPURB_32_ldapstl_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPURB_32_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPURB_32_ldapstl_unscaled {
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
                0b00011001010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDAPURSB_64_ldapstl_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPURSB_64_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPURSB_64_ldapstl_unscaled {
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
                0b00011001100u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDAPURSB_32_ldapstl_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPURSB_32_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPURSB_32_ldapstl_unscaled {
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
                0b00011001110u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STLURH_32_ldapstl_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLURH_32_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLURH_32_ldapstl_unscaled {
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
                0b01011001000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDAPURH_32_ldapstl_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPURH_32_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPURH_32_ldapstl_unscaled {
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
                0b01011001010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDAPURSH_64_ldapstl_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPURSH_64_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPURSH_64_ldapstl_unscaled {
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
                0b01011001100u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDAPURSH_32_ldapstl_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPURSH_32_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPURSH_32_ldapstl_unscaled {
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
                0b01011001110u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STLUR_32_ldapstl_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLUR_32_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLUR_32_ldapstl_unscaled {
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
                0b10011001000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDAPUR_32_ldapstl_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPUR_32_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPUR_32_ldapstl_unscaled {
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
                0b10011001010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDAPURSW_64_ldapstl_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPURSW_64_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPURSW_64_ldapstl_unscaled {
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
                0b10011001100u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod STLUR_64_ldapstl_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct STLUR_64_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl STLUR_64_ldapstl_unscaled {
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
                0b11011001000u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
pub mod LDAPUR_64_ldapstl_unscaled {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct LDAPUR_64_ldapstl_unscaled {
        pub imm9: ::aarchmrs_types::BitValue<9>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rt: ::aarchmrs_types::BitValue<5>,
    }
    impl LDAPUR_64_ldapstl_unscaled {
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
                0b11011001010u32 << 21u32
                    | self.imm9.into_inner() << 12u32
                    | 0b00u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rt.into_inner() << 0u32,
            )
        }
    }
}
