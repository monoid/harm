/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use std::collections::HashMap;

use harm::reloc::{Addr64, LabelId, Offset64, Rel64, Rel64Error};

#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    #[error("Address overflow: base {0}, offset {1}")]
    AddressOverflow(u64, usize),
    #[error("Relocation error: {nested:?} at offset {offset}")]
    Relocation { nested: Rel64Error, offset: usize },
    #[error("Undefined label: {0:?}")]
    UndefinedLabel(LabelId),
}

/// Do static relocations: recalculate labels and applies relocations, producing memory ready for execution.
///
/// Please note that real memory location may be different from base address: it allows to build at some buffer
/// and then move data to real position later.
pub struct Builder<'mem> {
    mem: &'mem mut [u8], // real memory
    base: Addr64,        // virtual base, on ARM system usually matches with `mem` start
}

impl<'mem> Builder<'mem> {
    pub fn new(mem: &'mem mut [u8], base: Addr64) -> Self {
        Self { mem, base }
    }

    pub fn build(
        self,
        named_labels: impl Iterator<Item = (&'mem str, LabelId)>,
        labels: impl Iterator<Item = (LabelId, Offset64)>,
        relocations: impl Iterator<Item = (usize, Rel64)>,
    ) -> Result<HashMap<String, u64>, BuilderError> {
        // Recalculate labels.
        let labels: HashMap<_, _> = labels
            .map(|(label_id, offset)| {
                // TODO is it wrapping?
                let addr = self.base.wrapping_add_signed(offset);
                (label_id, addr)
            })
            .collect();

        // Calculate label addresses.
        let label_addresses = named_labels
            .map(|(name, label_id)| {
                let label_addr = labels
                    .get(&label_id)
                    .copied()
                    .ok_or_else(|| BuilderError::UndefinedLabel(label_id))?;
                Ok((name.to_owned(), label_addr))
            })
            .collect::<Result<_, BuilderError>>()?;
        
        // Apply relocations to the self.mem.
        for (offset, rel) in relocations {
            let label_addr = labels
                .get(&rel.label.id)
                .copied()
                .ok_or_else(|| BuilderError::UndefinedLabel(rel.label.id))?;
            // TODO is it wrapping?
            let label_ref_addr = label_addr.wrapping_add_signed(rel.label.addend);
            
            rel.apply(self.base, label_ref_addr, self.mem, offset)
                .map_err(|nested| BuilderError::Relocation { nested, offset })?;
        }

        Ok(label_addresses)
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
        let res = builder.build(
            [].into_iter(),
            [(LabelId(0), 4)].into_iter(),
            relocations.into_iter(),
        );

        assert!(res.is_ok(), "{res:?}");
    }

    #[test]
    fn test_bad_offset() {
        let mut mem = vec![0u8; 4];
        let builder = Builder::new(&mut mem, 0);
        let label_ref = LabelRef {
            id: LabelId(0),
            addend: 0,
        };
        // N.B. NONE relocation is 0 bytes wide, so 4 doesn't fail.  Use 5.
        let relocations = [(5, Rel64::new(Rel64Tag::NONE, label_ref))];
        let res = builder.build(
            [].into_iter(),
            [(LabelId(0), 4)].into_iter(),
            relocations.into_iter(),
        );

        assert!(
            matches!(
                res,
                Err(BuilderError::Relocation {
                    nested: _,
                    offset: _
                })
            ),
            "{res:?}"
        );
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
        let res = builder.build(
            [].into_iter(),
            [(LabelId(0), 4)].into_iter(),
            relocations.into_iter(),
        );

        assert!(
            matches!(
                res,
                Err(BuilderError::Relocation {
                    nested: _,
                    offset: _
                })
            ),
            "{res:?}"
        );
    }
}
