/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use std::collections::HashMap;

use crate::labels::LabelRegistry;
use harm::InstructionCode;
use harm::reloc::{Reloc, LabelId};
use harm::instructions::InstructionSeq;

// N.N. we keep here internal relocation type, and convert it to external on serialization.
#[derive(Default)]
pub struct Assembler {
    label_manager: LabelRegistry,
    instructions: Vec<InstructionCode>,
    relocations: HashMap<usize, Reloc>,
}

impl Assembler {
    #[inline]
    pub fn new() -> Self {
        <_>::default()
    }

    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            label_manager: LabelRegistry::new(),
            instructions: Vec::with_capacity(cap),
            relocations: HashMap::new(),
        }
    }

    pub fn insert<InstSeq: InstructionSeq>(&mut self, s: InstSeq) {
        for (inst, rel) in s.encode() {
            let pos = self.instructions.len();
            if let Some(rel) = rel {
                self.relocations.insert(pos, rel);
            }
            self.instructions.push(inst);
        }
    }

    // TODO the label have to be aligned.
    // For an instruction, it is alwasy 4 bytes, but for data it can be different, from 1 to N bytes.
    pub fn current_label(&mut self) -> LabelId {
        let pos = self.instructions.len();
        todo!()
    }

    pub fn current_named_label(&mut self, name: &str) -> LabelId {
        let id = self.new_forward_named_label(name);
        self.assign_forward_label(id);
        id
    }

    pub fn new_forward_label(&mut self) -> LabelId {
        todo!()
    }

    pub fn new_forward_named_label(&mut self, name: &str) -> LabelId {
        todo!()
    }

    pub fn assign_forward_label(&mut self, label: LabelId) {
        todo!()
    }
}
