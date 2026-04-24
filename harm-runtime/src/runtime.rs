/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use std::collections::HashMap;

use crate::builder::Builder;
use crate::labels::LabelRegistry;
use crate::memory::{IntoExecutableMemory, IntoPositionedMemory, Memory, PositionedMemory};
use harm::instructions::InstructionSeq;
use harm::reloc::{LabelId, Offset64, Rel64};

// N.B. we keep here internal relocation type, and convert it to external on serialization.
#[derive(Default)]
pub struct Assembler<Mem: Memory> {
    label_manager: LabelRegistry,
    memory: Mem,
    relocations: HashMap<usize, Rel64>,
}

impl<Mem: Memory> Assembler<Mem> {
    #[inline]
    pub fn new(mem: Mem) -> Self {
        Self {
            label_manager: LabelRegistry::new(),
            memory: mem,
            relocations: HashMap::new(),
        }
    }

    pub fn build<FM>(self) -> Result<FM, BuilderError>
    where
        Mem: IntoPositionedMemory<FM>,
        FM: PositionedMemory,
    {
        let mut fixed_memory = self.memory.into_positioned_memory()?;
        let base = fixed_memory.get_base_address();
        let builder = Builder::new(fixed_memory.as_mut(), base);
        builder.build(
            self.label_manager
                .defined_labels()
                .map(|(name, offset)| (name, offset as i64)),
            self.relocations.into_iter(),
        )?;
        Ok(fixed_memory)
    }

    pub fn compile<FM>(self) -> Result<<FM as IntoExecutableMemory>::ExecutableMemory, BuilderError>
    where
        Mem: IntoPositionedMemory<FM>,
        FM: PositionedMemory + IntoExecutableMemory,
    {
        let fixed_memory = self.build()?;
        fixed_memory.into_executable_memory()
    }

    pub fn append<InstSeq: InstructionSeq>(&mut self, s: InstSeq) -> Result<(), Mem::ExtendError> {
        // TODO align by instruction alignment?
        for (inst, rel) in s.encode() {
            let pos = self.memory.pos();
            self.memory.try_extend(inst.0.iter().cloned())?;
            if let Some(rel) = rel {
                self.relocations.insert(pos, rel);
            }
        }
        Ok(())
    }

    // TODO the label have to be aligned.  Except for data labels?..
    // For an instruction, it is alwasy 4 bytes, but for data it can be different, from 1 to N bytes.
    pub fn current_label(&mut self) -> LabelId {
        let pos = self.memory.pos();

        // TODO can be fused
        let label_id = self.label_manager.forward_label();
        self.label_manager.define_label(label_id, pos as Offset64);

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
        let pos = self.memory.pos();

        self.label_manager.define_label(label_id, pos as Offset64);
    }
}
