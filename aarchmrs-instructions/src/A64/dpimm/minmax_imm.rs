/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SMAX_32_minmax_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SMAX_32_minmax_imm {
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SMAX_32_minmax_imm {
        #[inline]
        pub const fn new(
            imm8: ::aarchmrs_types::BitValue<8>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm8, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00010001110000u32 << 18u32
                    | self.imm8.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod UMAX_32U_minmax_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UMAX_32U_minmax_imm {
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UMAX_32U_minmax_imm {
        #[inline]
        pub const fn new(
            imm8: ::aarchmrs_types::BitValue<8>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm8, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00010001110001u32 << 18u32
                    | self.imm8.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SMIN_32_minmax_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SMIN_32_minmax_imm {
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SMIN_32_minmax_imm {
        #[inline]
        pub const fn new(
            imm8: ::aarchmrs_types::BitValue<8>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm8, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00010001110010u32 << 18u32
                    | self.imm8.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod UMIN_32U_minmax_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UMIN_32U_minmax_imm {
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UMIN_32U_minmax_imm {
        #[inline]
        pub const fn new(
            imm8: ::aarchmrs_types::BitValue<8>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm8, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00010001110011u32 << 18u32
                    | self.imm8.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SMAX_64_minmax_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SMAX_64_minmax_imm {
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SMAX_64_minmax_imm {
        #[inline]
        pub const fn new(
            imm8: ::aarchmrs_types::BitValue<8>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm8, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10010001110000u32 << 18u32
                    | self.imm8.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod UMAX_64U_minmax_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UMAX_64U_minmax_imm {
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UMAX_64U_minmax_imm {
        #[inline]
        pub const fn new(
            imm8: ::aarchmrs_types::BitValue<8>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm8, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10010001110001u32 << 18u32
                    | self.imm8.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SMIN_64_minmax_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SMIN_64_minmax_imm {
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SMIN_64_minmax_imm {
        #[inline]
        pub const fn new(
            imm8: ::aarchmrs_types::BitValue<8>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm8, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10010001110010u32 << 18u32
                    | self.imm8.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod UMIN_64U_minmax_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UMIN_64U_minmax_imm {
        pub imm8: ::aarchmrs_types::BitValue<8>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl UMIN_64U_minmax_imm {
        #[inline]
        pub const fn new(
            imm8: ::aarchmrs_types::BitValue<8>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm8, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b10010001110011u32 << 18u32
                    | self.imm8.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
