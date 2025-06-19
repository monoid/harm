/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod FMOV_S_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMOV_S_floatdp1 {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMOV_S_floatdp1 {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100010000u32 << 17u32
                    | self.opc.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FABS_S_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FABS_S_floatdp1 {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FABS_S_floatdp1 {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100010000u32 << 17u32
                    | self.opc.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FNEG_S_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FNEG_S_floatdp1 {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FNEG_S_floatdp1 {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100010000u32 << 17u32
                    | self.opc.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FSQRT_S_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FSQRT_S_floatdp1 {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FSQRT_S_floatdp1 {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100010000u32 << 17u32
                    | self.opc.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FCVT_DS_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVT_DS_floatdp1 {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVT_DS_floatdp1 {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100010001u32 << 17u32
                    | self.opc.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FCVT_HS_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVT_HS_floatdp1 {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVT_HS_floatdp1 {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100010001u32 << 17u32
                    | self.opc.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTN_S_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTN_S_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTN_S_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110001001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTP_S_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTP_S_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTP_S_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110001001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTM_S_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTM_S_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTM_S_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110001001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTZ_S_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTZ_S_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTZ_S_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110001001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTA_S_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTA_S_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTA_S_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110001001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTX_S_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTX_S_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTX_S_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110001001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTI_S_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTI_S_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTI_S_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110001001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINT32Z_S_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINT32Z_S_floatdp1 {
        pub op: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINT32Z_S_floatdp1 {
        #[inline]
        pub const fn new(
            op: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { op, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100010100u32 << 17u32
                    | self.op.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINT32X_S_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINT32X_S_floatdp1 {
        pub op: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINT32X_S_floatdp1 {
        #[inline]
        pub const fn new(
            op: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { op, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100010100u32 << 17u32
                    | self.op.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINT64Z_S_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINT64Z_S_floatdp1 {
        pub op: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINT64Z_S_floatdp1 {
        #[inline]
        pub const fn new(
            op: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { op, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100010100u32 << 17u32
                    | self.op.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINT64X_S_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINT64X_S_floatdp1 {
        pub op: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINT64X_S_floatdp1 {
        #[inline]
        pub const fn new(
            op: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { op, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100010100u32 << 17u32
                    | self.op.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FMOV_D_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMOV_D_floatdp1 {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMOV_D_floatdp1 {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100110000u32 << 17u32
                    | self.opc.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FABS_D_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FABS_D_floatdp1 {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FABS_D_floatdp1 {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100110000u32 << 17u32
                    | self.opc.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FNEG_D_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FNEG_D_floatdp1 {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FNEG_D_floatdp1 {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100110000u32 << 17u32
                    | self.opc.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FSQRT_D_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FSQRT_D_floatdp1 {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FSQRT_D_floatdp1 {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100110000u32 << 17u32
                    | self.opc.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FCVT_SD_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVT_SD_floatdp1 {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVT_SD_floatdp1 {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100110001u32 << 17u32
                    | self.opc.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod BFCVT_BS_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BFCVT_BS_floatdp1 {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl BFCVT_BS_floatdp1 {
        #[inline]
        pub const fn new(
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0001111001100011010000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FCVT_HD_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVT_HD_floatdp1 {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVT_HD_floatdp1 {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100110001u32 << 17u32
                    | self.opc.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTN_D_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTN_D_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTN_D_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110011001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTP_D_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTP_D_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTP_D_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110011001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTM_D_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTM_D_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTM_D_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110011001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTZ_D_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTZ_D_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTZ_D_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110011001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTA_D_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTA_D_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTA_D_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110011001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTX_D_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTX_D_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTX_D_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110011001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTI_D_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTI_D_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTI_D_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110011001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINT32Z_D_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINT32Z_D_floatdp1 {
        pub op: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINT32Z_D_floatdp1 {
        #[inline]
        pub const fn new(
            op: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { op, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100110100u32 << 17u32
                    | self.op.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINT32X_D_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINT32X_D_floatdp1 {
        pub op: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINT32X_D_floatdp1 {
        #[inline]
        pub const fn new(
            op: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { op, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100110100u32 << 17u32
                    | self.op.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINT64Z_D_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINT64Z_D_floatdp1 {
        pub op: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINT64Z_D_floatdp1 {
        #[inline]
        pub const fn new(
            op: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { op, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100110100u32 << 17u32
                    | self.op.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINT64X_D_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINT64X_D_floatdp1 {
        pub op: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINT64X_D_floatdp1 {
        #[inline]
        pub const fn new(
            op: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { op, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111100110100u32 << 17u32
                    | self.op.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FMOV_H_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FMOV_H_floatdp1 {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FMOV_H_floatdp1 {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111101110000u32 << 17u32
                    | self.opc.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FABS_H_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FABS_H_floatdp1 {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FABS_H_floatdp1 {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111101110000u32 << 17u32
                    | self.opc.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FNEG_H_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FNEG_H_floatdp1 {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FNEG_H_floatdp1 {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111101110000u32 << 17u32
                    | self.opc.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FSQRT_H_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FSQRT_H_floatdp1 {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FSQRT_H_floatdp1 {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111101110000u32 << 17u32
                    | self.opc.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FCVT_SH_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVT_SH_floatdp1 {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVT_SH_floatdp1 {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111101110001u32 << 17u32
                    | self.opc.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FCVT_DH_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FCVT_DH_floatdp1 {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FCVT_DH_floatdp1 {
        #[inline]
        pub const fn new(
            opc: ::aarchmrs_types::BitValue<2>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { opc, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b000111101110001u32 << 17u32
                    | self.opc.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTN_H_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTN_H_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTN_H_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110111001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTP_H_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTP_H_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTP_H_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110111001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTM_H_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTM_H_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTM_H_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110111001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTZ_H_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTZ_H_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTZ_H_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110111001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTA_H_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTA_H_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTA_H_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110111001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTX_H_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTX_H_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTX_H_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110111001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod FRINTI_H_floatdp1 {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FRINTI_H_floatdp1 {
        pub rmode: ::aarchmrs_types::BitValue<3>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl FRINTI_H_floatdp1 {
        #[inline]
        pub const fn new(
            rmode: ::aarchmrs_types::BitValue<3>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { rmode, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b00011110111001u32 << 18u32
                    | self.rmode.into_inner() << 15u32
                    | 0b10000u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
