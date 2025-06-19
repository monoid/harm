/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CPYFP_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFP_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFP_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFPWT_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPWT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPWT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFPRT_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPRT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPRT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFPT_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFPWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFPWTWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPWTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPWTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFPRTWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPRTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPRTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFPTWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFPRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFPWTRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPWTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPWTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFPRTRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPRTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPRTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFPTRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFPN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFPWTN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPWTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPWTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFPRTN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPRTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPRTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFPTN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFPTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFPTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFM_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFM_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFM_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFMWT_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMWT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMWT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFMRT_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMRT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMRT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFMT_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFMWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFMWTWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMWTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMWTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFMRTWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMRTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMRTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFMTWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFMRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFMWTRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMWTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMWTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFMRTRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMRTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMRTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFMTRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFMN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFMWTN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMWTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMWTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFMRTN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMRTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMRTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFMTN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFMTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFMTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFE_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFE_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFE_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFEWT_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFEWT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFEWT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFERT_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFERT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFERT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFET_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFET_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFET_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFEWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFEWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFEWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFEWTWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFEWTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFEWTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFERTWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFERTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFERTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFETWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFETWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFETWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFERN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFERN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFERN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFEWTRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFEWTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFEWTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFERTRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFERTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFERTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFETRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFETRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFETRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFEN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFEN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFEN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFEWTN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFEWTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFEWTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFERTN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFERTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFERTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYFETN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYFETN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYFETN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETP_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETP_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETP_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETPT_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETPT_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETPT_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETPN_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETPN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETPN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETPTN_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETPTN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETPTN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETM_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETM_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETM_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETMT_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETMT_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETMT_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETMN_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETMN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETMN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETMTN_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETMTN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETMTN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETE_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETE_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETE_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETET_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETET_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETET_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETEN_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETEN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETEN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETETN_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETETN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETETN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011001110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYP_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYP_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYP_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYPWT_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPWT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPWT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYPRT_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPRT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPRT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYPT_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYPWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYPWTWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPWTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPWTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYPRTWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPRTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPRTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYPTWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYPRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYPWTRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPWTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPWTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYPRTRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPRTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPRTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYPTRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYPN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYPWTN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPWTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPWTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYPRTN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPRTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPRTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYPTN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYPTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYPTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101000u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYM_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYM_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYM_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYMWT_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMWT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMWT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYMRT_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMRT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMRT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYMT_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYMWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYMWTWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMWTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMWTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYMRTWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMRTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMRTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYMTWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYMRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYMWTRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMWTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMWTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYMRTRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMRTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMRTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYMTRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYMN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYMWTN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMWTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMWTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYMRTN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMRTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMRTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYMTN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYMTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYMTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101010u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYE_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYE_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYE_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYEWT_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYEWT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYEWT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYERT_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYERT_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYERT_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYET_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYET_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYET_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYEWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYEWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYEWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYEWTWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYEWTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYEWTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYERTWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYERTWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYERTWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYETWN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYETWN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYETWN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYERN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYERN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYERN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYEWTRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYEWTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYEWTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYERTRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYERTRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYERTRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYETRN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYETRN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYETRN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYEN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYEN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYEN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYEWTN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYEWTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYEWTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b110101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYERTN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYERTN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYERTN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod CPYETN_CPY_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CPYETN_CPY_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CPYETN_CPY_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101100u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b111101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETGP_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGP_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGP_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETGPT_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGPT_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGPT_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b000101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETGPN_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGPN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGPN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETGPTN_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGPTN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGPTN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b001101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETGM_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGM_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGM_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETGMT_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGMT_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGMT_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b010101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETGMN_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGMN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGMN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETGMTN_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGMTN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGMTN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b011101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETGE_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGE_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGE_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETGET_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGET_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGET_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b100101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETGEN_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGEN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGEN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101001u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
pub mod SETGETN_SET_memcms {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SETGETN_SET_memcms {
        pub sz: ::aarchmrs_types::BitValue<2>,
        pub Rs: ::aarchmrs_types::BitValue<5>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl SETGETN_SET_memcms {
        #[inline]
        pub const fn new(
            sz: ::aarchmrs_types::BitValue<2>,
            Rs: ::aarchmrs_types::BitValue<5>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rd: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { sz, Rs, Rn, Rd }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                self.sz.into_inner() << 30u32
                    | 0b011101110u32 << 21u32
                    | self.Rs.into_inner() << 16u32
                    | 0b101101u32 << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rd.into_inner() << 0u32,
            )
        }
    }
}
