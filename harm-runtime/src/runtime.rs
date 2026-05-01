/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use std::collections::HashMap;

use crate::builder::{Builder, BuilderError};
use crate::labels::LabelRegistry;
use crate::memory::{IntoExecutableMemory, IntoPositionedMemory, Memory, PositionedMemory};
use harm::instructions::InstructionSeq;
use harm::reloc::{Addr64, LabelId, Offset64, Rel64};

#[derive(Debug, thiserror::Error)]
pub enum AssemblerError<MemErr, PMErr, EMErr> {
    #[error("builder error: {0}")]
    Builder(#[from] BuilderError),
    #[error("memory error: {0}")]
    Memory(MemErr),
    #[error("positioned memory error: {0}")]
    PositionedMemory(PMErr),
    #[error("executable memory error: {0}")]
    ExecutableMemory(EMErr),
}
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

    /// Build the program without making it executable.
    pub fn build<FM, E>(
        self,
    ) -> Result<
        (FM, HashMap<String, Addr64>),
        AssemblerError<
            Mem::ExtendError,
            <Mem as IntoPositionedMemory<FM>>::PositionedMemoryError,
            E,
        >,
    >
    where
        Mem: IntoPositionedMemory<FM>,
        FM: PositionedMemory,
    {
        let mut fixed_memory = self
            .memory
            .into_positioned_memory()
            .map_err(AssemblerError::PositionedMemory)?;
        let base = fixed_memory.get_base_address();
        let builder = Builder::new(fixed_memory.as_mut(), base);
        let labels = builder.build(
            self.label_manager.get_named_labels(),
            self.label_manager.get_defined_labels(),
            self.relocations.into_iter(),
        )?;
        Ok((fixed_memory, labels))
    }

    /// Build the program and make it executable.
    pub fn compile<FM>(
        self,
    ) -> Result<
        (
            <FM as IntoExecutableMemory>::ExecutableMemory,
            HashMap<String, Addr64>,
        ),
        AssemblerError<
            Mem::ExtendError,
            <Mem as IntoPositionedMemory<FM>>::PositionedMemoryError,
            <FM as IntoExecutableMemory>::ExecutableMemoryError,
        >,
    >
    where
        Mem: IntoPositionedMemory<FM>,
        FM: PositionedMemory + IntoExecutableMemory,
    {
        let (fixed_memory, labels) = self.build()?;
        let exec_memory = fixed_memory
            .into_executable_memory()
            .map_err(AssemblerError::ExecutableMemory)?;
        Ok((exec_memory, labels))
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
        self.label_manager.get_forward_named_label(name)
    }

    pub fn assign_forward_label(&mut self, label_id: LabelId) {
        let pos = self.memory.pos();

        self.label_manager.define_label(label_id, pos as Offset64);
    }
}

#[cfg(test)]
mod tests {
    use harm::{
        instructions::{
            arith::add::add,
            control::{b, ret},
            dpimm::movz,
        },
        register::Reg64,
        reloc::LabelRef,
    };

    use crate::memory::{ForeignMemoryBuffer, foreign_memory::ForeignMemory};

    use super::*;

    #[test]
    fn test_assembler_build() {
        let mem = ForeignMemoryBuffer::new(0x1000);
        let mut asm = Assembler::new(mem);

        let finish_label = asm.new_forward_label();
        // TODO constructor
        let finish_ref = LabelRef {
            id: finish_label,
            addend: 0,
        };

        asm.append(add(Reg64::X0, Reg64::X0, Reg64::X1));
        asm.append(b(finish_ref));
        asm.append(movz(Reg64::X0, 0));
        asm.assign_forward_label(finish_label);
        asm.append(ret());

        let (fm, _) = asm.build::<ForeignMemory, ()>().unwrap();

        let mut expected = vec![];
        expected.extend(add(Reg64::X0, Reg64::X0, Reg64::X1).bytes());
        expected.extend(b(8).unwrap().bytes());
        expected.extend(movz(Reg64::X0, 0).bytes());
        expected.extend(ret().bytes());

        assert_eq!(fm.as_ref(), &*expected);
    }

    // Execute the code from the `test_assembler_build`.
    #[cfg(all(target_arch = "aarch64", feature = "memmap2"))]
    #[test]
    fn test_assembler_aarch64_execute() {
        use crate::memory::MmapBuffer;

        let mem = MmapBuffer::allocate(16).unwrap();
        let mut asm = Assembler::new(mem);

        let finish_label = asm.new_forward_label();
        // TODO constructor
        let finish_ref = LabelRef {
            id: finish_label,
            addend: 0,
        };

        let _start = asm.current_named_label("_start");
        asm.append(add(Reg64::X0, Reg64::X0, Reg64::X1)).unwrap();
        asm.append(b(finish_ref)).unwrap();
        asm.append(movz(Reg64::X0, 0)).unwrap();
        asm.assign_forward_label(finish_label);
        asm.append(ret()).unwrap();

        let (fm, labels) = asm.compile().unwrap();
        let start_addr = labels.get("_start").cloned().unwrap() as usize;

        let res = unsafe {
            clear_cache::clear_cache(fm.as_ptr(), fm.as_ptr().add(fm.len()));

            let start: unsafe extern "C" fn(i64, i64) -> i64 = std::mem::transmute(start_addr);
            start(42, 8)
        };
        assert_eq!(res, 50);
    }
}
