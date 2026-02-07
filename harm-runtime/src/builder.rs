/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use std::collections::HashMap;

use harm::reloc::{Rel64, Rel64Error};

#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    #[error("Relocation offset out of range: {0}")]
    BadRelocationOffset(usize),
    #[error("Address overflow: base {0}, offset {1}")]
    AddressOverflow(u64, usize),
    #[error("Relocation error: {nested:?} at offset {offset}")]
    Relocation { nested: Rel64Error, offset: usize },
}

/// Do static relocations: recalculate labels and applies relocations, producing memory ready for execution.
///
/// Please note that real memory location may be different from base address: it allows to build at some buffer
/// and then move data to real position later.
pub struct Builder<'mem> {
    mem: &'mem mut [u8], // real memory
    base: u64,           // virtual base, on ARM system usually matches with `mem` start
}

impl<'mem> Builder<'mem> {
    pub fn new(mem: &'mem mut [u8], base: u64) -> Self {
        Self { mem, base }
    }

    pub fn build(
        self,
        _label_defs: impl Iterator<Item = (&'mem str, i64)>,
        relocations: impl Iterator<Item = (usize, Rel64)>,
    ) -> Result<HashMap<&'mem str, u64>, BuilderError> {
        // Recalculate labels.
        // todo!();

        // Apply relocations to the self.mem.
        for (offset, rel) in relocations {
            let label_addr = todo!(); // Get the address of the label for this relocation.
            rel.apply(self.base, label_addr, self.mem, offset)
                .map_err(|nested| BuilderError::Relocation { nested, offset })?;
        }

        Ok(Default::default())
    }
}

#[cfg(test)]
mod tests {
    use harm::reloc::{LabelId, LabelRef, Rel64Tag};

    use super::*;

    #[test]
    fn test_good_offset() {
        let mut mem = vec![0u8; 4];
        let builder = Builder::new(&mut mem, 0);
        let label_ref = LabelRef {
            id: LabelId(0),
            addend: 0,
        };
        let relocations = [(0, Rel64::new(Rel64Tag::NONE, label_ref))];
        let res = builder.build([].into_iter(), relocations.into_iter());

        assert!(res.is_ok());
    }

    #[test]
    fn test_bad_offset() {
        let mut mem = vec![0u8; 4];
        let builder = Builder::new(&mut mem, 0);
        let label_ref = LabelRef {
            id: LabelId(0),
            addend: 0,
        };
        let relocations = [(1, Rel64::new(Rel64Tag::NONE, label_ref))];
        let res = builder.build([].into_iter(), relocations.into_iter());

        assert!(matches!(res, Err(BuilderError::BadRelocationOffset(_))));
    }

    #[test]
    fn test_bad_offset_max() {
        let mut mem = vec![0u8; 4];
        let builder = Builder::new(&mut mem, 0);
        let label_ref = LabelRef {
            id: LabelId(0),
            addend: 0,
        };
        let relocations = [(usize::MAX, Rel64::new(Rel64Tag::NONE, label_ref))];
        let res = builder.build([].into_iter(), relocations.into_iter());

        assert!(matches!(res, Err(BuilderError::BadRelocationOffset(_))));
    }
}
