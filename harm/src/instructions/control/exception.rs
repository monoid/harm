/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::control::exception::{
    BRK_EX_exception::BRK_EX_exception, DCPS1_DC_exception::DCPS1_DC_exception,
    DCPS2_DC_exception::DCPS2_DC_exception, DCPS3_DC_exception::DCPS3_DC_exception,
    HLT_EX_exception::HLT_EX_exception, HVC_EX_exception::HVC_EX_exception,
    SMC_EX_exception::SMC_EX_exception, SVC_EX_exception::SVC_EX_exception,
    TCANCEL_EX_exception::TCANCEL_EX_exception,
};

use crate::{bits::UBitValue, instructions::RawInstruction};

pub type ExceptionId = UBitValue<16>;

macro_rules! define_simple_exception {
    ($name:ident, $cat:ident) => {
        ::paste::paste! {
            #[derive(Debug, Copy, Clone)]
            pub struct [<$name:camel>](ExceptionId);

            impl RawInstruction for [<$name:camel>]{
                #[inline]
                fn to_code(&self) -> aarchmrs_types::InstructionCode {
                    [<$name:upper _ $cat:upper _exception>](self.0.into())
                }
            }

            #[inline]
            pub fn [<$name:lower>](arg: ExceptionId) -> [<$name:camel>] {
                [<$name:camel>](arg)
            }
        }
    };
}

define_simple_exception!(brk, ex);
define_simple_exception!(dcps1, dc);
define_simple_exception!(dcps2, dc);
define_simple_exception!(dcps3, dc);
define_simple_exception!(hlt, ex);
define_simple_exception!(hvc, ex);
define_simple_exception!(smc, ex);
define_simple_exception!(svc, ex);
define_simple_exception!(tcancel, ex);

#[cfg(test)]
mod tests {
    use harm_test_utils::test_cases;

    use super::*;
    use crate::instructions::InstructionSeq as _;

    const EXCEPTION_DB: &str = "
d4200240	brk #0x12
d4a002e1	dcps1 #0x17
d4a004a2	dcps2 #0x25
d4a00563	dcps3 #0x2b
d4400680	hlt #0x34
d40008a2	hvc #0x45
d40009a3	smc #0x4d
d4007061	svc #0x383
d4607660	tcancel #0x3b3
";

    test_cases! {
        EXCEPTION_DB, unchecked_exception_db;
        test_brk, brk(ExceptionId::new(0x12).unwrap()), "brk #0x12";
        test_dcps1, dcps1(ExceptionId::new(0x17).unwrap()), "dcps1 #0x17";
        test_dcps2, dcps2(ExceptionId::new(0x25).unwrap()), "dcps2 #0x25";
        test_dcps3, dcps3(ExceptionId::new(0x2b).unwrap()), "dcps3 #0x2b";
        test_hlt, hlt(ExceptionId::new(0x34).unwrap()), "hlt #0x34";
        test_hvc, hvc(ExceptionId::new(0x45).unwrap()), "hvc #0x45";
        test_smc, smc(ExceptionId::new(0x4d).unwrap()), "smc #0x4d";
        test_svc, svc(ExceptionId::new(0x383).unwrap()), "svc #0x383";
        test_tcancel, tcancel(ExceptionId::new(0x3b3).unwrap()), "tcancel #0x3b3";
    }
}
