/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ADD_32_addsub_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADD_32_addsub_imm {
        pub sh: ::aarchmrs_types::BitValue<1>,
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADD_32_addsub_imm {
        #[inline]
        pub const fn new(
            sh: ::aarchmrs_types::BitValue<1>,
            imm12: ::aarchmrs_types::BitValue<12>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sh, imm12, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000100010u32 << 23u32
                    | self.sh.into_inner() << 22u32
                    | self.imm12.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod ADDS_32S_addsub_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADDS_32S_addsub_imm {
        pub sh: ::aarchmrs_types::BitValue<1>,
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADDS_32S_addsub_imm {
        #[inline]
        pub const fn new(
            sh: ::aarchmrs_types::BitValue<1>,
            imm12: ::aarchmrs_types::BitValue<12>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sh, imm12, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b001100010u32 << 23u32
                    | self.sh.into_inner() << 22u32
                    | self.imm12.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SUB_32_addsub_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUB_32_addsub_imm {
        pub sh: ::aarchmrs_types::BitValue<1>,
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUB_32_addsub_imm {
        #[inline]
        pub const fn new(
            sh: ::aarchmrs_types::BitValue<1>,
            imm12: ::aarchmrs_types::BitValue<12>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sh, imm12, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010100010u32 << 23u32
                    | self.sh.into_inner() << 22u32
                    | self.imm12.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SUBS_32S_addsub_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUBS_32S_addsub_imm {
        pub sh: ::aarchmrs_types::BitValue<1>,
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUBS_32S_addsub_imm {
        #[inline]
        pub const fn new(
            sh: ::aarchmrs_types::BitValue<1>,
            imm12: ::aarchmrs_types::BitValue<12>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sh, imm12, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b011100010u32 << 23u32
                    | self.sh.into_inner() << 22u32
                    | self.imm12.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod ADD_64_addsub_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADD_64_addsub_imm {
        pub sh: ::aarchmrs_types::BitValue<1>,
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADD_64_addsub_imm {
        #[inline]
        pub const fn new(
            sh: ::aarchmrs_types::BitValue<1>,
            imm12: ::aarchmrs_types::BitValue<12>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sh, imm12, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b100100010u32 << 23u32
                    | self.sh.into_inner() << 22u32
                    | self.imm12.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod ADDS_64S_addsub_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ADDS_64S_addsub_imm {
        pub sh: ::aarchmrs_types::BitValue<1>,
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ADDS_64S_addsub_imm {
        #[inline]
        pub const fn new(
            sh: ::aarchmrs_types::BitValue<1>,
            imm12: ::aarchmrs_types::BitValue<12>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sh, imm12, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b101100010u32 << 23u32
                    | self.sh.into_inner() << 22u32
                    | self.imm12.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SUB_64_addsub_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUB_64_addsub_imm {
        pub sh: ::aarchmrs_types::BitValue<1>,
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUB_64_addsub_imm {
        #[inline]
        pub const fn new(
            sh: ::aarchmrs_types::BitValue<1>,
            imm12: ::aarchmrs_types::BitValue<12>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sh, imm12, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110100010u32 << 23u32
                    | self.sh.into_inner() << 22u32
                    | self.imm12.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SUBS_64S_addsub_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SUBS_64S_addsub_imm {
        pub sh: ::aarchmrs_types::BitValue<1>,
        pub imm12: ::aarchmrs_types::BitValue<12>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SUBS_64S_addsub_imm {
        #[inline]
        pub const fn new(
            sh: ::aarchmrs_types::BitValue<1>,
            imm12: ::aarchmrs_types::BitValue<12>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sh, imm12, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b111100010u32 << 23u32
                    | self.sh.into_inner() << 22u32
                    | self.imm12.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
