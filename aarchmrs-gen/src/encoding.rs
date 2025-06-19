/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use std::rc::Rc;

use aarchmrs_parser::instructions::{Encodeset, Range};

const INSTRUCTION_BIT_WIDTH: usize = 32;

type InstructionBits = [Option<Bits>; INSTRUCTION_BIT_WIDTH];

/// This function combines the definitions from root (first element) to child (last element),
/// producing a new `Encodeset`.
///
/// The AARCHMRS provides tree-like definition of Encodesets where a parent node defines
/// common fields, and a child defines more specific fields (sometimes overriding parent's
/// bits).
pub fn flatten_encodeset(encodings: &[&Encodeset]) -> Vec<Bits> {
    use aarchmrs_parser::instructions::Encode;

    let mut instruction_bits: InstructionBits = [const { None }; INSTRUCTION_BIT_WIDTH];

    for encset in encodings {
        for enc in *encset {
            match enc {
                Encode::Field(field) => fill_field(&mut instruction_bits, field),
                Encode::Bits(bits) => fill_bits(&mut instruction_bits, bits),
            }
        }
    }

    regroup_back(&instruction_bits)
}

fn regroup_back(instruction_bits: &InstructionBits) -> Vec<Bits> {
    let mut last_encode: Option<Bits> = None;
    let mut values: Vec<Bits> = vec![];

    for src in instruction_bits {
        match (src, last_encode.take()) {
            (None, None) => {}
            (None, Some(enc)) => {
                values.push(enc);
            }
            (Some(bit), None) => {
                last_encode = Some(bit.clone());
            }
            (
                Some(Bits::Bit { bits: src_bits, .. }),
                Some(Bits::Bit {
                    mut bits,
                    mut range,
                }),
            ) => {
                bits |= src_bits << range.width;
                range.width += 1;
                last_encode = Some(Bits::Bit { bits, range });
            }
            (Some(Bits::Bit { .. }), Some(Bits::Field { name, range })) => {
                values.push(Bits::Field { name, range });
                last_encode = src.clone();
            }
            (
                Some(Bits::Field {
                    name: src_name,
                    range: src_range,
                }),
                Some(Bits::Field { name, mut range }),
            ) => {
                if src_name == &name {
                    // combine
                    range.width += 1;
                    last_encode = Some(Bits::Field { name, range });
                } else {
                    // evict older value
                    values.push(Bits::Field { name, range });
                    last_encode = Some(Bits::Field {
                        name: src_name.clone(),
                        range: *src_range,
                    });
                }
            }
            (Some(Bits::Field { .. }), Some(Bits::Bit { bits, range })) => {
                values.push(Bits::Bit { bits, range });
                last_encode = src.clone();
            }
        }
    }

    if let Some(last) = last_encode {
        values.push(last);
    }

    values
}

fn fill_bits(instruction_bits: &mut InstructionBits, bits: &aarchmrs_parser::instructions::Bits) {
    for (bit_char, bit_idx) in bits
        .value
        .as_str()
        .expect("string value")
        .chars()
        .rev()
        .zip(bits.range)
    {
        let range = Range {
            start: bit_idx,
            width: 1,
        };
        let bit = match bit_char {
            '0' => Some(Bits::Bit { bits: 0, range }),
            '1' => Some(Bits::Bit { bits: 1, range }),
            'x' => None,
            _ => panic!("unexpected char {bit_char:?}"),
        };
        instruction_bits[bit_idx as usize] = bit;
    }
}

fn fill_field(
    instruction_bits: &mut InstructionBits,
    field: &aarchmrs_parser::instructions::Field,
) {
    assert!(
        field
            .value
            .as_str()
            .expect("string value")
            .chars()
            .all(|c| c == 'x')
    );
    let name: Rc<str> = field.name.as_str().into();

    for pos_idx in field.range {
        instruction_bits[pos_idx as usize] = Some(Bits::Field {
            name: name.clone(),
            range: Range {
                start: pos_idx,
                width: 1,
            },
        });
    }
}

#[derive(Debug, Clone)]
pub enum Bits {
    Bit { bits: u32, range: Range },
    Field { name: Rc<str>, range: Range },
}
