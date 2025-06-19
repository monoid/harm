/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmla_za_zzi_s2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmla_za_zzi_s2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fmla_za_zzi_s2xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i2: i2.into(),
                Zn: Zn.into(),
                S: S.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010101u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b0u32 << 12u32
                    | u32::from(self.i2) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.S) << 4u32
                    | 0b0u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod fvdot_za_zzi_2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fvdot_za_zzi_2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fvdot_za_zzi_2xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i2: i2.into(),
                Zn: Zn.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010101u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b0u32 << 12u32
                    | u32::from(self.i2) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b001u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod bfvdot_za_zzi_2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfvdot_za_zzi_2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl bfvdot_za_zzi_2xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i2: i2.into(),
                Zn: Zn.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010101u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b0u32 << 12u32
                    | u32::from(self.i2) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b011u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod svdot_za32_zzi_2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct svdot_za32_zzi_2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl svdot_za32_zzi_2xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i2: i2.into(),
                Zn: Zn.into(),
                U: U.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010101u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b0u32 << 12u32
                    | u32::from(self.i2) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b1u32 << 5u32
                    | u32::from(self.U) << 4u32
                    | 0b0u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod fdot_za32_z8z8i_2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fdot_za32_z8z8i_2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fdot_za32_z8z8i_2xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i2: i2.into(),
                Zn: Zn.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010101u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b0u32 << 12u32
                    | u32::from(self.i2) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b111u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod sdot_za32_zzi_2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sdot_za32_zzi_2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl sdot_za32_zzi_2xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i2: i2.into(),
                Zn: Zn.into(),
                U: U.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010101u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b1u32 << 12u32
                    | u32::from(self.i2) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.U) << 4u32
                    | 0b0u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod fdot_za_zzi_2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fdot_za_zzi_2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fdot_za_zzi_2xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i2: i2.into(),
                Zn: Zn.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010101u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b1u32 << 12u32
                    | u32::from(self.i2) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b001u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod bfdot_za_zzi_2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct bfdot_za_zzi_2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl bfdot_za_zzi_2xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i2: i2.into(),
                Zn: Zn.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010101u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b1u32 << 12u32
                    | u32::from(self.i2) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b011u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod sdot_za_zzi_s2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sdot_za_zzi_s2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl sdot_za_zzi_s2xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i2: i2.into(),
                Zn: Zn.into(),
                U: U.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010101u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b1u32 << 12u32
                    | u32::from(self.i2) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b1u32 << 5u32
                    | u32::from(self.U) << 4u32
                    | 0b0u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod usdot_za_zzi_s2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct usdot_za_zzi_s2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl usdot_za_zzi_s2xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i2: i2.into(),
                Zn: Zn.into(),
                U: U.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010101u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b1u32 << 12u32
                    | u32::from(self.i2) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b1u32 << 5u32
                    | u32::from(self.U) << 4u32
                    | 0b1u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod fmls_za_zzi_s2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct fmls_za_zzi_s2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub S: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl fmls_za_zzi_s2xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            S: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i2: i2.into(),
                Zn: Zn.into(),
                S: S.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010101u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b0u32 << 12u32
                    | u32::from(self.i2) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.S) << 4u32
                    | 0b0u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod uvdot_za32_zzi_2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct uvdot_za32_zzi_2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl uvdot_za32_zzi_2xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i2: i2.into(),
                Zn: Zn.into(),
                U: U.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010101u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b0u32 << 12u32
                    | u32::from(self.i2) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b1u32 << 5u32
                    | u32::from(self.U) << 4u32
                    | 0b0u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod udot_za32_zzi_2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct udot_za32_zzi_2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl udot_za32_zzi_2xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i2: i2.into(),
                Zn: Zn.into(),
                U: U.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010101u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b1u32 << 12u32
                    | u32::from(self.i2) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b0u32 << 5u32
                    | u32::from(self.U) << 4u32
                    | 0b0u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod udot_za_zzi_s2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct udot_za_zzi_s2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl udot_za_zzi_s2xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i2: i2.into(),
                Zn: Zn.into(),
                U: U.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010101u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b1u32 << 12u32
                    | u32::from(self.i2) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b1u32 << 5u32
                    | u32::from(self.U) << 4u32
                    | 0b0u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
pub mod sudot_za_zzi_s2xi {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct sudot_za_zzi_s2xi {
        pub Zm: ::aarchmrs_types::BitValue<4>,
        pub Rv: ::aarchmrs_types::BitValue<2>,
        pub i2: ::aarchmrs_types::BitValue<2>,
        pub Zn: ::aarchmrs_types::BitValue<4>,
        pub U: ::aarchmrs_types::BitValue<1>,
        pub off3: ::aarchmrs_types::BitValue<3>,
    }
    impl sudot_za_zzi_s2xi {
        #[inline]
        pub fn new(
            Zm: impl Into<::aarchmrs_types::BitValue<4>>,
            Rv: impl Into<::aarchmrs_types::BitValue<2>>,
            i2: impl Into<::aarchmrs_types::BitValue<2>>,
            Zn: impl Into<::aarchmrs_types::BitValue<4>>,
            U: impl Into<::aarchmrs_types::BitValue<1>>,
            off3: impl Into<::aarchmrs_types::BitValue<3>>,
        ) -> Self {
            Self {
                Zm: Zm.into(),
                Rv: Rv.into(),
                i2: i2.into(),
                Zn: Zn.into(),
                U: U.into(),
                off3: off3.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000010101u32 << 20u32
                    | u32::from(self.Zm) << 16u32
                    | 0b0u32 << 15u32
                    | u32::from(self.Rv) << 13u32
                    | 0b1u32 << 12u32
                    | u32::from(self.i2) << 10u32
                    | u32::from(self.Zn) << 6u32
                    | 0b1u32 << 5u32
                    | u32::from(self.U) << 4u32
                    | 0b1u32 << 3u32
                    | u32::from(self.off3) << 0u32,
            )
        }
    }
}
