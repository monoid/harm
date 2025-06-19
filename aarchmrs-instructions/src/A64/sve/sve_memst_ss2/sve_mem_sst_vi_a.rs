/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod st1b_z_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1b_z_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st1b_z_p_ai_d {
        #[inline]
        pub const fn new(
            imm5: ::aarchmrs_types::BitValue<5>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm5, Pg, Zn, Zt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100100010u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod st1h_z_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1h_z_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st1h_z_p_ai_d {
        #[inline]
        pub const fn new(
            imm5: ::aarchmrs_types::BitValue<5>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm5, Pg, Zn, Zt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100100110u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod st1w_z_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1w_z_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st1w_z_p_ai_d {
        #[inline]
        pub const fn new(
            imm5: ::aarchmrs_types::BitValue<5>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm5, Pg, Zn, Zt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100101010u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
pub mod st1d_z_p_ai_d {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct st1d_z_p_ai_d {
        pub imm5: ::aarchmrs_types::BitValue<5>,
        pub Pg: ::aarchmrs_types::BitValue<3>,
        pub Zn: ::aarchmrs_types::BitValue<5>,
        pub Zt: ::aarchmrs_types::BitValue<5>,
    }
    impl st1d_z_p_ai_d {
        #[inline]
        pub const fn new(
            imm5: ::aarchmrs_types::BitValue<5>,
            Pg: ::aarchmrs_types::BitValue<3>,
            Zn: ::aarchmrs_types::BitValue<5>,
            Zt: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { imm5, Pg, Zn, Zt }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11100101110u32 << 21u32
                    | self.imm5.into_inner() << 16u32
                    | 0b101u32 << 13u32
                    | self.Pg.into_inner() << 10u32
                    | self.Zn.into_inner() << 5u32
                    | self.Zt.into_inner() << 0u32,
            )
        }
    }
}
