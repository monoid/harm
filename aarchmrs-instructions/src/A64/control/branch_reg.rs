/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod BR_64_branch_reg {
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BR_64_branch_reg";
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
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            op: ::aarchmrs_types::BitValue<2>,
            A: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rm: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Z,
                op,
                A,
                M,
                Rn,
                Rm,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | self.Z.into_inner() << 24u32
                    | 0b0u32 << 23u32
                    | self.op.into_inner() << 21u32
                    | 0b111110000u32 << 12u32
                    | self.A.into_inner() << 11u32
                    | self.M.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rm.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod BRAAZ_64_branch_reg {
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BRAAZ_64_branch_reg";
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
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            op: ::aarchmrs_types::BitValue<2>,
            A: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rm: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Z,
                op,
                A,
                M,
                Rn,
                Rm,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | self.Z.into_inner() << 24u32
                    | 0b0u32 << 23u32
                    | self.op.into_inner() << 21u32
                    | 0b111110000u32 << 12u32
                    | self.A.into_inner() << 11u32
                    | self.M.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rm.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod BRABZ_64_branch_reg {
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BRABZ_64_branch_reg";
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
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            op: ::aarchmrs_types::BitValue<2>,
            A: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rm: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Z,
                op,
                A,
                M,
                Rn,
                Rm,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | self.Z.into_inner() << 24u32
                    | 0b0u32 << 23u32
                    | self.op.into_inner() << 21u32
                    | 0b111110000u32 << 12u32
                    | self.A.into_inner() << 11u32
                    | self.M.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rm.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod BLR_64_branch_reg {
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BLR_64_branch_reg";
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
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            op: ::aarchmrs_types::BitValue<2>,
            A: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rm: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Z,
                op,
                A,
                M,
                Rn,
                Rm,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | self.Z.into_inner() << 24u32
                    | 0b0u32 << 23u32
                    | self.op.into_inner() << 21u32
                    | 0b111110000u32 << 12u32
                    | self.A.into_inner() << 11u32
                    | self.M.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rm.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod BLRAAZ_64_branch_reg {
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BLRAAZ_64_branch_reg";
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
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            op: ::aarchmrs_types::BitValue<2>,
            A: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rm: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Z,
                op,
                A,
                M,
                Rn,
                Rm,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | self.Z.into_inner() << 24u32
                    | 0b0u32 << 23u32
                    | self.op.into_inner() << 21u32
                    | 0b111110000u32 << 12u32
                    | self.A.into_inner() << 11u32
                    | self.M.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rm.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod BLRABZ_64_branch_reg {
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BLRABZ_64_branch_reg";
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
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            op: ::aarchmrs_types::BitValue<2>,
            A: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rm: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Z,
                op,
                A,
                M,
                Rn,
                Rm,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | self.Z.into_inner() << 24u32
                    | 0b0u32 << 23u32
                    | self.op.into_inner() << 21u32
                    | 0b111110000u32 << 12u32
                    | self.A.into_inner() << 11u32
                    | self.M.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rm.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod RET_64R_branch_reg {
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "RET_64R_branch_reg";
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
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            op: ::aarchmrs_types::BitValue<2>,
            A: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rm: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Z,
                op,
                A,
                M,
                Rn,
                Rm,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | self.Z.into_inner() << 24u32
                    | 0b0u32 << 23u32
                    | self.op.into_inner() << 21u32
                    | 0b111110000u32 << 12u32
                    | self.A.into_inner() << 11u32
                    | self.M.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rm.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod RETAASPPCR_64M_branch_reg {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111101111100000u32;
    pub const OPCODE: u32 = 0b11010110010111110000101111100000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "RETAASPPCR_64M_branch_reg";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RETAASPPCR_64M_branch_reg {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
    }
    impl RETAASPPCR_64M_branch_reg {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { M, Rm }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110101100101111100001u32 << 11u32
                    | self.M.into_inner() << 10u32
                    | 0b11111u32 << 5u32
                    | self.Rm.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod RETAA_64E_branch_reg {
    pub const OPCODE_MASK: u32 = 0b11111110100111111111001111100000u32;
    pub const OPCODE: u32 = 0b11010110000111110000001111100000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "RETAA_64E_branch_reg";
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
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            op: ::aarchmrs_types::BitValue<2>,
            A: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Z, op, A, M, Rm }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | self.Z.into_inner() << 24u32
                    | 0b0u32 << 23u32
                    | self.op.into_inner() << 21u32
                    | 0b111110000u32 << 12u32
                    | self.A.into_inner() << 11u32
                    | self.M.into_inner() << 10u32
                    | 0b11111u32 << 5u32
                    | self.Rm.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod RETABSPPCR_64M_branch_reg {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111101111100000u32;
    pub const OPCODE: u32 = 0b11010110010111110000101111100000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "RETABSPPCR_64M_branch_reg";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RETABSPPCR_64M_branch_reg {
        pub M: ::aarchmrs_types::BitValue<1>,
        pub Rm: ::aarchmrs_types::BitValue<5>,
    }
    impl RETABSPPCR_64M_branch_reg {
        #[inline]
        pub const fn new(
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { M, Rm }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110101100101111100001u32 << 11u32
                    | self.M.into_inner() << 10u32
                    | 0b11111u32 << 5u32
                    | self.Rm.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod RETAB_64E_branch_reg {
    pub const OPCODE_MASK: u32 = 0b11111110100111111111001111100000u32;
    pub const OPCODE: u32 = 0b11010110000111110000001111100000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "RETAB_64E_branch_reg";
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
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            op: ::aarchmrs_types::BitValue<2>,
            A: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rm: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self { Z, op, A, M, Rm }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | self.Z.into_inner() << 24u32
                    | 0b0u32 << 23u32
                    | self.op.into_inner() << 21u32
                    | 0b111110000u32 << 12u32
                    | self.A.into_inner() << 11u32
                    | self.M.into_inner() << 10u32
                    | 0b11111u32 << 5u32
                    | self.Rm.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod ERET_64E_branch_reg {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111001111111111u32;
    pub const OPCODE: u32 = 0b11010110100111110000001111100000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ERET_64E_branch_reg";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ERET_64E_branch_reg {
        pub A: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
    }
    impl ERET_64E_branch_reg {
        #[inline]
        pub const fn new(
            A: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self { A, M }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010110100111110000u32 << 12u32
                    | self.A.into_inner() << 11u32
                    | self.M.into_inner() << 10u32
                    | 0b1111100000u32 << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod ERETAA_64E_branch_reg {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111001111111111u32;
    pub const OPCODE: u32 = 0b11010110100111110000001111111111u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ERETAA_64E_branch_reg";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ERETAA_64E_branch_reg {
        pub A: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
    }
    impl ERETAA_64E_branch_reg {
        #[inline]
        pub const fn new(
            A: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self { A, M }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010110100111110000u32 << 12u32
                    | self.A.into_inner() << 11u32
                    | self.M.into_inner() << 10u32
                    | 0b1111111111u32 << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod ERETAB_64E_branch_reg {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111001111111111u32;
    pub const OPCODE: u32 = 0b11010110100111110000001111111111u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "ERETAB_64E_branch_reg";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ERETAB_64E_branch_reg {
        pub A: ::aarchmrs_types::BitValue<1>,
        pub M: ::aarchmrs_types::BitValue<1>,
    }
    impl ERETAB_64E_branch_reg {
        #[inline]
        pub const fn new(
            A: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
        ) -> Self {
            Self { A, M }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010110100111110000u32 << 12u32
                    | self.A.into_inner() << 11u32
                    | self.M.into_inner() << 10u32
                    | 0b1111111111u32 << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod DRPS_64E_branch_reg {
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    pub const OPCODE: u32 = 0b11010110101111110000001111100000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "DRPS_64E_branch_reg";
    #[derive(Copy, Clone, Debug, Default)]
    pub struct DRPS_64E_branch_reg {}
    impl DRPS_64E_branch_reg {
        #[inline]
        pub const fn new() -> Self {
            Self {}
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11010110101111110000001111100000u32 << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod BRAA_64P_branch_reg {
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BRAA_64P_branch_reg";
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
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            op: ::aarchmrs_types::BitValue<2>,
            A: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rm: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Z,
                op,
                A,
                M,
                Rn,
                Rm,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | self.Z.into_inner() << 24u32
                    | 0b0u32 << 23u32
                    | self.op.into_inner() << 21u32
                    | 0b111110000u32 << 12u32
                    | self.A.into_inner() << 11u32
                    | self.M.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rm.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod BRAB_64P_branch_reg {
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BRAB_64P_branch_reg";
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
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            op: ::aarchmrs_types::BitValue<2>,
            A: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rm: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Z,
                op,
                A,
                M,
                Rn,
                Rm,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | self.Z.into_inner() << 24u32
                    | 0b0u32 << 23u32
                    | self.op.into_inner() << 21u32
                    | 0b111110000u32 << 12u32
                    | self.A.into_inner() << 11u32
                    | self.M.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rm.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod BLRAA_64P_branch_reg {
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BLRAA_64P_branch_reg";
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
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            op: ::aarchmrs_types::BitValue<2>,
            A: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rm: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Z,
                op,
                A,
                M,
                Rn,
                Rm,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | self.Z.into_inner() << 24u32
                    | 0b0u32 << 23u32
                    | self.op.into_inner() << 21u32
                    | 0b111110000u32 << 12u32
                    | self.A.into_inner() << 11u32
                    | self.M.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rm.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
pub mod BLRAB_64P_branch_reg {
    pub const OPCODE_MASK: u32 = 0b11111110100111111111000000000000u32;
    pub const OPCODE: u32 = 0b11010110000111110000000000000000u32;
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    pub const NAME: &str = "BLRAB_64P_branch_reg";
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
        pub const fn new(
            Z: ::aarchmrs_types::BitValue<1>,
            op: ::aarchmrs_types::BitValue<2>,
            A: ::aarchmrs_types::BitValue<1>,
            M: ::aarchmrs_types::BitValue<1>,
            Rn: ::aarchmrs_types::BitValue<5>,
            Rm: ::aarchmrs_types::BitValue<5>,
        ) -> Self {
            Self {
                Z,
                op,
                A,
                M,
                Rn,
                Rm,
            }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b1101011u32 << 25u32
                    | self.Z.into_inner() << 24u32
                    | 0b0u32 << 23u32
                    | self.op.into_inner() << 21u32
                    | 0b111110000u32 << 12u32
                    | self.A.into_inner() << 11u32
                    | self.M.into_inner() << 10u32
                    | self.Rn.into_inner() << 5u32
                    | self.Rm.into_inner() << 0u32,
            )
        }
        #[inline]
        pub const fn opcode_mask() -> u32 {
            self::OPCODE_MASK
        }
        #[inline]
        pub const fn opcode() -> u32 {
            self::OPCODE
        }
        #[inline]
        pub const fn should_be_mask() -> u32 {
            self::SHOULD_BE_MASK
        }
        #[inline]
        pub const fn match_opcode(opcode: u32) -> bool {
            let opcode = opcode & self::OPCODE_MASK;
            opcode == self::OPCODE
        }
        #[inline]
        pub const fn name() -> &'static str {
            self::NAME
        }
    }
}
