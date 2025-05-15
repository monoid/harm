//! Simple AARCHMRS parser for `Instructions.json`.
//!
//! It extracts ARM instruction info (`Instructions.json`) from the ARM's AARCHMRS open source archive.
//! It extracts only minimal information required for the code generation of the `aarchmrs-instructions` crate.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Instructions {
    pub instructions: Vec<InstructionSet>,
    pub _meta: Meta,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub license: License,
}

#[derive(Debug, Deserialize)]
pub struct License {
    pub copyright: String,
    pub info: String,
}

#[derive(Debug, Deserialize)]
pub struct InstructionSet {
    pub children: Vec<InstructionGroupOrInstruction>,
    pub encoding: Encodeset,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct InstructionGroup {
    pub children: Vec<InstructionGroupOrInstruction>,
    pub encoding: Encodeset,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "_type")]
pub enum InstructionGroupOrInstruction {
    #[serde(rename = "Instruction.InstructionGroup")]
    InstructionGroup(InstructionGroup),
    #[serde(rename = "Instruction.Instruction")]
    Instruction(Instruction),
    #[serde(rename = "Instruction.InstructionAlias")]
    InstructionAlias(InstructionAlias),
}

#[derive(Debug, Deserialize)]
pub struct Encodeset {
    pub values: Vec<Encode>,
}

impl<'a> IntoIterator for &'a Encodeset {
    type Item = &'a Encode;

    type IntoIter = std::slice::Iter<'a, Encode>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "_type")]
pub enum Encode {
    #[serde(rename = "Instruction.Encodeset.Field")]
    Field(Field),
    #[serde(rename = "Instruction.Encodeset.Bits")]
    Bits(Bits),
}

#[derive(Debug, Deserialize)]
pub struct Field {
    pub name: String,
    pub range: Range,
    pub should_be_mask: Value,
    pub value: Value,
}

#[derive(Debug, Deserialize)]
pub struct Bits {
    pub range: Range,
    pub should_be_mask: Value,
    pub value: Value,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Range {
    pub start: u32,
    pub width: u32,
}

impl IntoIterator for Range {
    type Item = u32;

    type IntoIter = std::ops::Range<u32>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..(self.start + self.width)
    }
}

#[derive(Debug, Deserialize)]
pub struct Value {
    // there is a "meaning" field, but it is always null.
    pub value: String,
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        if self.value.starts_with('\'') && self.value.ends_with('\'') {
            let val = self.value.as_str();
            Some(val.split_at(val.len() - 1).0.split_at(1).1)
        } else {
            None
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Instruction {
    pub encoding: Encodeset,
    pub name: String,
    pub operation_id: String,
    #[serde(default)]
    pub children: Vec<InstructionGroupOrInstruction>,
}

#[derive(Debug, Deserialize)]
pub struct InstructionAlias {
    // TODO what are the real fields?
    pub name: String,
    pub operation_id: String,
    #[serde(default)]
    pub children: Vec<InstructionGroupOrInstruction>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_as_str_empty() {
        assert_eq!(Value { value: "''".into() }.as_str(), Some(""));
    }

    #[test]
    fn test_value_as_str() {
        assert_eq!(
            Value {
                value: "'000'".into()
            }
            .as_str(),
            Some("000")
        );
    }
}
