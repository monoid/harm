//! Simple AARCHMRS parser
//!
//! It extracts ARM instruction info (`Instructions.json`) from the ARM's AARCHMRS open source archive.
//! It extracts only minimal information required for the code generation of the `aarchmrs-instructions` crate.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Instructions {
    pub instructions: Vec<InstructionSet>,
}

#[derive(Debug, Deserialize)]
pub struct InstructionSet {
    pub children: Vec<InstructionGroup>,
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

#[derive(Debug, Deserialize)]
pub struct Range {
    pub start: u32,
    pub width: u32,
}

#[derive(Debug, Deserialize)]
pub struct Value {
    pub value: String,
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
