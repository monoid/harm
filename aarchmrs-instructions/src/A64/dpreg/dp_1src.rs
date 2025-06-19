/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod RBIT_32_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RBIT_32_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl RBIT_32_dp_1src {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101101011000000000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod REV16_32_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct REV16_32_dp_1src {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl REV16_32_dp_1src {
        #[inline]
        pub fn new(
            opc: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                opc: opc.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011010110000000000u32 << 12u32
                    | u32::from(self.opc) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod REV_32_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct REV_32_dp_1src {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl REV_32_dp_1src {
        #[inline]
        pub fn new(
            opc: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                opc: opc.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01011010110000000000u32 << 12u32
                    | u32::from(self.opc) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CLZ_32_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CLZ_32_dp_1src {
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CLZ_32_dp_1src {
        #[inline]
        pub fn new(
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010110101100000000010u32 << 11u32
                    | u32::from(self.op) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CLS_32_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CLS_32_dp_1src {
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CLS_32_dp_1src {
        #[inline]
        pub fn new(
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b010110101100000000010u32 << 11u32
                    | u32::from(self.op) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CTZ_32_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CTZ_32_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CTZ_32_dp_1src {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101101011000000000110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CNT_32_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CNT_32_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CNT_32_dp_1src {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101101011000000000111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ABS_32_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ABS_32_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ABS_32_dp_1src {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0101101011000000001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod RBIT_64_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RBIT_64_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl RBIT_64_dp_1src {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101101011000000000000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod REV16_64_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct REV16_64_dp_1src {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl REV16_64_dp_1src {
        #[inline]
        pub fn new(
            opc: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                opc: opc.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000000000u32 << 12u32
                    | u32::from(self.opc) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod REV32_64_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct REV32_64_dp_1src {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl REV32_64_dp_1src {
        #[inline]
        pub fn new(
            opc: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                opc: opc.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000000000u32 << 12u32
                    | u32::from(self.opc) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod REV_64_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct REV_64_dp_1src {
        pub opc: ::aarchmrs_types::BitValue<2>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl REV_64_dp_1src {
        #[inline]
        pub fn new(
            opc: impl Into<::aarchmrs_types::BitValue<2>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                opc: opc.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000000000u32 << 12u32
                    | u32::from(self.opc) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CLZ_64_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CLZ_64_dp_1src {
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CLZ_64_dp_1src {
        #[inline]
        pub fn new(
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000000010u32 << 11u32
                    | u32::from(self.op) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CLS_64_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CLS_64_dp_1src {
        pub op: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CLS_64_dp_1src {
        #[inline]
        pub fn new(
            op: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                op: op.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000000010u32 << 11u32
                    | u32::from(self.op) << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CTZ_64_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CTZ_64_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CTZ_64_dp_1src {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101101011000000000110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod CNT_64_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CNT_64_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl CNT_64_dp_1src {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101101011000000000111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod ABS_64_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ABS_64_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl ABS_64_dp_1src {
        #[inline]
        pub fn new(
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101101011000000001000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod PACIA_64P_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIA_64P_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl PACIA_64P_dp_1src {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | u32::from(self.Z) << 13u32
                    | 0b000u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod PACIB_64P_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIB_64P_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl PACIB_64P_dp_1src {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | u32::from(self.Z) << 13u32
                    | 0b001u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod PACDA_64P_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACDA_64P_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl PACDA_64P_dp_1src {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | u32::from(self.Z) << 13u32
                    | 0b010u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod PACDB_64P_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACDB_64P_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl PACDB_64P_dp_1src {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | u32::from(self.Z) << 13u32
                    | 0b011u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod AUTIA_64P_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIA_64P_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTIA_64P_dp_1src {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | u32::from(self.Z) << 13u32
                    | 0b100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod AUTIB_64P_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIB_64P_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTIB_64P_dp_1src {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | u32::from(self.Z) << 13u32
                    | 0b101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod AUTDA_64P_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTDA_64P_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTDA_64P_dp_1src {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | u32::from(self.Z) << 13u32
                    | 0b110u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod AUTDB_64P_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTDB_64P_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rn: ::aarchmrs_types::BitValue<5>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTDB_64P_dp_1src {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            Rn: impl Into<::aarchmrs_types::BitValue<5>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                Rn: Rn.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | u32::from(self.Z) << 13u32
                    | 0b111u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod PACIZA_64Z_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIZA_64Z_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl PACIZA_64Z_dp_1src {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | u32::from(self.Z) << 13u32
                    | 0b00011111u32 << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod PACIZB_64Z_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIZB_64Z_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl PACIZB_64Z_dp_1src {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | u32::from(self.Z) << 13u32
                    | 0b00111111u32 << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod PACDZA_64Z_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACDZA_64Z_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl PACDZA_64Z_dp_1src {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | u32::from(self.Z) << 13u32
                    | 0b01011111u32 << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod PACDZB_64Z_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACDZB_64Z_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl PACDZB_64Z_dp_1src {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | u32::from(self.Z) << 13u32
                    | 0b01111111u32 << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod AUTIZA_64Z_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIZA_64Z_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTIZA_64Z_dp_1src {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | u32::from(self.Z) << 13u32
                    | 0b10011111u32 << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod AUTIZB_64Z_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIZB_64Z_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTIZB_64Z_dp_1src {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | u32::from(self.Z) << 13u32
                    | 0b10111111u32 << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod AUTDZA_64Z_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTDZA_64Z_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTDZA_64Z_dp_1src {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | u32::from(self.Z) << 13u32
                    | 0b11011111u32 << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod AUTDZB_64Z_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTDZB_64Z_dp_1src {
        pub Z: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTDZB_64Z_dp_1src {
        #[inline]
        pub fn new(
            Z: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                Z: Z.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000100u32 << 14u32
                    | u32::from(self.Z) << 13u32
                    | 0b11111111u32 << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod XPACI_64Z_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct XPACI_64Z_dp_1src {
        pub D: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl XPACI_64Z_dp_1src {
        #[inline]
        pub fn new(
            D: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                D: D.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000101000u32 << 11u32
                    | u32::from(self.D) << 10u32
                    | 0b11111u32 << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod XPACD_64Z_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct XPACD_64Z_dp_1src {
        pub D: ::aarchmrs_types::BitValue<1>,
        pub Rd: ::aarchmrs_types::BitValue<5>,
    }
    impl XPACD_64Z_dp_1src {
        #[inline]
        pub fn new(
            D: impl Into<::aarchmrs_types::BitValue<1>>,
            Rd: impl Into<::aarchmrs_types::BitValue<5>>,
        ) -> Self {
            Self {
                D: D.into(),
                Rd: Rd.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110110101100000101000u32 << 11u32
                    | u32::from(self.D) << 10u32
                    | 0b11111u32 << 5u32
                    | u32::from(self.Rd) << 0u32,
            )
        }
    }
}
pub mod PACNBIASPPC_64LR_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACNBIASPPC_64LR_dp_1src {}
    impl PACNBIASPPC_64LR_dp_1src {
        #[inline]
        pub fn new() -> Self {
            Self {}
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000011000001111111110u32 << 0u32,
            )
        }
    }
}
pub mod PACNBIBSPPC_64LR_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACNBIBSPPC_64LR_dp_1src {}
    impl PACNBIBSPPC_64LR_dp_1src {
        #[inline]
        pub fn new() -> Self {
            Self {}
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000011000011111111110u32 << 0u32,
            )
        }
    }
}
pub mod PACIA171615_64LR_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIA171615_64LR_dp_1src {}
    impl PACIA171615_64LR_dp_1src {
        #[inline]
        pub fn new() -> Self {
            Self {}
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000011000101111111110u32 << 0u32,
            )
        }
    }
}
pub mod PACIB171615_64LR_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIB171615_64LR_dp_1src {}
    impl PACIB171615_64LR_dp_1src {
        #[inline]
        pub fn new() -> Self {
            Self {}
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000011000111111111110u32 << 0u32,
            )
        }
    }
}
pub mod AUTIASPPCR_64LRR_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIASPPCR_64LRR_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTIASPPCR_64LRR_dp_1src {
        #[inline]
        pub fn new(Rn: impl Into<::aarchmrs_types::BitValue<5>>) -> Self {
            Self { Rn: Rn.into() }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101101011000001100100u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11110u32 << 0u32,
            )
        }
    }
}
pub mod AUTIBSPPCR_64LRR_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIBSPPCR_64LRR_dp_1src {
        pub Rn: ::aarchmrs_types::BitValue<5>,
    }
    impl AUTIBSPPCR_64LRR_dp_1src {
        #[inline]
        pub fn new(Rn: impl Into<::aarchmrs_types::BitValue<5>>) -> Self {
            Self { Rn: Rn.into() }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101101011000001100101u32 << 10u32
                    | u32::from(self.Rn) << 5u32
                    | 0b11110u32 << 0u32,
            )
        }
    }
}
pub mod PACIASPPC_64LR_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIASPPC_64LR_dp_1src {}
    impl PACIASPPC_64LR_dp_1src {
        #[inline]
        pub fn new() -> Self {
            Self {}
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000011010001111111110u32 << 0u32,
            )
        }
    }
}
pub mod PACIBSPPC_64LR_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct PACIBSPPC_64LR_dp_1src {}
    impl PACIBSPPC_64LR_dp_1src {
        #[inline]
        pub fn new() -> Self {
            Self {}
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000011010011111111110u32 << 0u32,
            )
        }
    }
}
pub mod AUTIA171615_64LR_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIA171615_64LR_dp_1src {}
    impl AUTIA171615_64LR_dp_1src {
        #[inline]
        pub fn new() -> Self {
            Self {}
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000011011101111111110u32 << 0u32,
            )
        }
    }
}
pub mod AUTIB171615_64LR_dp_1src {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIB171615_64LR_dp_1src {}
    impl AUTIB171615_64LR_dp_1src {
        #[inline]
        pub fn new() -> Self {
            Self {}
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11011010110000011011111111111110u32 << 0u32,
            )
        }
    }
}
