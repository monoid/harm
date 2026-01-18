/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use std::collections::HashMap;

use crate::labels::LabelRegistry;
use harm::instructions::InstructionSeq;
use harm::reloc::{LabelId, Offset, Rel64};

// N.N. we keep here internal relocation type, and convert it to external on serialization.
#[derive(Default)]
pub struct Assembler {
    label_manager: LabelRegistry,
    memory: Vec<u8>,
    relocations: HashMap<usize, Rel64>,
}

impl Assembler {
    #[inline]
    pub fn new() -> Self {
        <_>::default()
    }

    pub fn build(self) {
        todo!()
    }

    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            label_manager: LabelRegistry::new(),
            memory: Vec::with_capacity(cap),
            relocations: HashMap::new(),
        }
    }

    pub fn insert<InstSeq: InstructionSeq>(&mut self, s: InstSeq) {
        // TODO align by instruction alignment?
        for (inst, rel) in s.encode() {
            let pos = self.memory.len();
            self.memory.extend(inst.0);
            if let Some(rel) = rel {
                self.relocations.insert(pos, rel);
            }
        }
    }

    // TODO the label have to be aligned.  Except for data labels?..
    // For an instruction, it is alwasy 4 bytes, but for data it can be different, from 1 to N bytes.
    pub fn current_label(&mut self) -> LabelId {
        let pos = self.memory.len();

        // TODO can be fused
        let label_id = self.label_manager.forward_label();
        self.label_manager.define_label(label_id, pos as Offset);

        label_id
    }

    pub fn current_named_label(&mut self, name: &str) -> LabelId {
        let id = self.new_forward_named_label(name);
        self.assign_forward_label(id);
        id
    }

    pub fn new_forward_label(&mut self) -> LabelId {
        self.label_manager.forward_label()
    }

    pub fn new_forward_named_label(&mut self, name: &str) -> LabelId {
        self.label_manager.forward_named_label(name)
    }

    pub fn assign_forward_label(&mut self, label_id: LabelId) {
        let pos = self.memory.len();

        self.label_manager.define_label(label_id, pos as Offset);
    }
}
