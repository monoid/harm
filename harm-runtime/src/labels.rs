/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use std::collections::HashMap;

use harm::reloc::{LabelId, Offset};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LabelInfo {
    Forward,
    // TODO segment
    Offset(Offset),
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
    pub fn forward_named_label(&mut self, name: &str) -> LabelId {
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

    pub fn define_label(&mut self, label_id: LabelId, offset: Offset) {
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
            panic!("Label {label_id:?} is not registered");
        }
    }

    #[inline]
    pub fn define_named_label(&mut self, name: &str, offset: Offset) -> LabelId {
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

    pub fn name_label(&mut self, id: LabelId, name: &str) {
        if self.labels.contains_key(&id) {
            self.named_labels.insert(name.to_string(), id);
        } else {
            panic!("Label {id:?} is not registered");
        }
    }

    #[inline]
    pub fn label_info(&self, id: LabelId) -> Option<&LabelInfo> {
        self.labels.get(&id)
    }

    fn next_label(&mut self) -> LabelId {
        let id = LabelId(self.next_id);
        self.next_id += 1;
        id
    }
}
