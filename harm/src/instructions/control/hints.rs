/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::control::hints::{
    BTI_HB_hints::BTI_HB_hints, HINT_HM_hints::HINT_HM_hints, STSHH_HI_hints::STSHH_HI_hints,
};

use crate::{bits::UBitValue, instructions::RawInstruction};

macro_rules! define_simple_hint {
    ($name:ident, $cat:ident) => {
        ::paste::paste! {
            #[derive(Debug, Copy, Clone)]
            pub struct [<$name:camel>];

            impl $crate::instructions::RawInstruction for [<$name:camel>]{
                #[inline]
                fn to_code(&self) -> aarchmrs_types::InstructionCode {
                    ::aarchmrs_instructions::A64::control::hints::[<$name:upper _ $cat _hints>]::[<$name:upper _ $cat _hints>]()
                }
            }

            #[inline]
            pub fn [<$name:lower>]() -> [<$name:camel>] {
                [<$name:camel>]
            }
        }
    };
}

define_simple_hint!(AUTIA1716, HI);
define_simple_hint!(AUTIASP, HI);
define_simple_hint!(AUTIAZ, HI);
define_simple_hint!(AUTIB1716, HI);
define_simple_hint!(AUTIBSP, HI);
define_simple_hint!(AUTIBZ, HI);
define_simple_hint!(CHKFEAT, HF);
define_simple_hint!(CSDB, HI);
define_simple_hint!(DGH, HI);
define_simple_hint!(ESB, HI);
define_simple_hint!(GCSB, HD);
define_simple_hint!(NOP, HI);
define_simple_hint!(PACIA1716, HI);
define_simple_hint!(PACIASP, HI);
define_simple_hint!(PACIAZ, HI);
define_simple_hint!(PACIB1716, HI);
define_simple_hint!(PACIBSP, HI);
define_simple_hint!(PACIBZ, HI);
define_simple_hint!(PACM, HI);
define_simple_hint!(PSB, HC);
define_simple_hint!(SEV, HI);
define_simple_hint!(SEVL, HI);
define_simple_hint!(TSB, HC);
define_simple_hint!(WFE, HI);
define_simple_hint!(WFI, HI);
define_simple_hint!(XPACLRI, HI);

#[derive(Debug, Copy, Clone)]
pub struct Yield;

impl crate::instructions::RawInstruction for Yield {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        ::aarchmrs_instructions::A64::control::hints::YIELD_HI_hints::YIELD_HI_hints()
    }
}

// paste cannot output a reseved keyword identifier. See https://github.com/dtolnay/paste/issues/74 for no info.
#[inline]
pub fn r#yield() -> Yield {
    Yield
}

pub type HintMode = UBitValue<7>;

pub struct Hint(HintMode);

impl RawInstruction for Hint {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let hint: u32 = self.0.bits();
        let crm = hint >> 3;
        let op2 = hint & 0x7;
        HINT_HM_hints(crm.into(), op2.into())
    }
}

// C6.2.92 HINT ... An assembler may support assembly of allocated encodings using HINT with the corresponding <imm>
// value, but it is not required to do so.
pub fn hint(mode: HintMode) -> Hint {
    Hint(mode)
}
// TODO bitset: J = 0b10 and C = 0b01.
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum BtiSet {
    EMPTY = 0b00,
    C = 0b01,
    J = 0b10,
    JC = 0b11,
}

pub struct Bti(BtiSet);

impl RawInstruction for Bti {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        BTI_HB_hints((self.0 as u8).into())
    }
}

pub fn bti(bti: BtiSet) -> Bti {
    Bti(bti)
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum StshhPolicy {
    KEEP = 0,
    STRM = 1,
}

impl From<StshhPolicy> for UBitValue<1> {
    fn from(policy: StshhPolicy) -> Self {
        match policy {
            StshhPolicy::KEEP => UBitValue::from(false),
            StshhPolicy::STRM => UBitValue::from(true),
        }
    }
}

pub struct Stshh(StshhPolicy);

impl RawInstruction for Stshh {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let op2: UBitValue<1> = self.0.into();
        STSHH_HI_hints(op2.into())
    }
}

pub fn stshh(policy: StshhPolicy) -> Stshh {
    Stshh(policy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::InstructionSeq;
    use harm_test_utils::test_cases;

    const HINTS_DB: &str = "
d503219f	autia1716
d50323bf	autiasp
d503239f	autiaz
d50321df	autib1716
d50323ff	autibsp
d50323df	autibz
d503251f	chkfeat x16
d503229f	csdb
d50320df	dgh
d503221f	esb
d503227f	gcsb dsync
d503201f	nop
d503211f	pacia1716
d503233f	paciasp
d503231f	paciaz
d503215f	pacib1716
d503237f	pacibsp
d503235f	pacibz
d50324ff	pacm
d503223f	psb csync
d503209f	sev
d50320bf	sevl
d503225f	tsb csync
d503205f	wfe
d503207f	wfi
d50320ff	xpaclri
d503203f	yield
d503255f	hint #0x2a
d503241f	bti
d503245f	bti c
d503249f	bti j
d50324df	bti jc
";

    test_cases!(
        HINTS_DB, untested_hints;
        test_autia1716, autia1716(), "autia1716";
        test_autiasp, autiasp(), "autiasp";
        test_autiaz, autiaz(), "autiaz";
        test_autib1716, autib1716(), "autib1716";
        test_autibsp, autibsp(), "autibsp";
        test_autibz, autibz(), "autibz";
        test_chkfeat, chkfeat(), "chkfeat x16";
        test_csdb, csdb(), "csdb";
        test_dgh, dgh(), "dgh";
        test_esb, esb(), "esb";
        test_gcsb, gcsb(), "gcsb dsync";
        test_nop, nop(), "nop";
        test_pacia1716, pacia1716(), "pacia1716";
        test_paciasp, paciasp(), "paciasp";
        test_paciaz, paciaz(), "paciaz";
        test_pacib1716, pacib1716(), "pacib1716";
        test_pacibsp, pacibsp(), "pacibsp";
        test_pacibz, pacibz(), "pacibz";
        test_pacm, pacm(), "pacm";
        test_psb , psb(), "psb csync";
        test_sev, sev(), "sev";
        test_sevl, sevl(), "sevl";
        test_tsb , tsb (), "tsb csync";
        test_wfe, wfe(), "wfe";
        test_wfi, wfi(), "wfi";
        test_xpaclri, xpaclri(), "xpaclri";
        test_yield, r#yield(), "yield";
        test_hint_0x2a, hint(UBitValue::new(0x2a).unwrap()), "hint #0x2a";
        test_bti_empty, bti(BtiSet::EMPTY), "bti";
        test_bti_c, bti(BtiSet::C), "bti c";
        test_bti_j, bti(BtiSet::J), "bti j";
        test_bti_jc, bti(BtiSet::JC), "bti jc";
    );
}
