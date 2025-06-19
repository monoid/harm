/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod BR_64_branch_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BR_64_branch_reg {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<2>,
        pub A: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
    }
    impl BR_64_branch_reg {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<2>>,
            A: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                op: op.into(),
                A: A.into(),
                M: M.into(),
                Rn: Rn.into(),
                Rm: Rm.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | u32::from(self.Z) << 24u32
                    | 0b0u32 << 23u32
                    | u32::from(self.op) << 21u32
                    | 0b111110000u32 << 12u32
                    | u32::from(self.A) << 11u32
                    | u32::from(self.M) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rm) << 0u32,
            )
        }
    }
}
pub mod BRAAZ_64_branch_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BRAAZ_64_branch_reg {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<2>,
        pub A: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
    }
    impl BRAAZ_64_branch_reg {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<2>>,
            A: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                op: op.into(),
                A: A.into(),
                M: M.into(),
                Rn: Rn.into(),
                Rm: Rm.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | u32::from(self.Z) << 24u32
                    | 0b0u32 << 23u32
                    | u32::from(self.op) << 21u32
                    | 0b111110000u32 << 12u32
                    | u32::from(self.A) << 11u32
                    | u32::from(self.M) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rm) << 0u32,
            )
        }
    }
}
pub mod BRABZ_64_branch_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BRABZ_64_branch_reg {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<2>,
        pub A: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
    }
    impl BRABZ_64_branch_reg {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<2>>,
            A: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                op: op.into(),
                A: A.into(),
                M: M.into(),
                Rn: Rn.into(),
                Rm: Rm.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | u32::from(self.Z) << 24u32
                    | 0b0u32 << 23u32
                    | u32::from(self.op) << 21u32
                    | 0b111110000u32 << 12u32
                    | u32::from(self.A) << 11u32
                    | u32::from(self.M) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rm) << 0u32,
            )
        }
    }
}
pub mod BLR_64_branch_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BLR_64_branch_reg {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<2>,
        pub A: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
    }
    impl BLR_64_branch_reg {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<2>>,
            A: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                op: op.into(),
                A: A.into(),
                M: M.into(),
                Rn: Rn.into(),
                Rm: Rm.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | u32::from(self.Z) << 24u32
                    | 0b0u32 << 23u32
                    | u32::from(self.op) << 21u32
                    | 0b111110000u32 << 12u32
                    | u32::from(self.A) << 11u32
                    | u32::from(self.M) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rm) << 0u32,
            )
        }
    }
}
pub mod BLRAAZ_64_branch_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BLRAAZ_64_branch_reg {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<2>,
        pub A: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
    }
    impl BLRAAZ_64_branch_reg {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<2>>,
            A: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                op: op.into(),
                A: A.into(),
                M: M.into(),
                Rn: Rn.into(),
                Rm: Rm.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | u32::from(self.Z) << 24u32
                    | 0b0u32 << 23u32
                    | u32::from(self.op) << 21u32
                    | 0b111110000u32 << 12u32
                    | u32::from(self.A) << 11u32
                    | u32::from(self.M) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rm) << 0u32,
            )
        }
    }
}
pub mod BLRABZ_64_branch_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BLRABZ_64_branch_reg {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<2>,
        pub A: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
    }
    impl BLRABZ_64_branch_reg {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<2>>,
            A: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                op: op.into(),
                A: A.into(),
                M: M.into(),
                Rn: Rn.into(),
                Rm: Rm.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | u32::from(self.Z) << 24u32
                    | 0b0u32 << 23u32
                    | u32::from(self.op) << 21u32
                    | 0b111110000u32 << 12u32
                    | u32::from(self.A) << 11u32
                    | u32::from(self.M) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rm) << 0u32,
            )
        }
    }
}
pub mod RET_64R_branch_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RET_64R_branch_reg {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<2>,
        pub A: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
    }
    impl RET_64R_branch_reg {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<2>>,
            A: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                op: op.into(),
                A: A.into(),
                M: M.into(),
                Rn: Rn.into(),
                Rm: Rm.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | u32::from(self.Z) << 24u32
                    | 0b0u32 << 23u32
                    | u32::from(self.op) << 21u32
                    | 0b111110000u32 << 12u32
                    | u32::from(self.A) << 11u32
                    | u32::from(self.M) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rm) << 0u32,
            )
        }
    }
}
pub mod RETAASPPCR_64M_branch_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RETAASPPCR_64M_branch_reg {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
    }
    impl RETAASPPCR_64M_branch_reg {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                M: M.into(),
                Rm: Rm.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110101100101111100001u32 << 11u32
                    | u32::from(self.M) << 10u32
                    | 0b11111u32 << 5u32
                    | u32::from(self.Rm) << 0u32,
            )
        }
    }
}
pub mod RETAA_64E_branch_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RETAA_64E_branch_reg {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<2>,
        pub A: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
    }
    impl RETAA_64E_branch_reg {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<2>>,
            A: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                op: op.into(),
                A: A.into(),
                M: M.into(),
                Rm: Rm.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | u32::from(self.Z) << 24u32
                    | 0b0u32 << 23u32
                    | u32::from(self.op) << 21u32
                    | 0b111110000u32 << 12u32
                    | u32::from(self.A) << 11u32
                    | u32::from(self.M) << 10u32
                    | 0b11111u32 << 5u32
                    | u32::from(self.Rm) << 0u32,
            )
        }
    }
}
pub mod RETABSPPCR_64M_branch_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RETABSPPCR_64M_branch_reg {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
    }
    impl RETABSPPCR_64M_branch_reg {
        #[inline]
        pub fn new(
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                M: M.into(),
                Rm: Rm.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110101100101111100001u32 << 11u32
                    | u32::from(self.M) << 10u32
                    | 0b11111u32 << 5u32
                    | u32::from(self.Rm) << 0u32,
            )
        }
    }
}
pub mod RETAB_64E_branch_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RETAB_64E_branch_reg {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<2>,
        pub A: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
    }
    impl RETAB_64E_branch_reg {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<2>>,
            A: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                op: op.into(),
                A: A.into(),
                M: M.into(),
                Rm: Rm.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | u32::from(self.Z) << 24u32
                    | 0b0u32 << 23u32
                    | u32::from(self.op) << 21u32
                    | 0b111110000u32 << 12u32
                    | u32::from(self.A) << 11u32
                    | u32::from(self.M) << 10u32
                    | 0b11111u32 << 5u32
                    | u32::from(self.Rm) << 0u32,
            )
        }
    }
}
pub mod ERET_64E_branch_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ERET_64E_branch_reg {
        pub A: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
    }
    impl ERET_64E_branch_reg {
        #[inline]
        pub fn new(
            A: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                A: A.into(),
                M: M.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010110100111110000u32 << 12u32
                    | u32::from(self.A) << 11u32
                    | u32::from(self.M) << 10u32
                    | 0b1111100000u32 << 0u32,
            )
        }
    }
}
pub mod ERETAA_64E_branch_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ERETAA_64E_branch_reg {
        pub A: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
    }
    impl ERETAA_64E_branch_reg {
        #[inline]
        pub fn new(
            A: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                A: A.into(),
                M: M.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010110100111110000u32 << 12u32
                    | u32::from(self.A) << 11u32
                    | u32::from(self.M) << 10u32
                    | 0b1111111111u32 << 0u32,
            )
        }
    }
}
pub mod ERETAB_64E_branch_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ERETAB_64E_branch_reg {
        pub A: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
    }
    impl ERETAB_64E_branch_reg {
        #[inline]
        pub fn new(
            A: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
        ) -> Self {
            Self {
                A: A.into(),
                M: M.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010110100111110000u32 << 12u32
                    | u32::from(self.A) << 11u32
                    | u32::from(self.M) << 10u32
                    | 0b1111111111u32 << 0u32,
            )
        }
    }
}
pub mod DRPS_64E_branch_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct DRPS_64E_branch_reg {}
    impl DRPS_64E_branch_reg {
        #[inline]
        pub fn new() -> Self {
            Self {}
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010110101111110000001111100000u32 << 0u32,
            )
        }
    }
}
pub mod BRAA_64P_branch_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BRAA_64P_branch_reg {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<2>,
        pub A: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
    }
    impl BRAA_64P_branch_reg {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<2>>,
            A: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                op: op.into(),
                A: A.into(),
                M: M.into(),
                Rn: Rn.into(),
                Rm: Rm.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | u32::from(self.Z) << 24u32
                    | 0b0u32 << 23u32
                    | u32::from(self.op) << 21u32
                    | 0b111110000u32 << 12u32
                    | u32::from(self.A) << 11u32
                    | u32::from(self.M) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rm) << 0u32,
            )
        }
    }
}
pub mod BRAB_64P_branch_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BRAB_64P_branch_reg {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<2>,
        pub A: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
    }
    impl BRAB_64P_branch_reg {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<2>>,
            A: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                op: op.into(),
                A: A.into(),
                M: M.into(),
                Rn: Rn.into(),
                Rm: Rm.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | u32::from(self.Z) << 24u32
                    | 0b0u32 << 23u32
                    | u32::from(self.op) << 21u32
                    | 0b111110000u32 << 12u32
                    | u32::from(self.A) << 11u32
                    | u32::from(self.M) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rm) << 0u32,
            )
        }
    }
}
pub mod BLRAA_64P_branch_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BLRAA_64P_branch_reg {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<2>,
        pub A: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
    }
    impl BLRAA_64P_branch_reg {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<2>>,
            A: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                op: op.into(),
                A: A.into(),
                M: M.into(),
                Rn: Rn.into(),
                Rm: Rm.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | u32::from(self.Z) << 24u32
                    | 0b0u32 << 23u32
                    | u32::from(self.op) << 21u32
                    | 0b111110000u32 << 12u32
                    | u32::from(self.A) << 11u32
                    | u32::from(self.M) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rm) << 0u32,
            )
        }
    }
}
pub mod BLRAB_64P_branch_reg {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct BLRAB_64P_branch_reg {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub op: ::aarchmrs_types::BitValue<2>,
        pub A: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
    }
    impl BLRAB_64P_branch_reg {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            op: impl Into<::aarchmrs_types::BitValue<2>>,
            A: impl Into<::aarchmrs_types::BitValue<1>>,
            M: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                op: op.into(),
                A: A.into(),
                M: M.into(),
                Rn: Rn.into(),
                Rm: Rm.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | u32::from(self.Z) << 24u32
                    | 0b0u32 << 23u32
                    | u32::from(self.op) << 21u32
                    | 0b111110000u32 << 12u32
                    | u32::from(self.A) << 11u32
                    | u32::from(self.M) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rm) << 0u32,
            )
        }
    }
}
