/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use std::collections::HashMap;

use harm::reloc::{LabelId, Offset64};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LabelInfo {
    Forward,
    // TODO segment
    Offset(Offset64),
}

#[derive(Debug, Default)]
pub struct LabelRegistry {
    named_labels: HashMap<String, LabelId>,
    labels: HashMap<LabelId, LabelInfo>,
    next_id: usize,
}

impl LabelRegistry {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn get_forward_named_label(&mut self, name: &str) -> LabelId {
        if let Some(id) = self.named_labels.get(name) {
            *id
        } else {
            let id = self.next_label();
            self.named_labels.insert(name.to_string(), id);
            self.labels.insert(id, LabelInfo::Forward);
            id
        }
    }

    #[inline]
    pub fn forward_label(&mut self) -> LabelId {
        let id = self.next_label();
        self.labels.insert(id, LabelInfo::Forward);
        id
    }

    /// Define the label to have its address to be base address plus `offset`.
    pub fn define_label(&mut self, label_id: LabelId, offset: Offset64) {
        if let Some(info) = self.labels.get_mut(&label_id) {
            match info {
                LabelInfo::Forward => {
                    *info = LabelInfo::Offset(offset);
                }
                LabelInfo::Offset(_) => {
                    todo!("Label {label_id:?} is already defined");
                }
            }
        } else {
            todo!("Label {label_id:?} is not registered");
        }
    }

    /// Define the label to have its address to be base address plus `offset`.
    #[inline]
    pub fn define_named_label(&mut self, name: &str, offset: Offset64) -> LabelId {
        if let Some(id) = self.named_labels.get(name).copied() {
            self.labels.insert(id, LabelInfo::Offset(offset));
            id
        } else {
            let id = self.next_label();
            self.named_labels.insert(name.to_string(), id);
            self.labels.insert(id, LabelInfo::Offset(offset));
            id
        }
    }

    /// Turn the label into a named.
    pub fn name_label(&mut self, id: LabelId, name: &str) {
        if self.labels.contains_key(&id) {
            self.named_labels.insert(name.to_string(), id);
        } else {
            todo!("Label {id:?} is not registered");
        }
    }

    /// Return current label info.
    #[inline]
    pub fn label_info(&self, id: LabelId) -> Option<&LabelInfo> {
        self.labels.get(&id)
    }

    pub fn get_named_labels(&self) -> impl Iterator<Item = (&str, LabelId)> {
        self.named_labels
            .iter()
            .map(|(name, id)| (name.as_str(), *id))
    }

    pub fn get_defined_labels(&self) -> impl Iterator<Item = (LabelId, Offset64)> {
        self.labels.iter().filter_map(|(id, info)| match info {
            LabelInfo::Offset(offset) => Some((*id, *offset)),
            LabelInfo::Forward => None,
        })
    }
    
    fn next_label(&mut self) -> LabelId {
        let id = LabelId(self.next_id);
        self.next_id += 1;
        id
    }
}
