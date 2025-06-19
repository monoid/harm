use std::rc::Rc;

use aarchmrs_parser::Encodeset;

const INSTRUCTION_BIT_WIDTH: usize = 32;

type InstructionBits = [Option<Bit>; INSTRUCTION_BIT_WIDTH];

/// This function combines the definitions from root (first element) to child (last element),
/// producing a new `Encodeset`.
///
/// The AARCHMRS provides tree-like definition of Encodesets where a parent node defines
/// common fields, and a child defines more specific fields (sometimes overriding parent's
/// bits).
pub fn flatten_encodeset(encodings: &[&Encodeset]) -> Encodeset {
    use aarchmrs_parser::Encode;

    let mut instruction_bits: InstructionBits = [const { None }; INSTRUCTION_BIT_WIDTH];

    for encset in encodings {
        for enc in *encset {
            match enc {
                Encode::Field(field) => fill_field(&mut instruction_bits, field),
                Encode::Bits(bits) => fill_bits(&mut instruction_bits, bits),
            }
        }
    }

    group_back(&instruction_bits)
}

fn group_back(instruction_bits: &InstructionBits) -> Encodeset {
    use itertools::Itertools as _;
    let empty: Rc<str> = <_>::from("");

    let groups = instruction_bits.iter().chunk_by(|item| match item {
        None => (0, empty.clone()),
        Some(Bit::Bit(_)) => (1, empty.clone()),
        Some(Bit::Field { name }) => (2, name.clone()),
    });

    todo!()
}

fn fill_bits(instruction_bits: &mut InstructionBits, bits: &aarchmrs_parser::Bits) {
    for (bit_char, bit_idx) in bits
        .value
        .as_str()
        .expect("string value")
        .chars()
        .rev()
        .zip(bits.range)
    {
        match bit_char {
            '0' => instruction_bits[bit_idx as usize] = Some(Bit::Bit(false)),
            '1' => instruction_bits[bit_idx as usize] = Some(Bit::Bit(true)),
            'x' => {}
            _ => panic!("unexpected char {bit_char:?}"),
        }
    }
}

fn fill_field(instruction_bits: &mut InstructionBits, field: &aarchmrs_parser::Field) {
    assert!(
        field
            .value
            .as_str()
            .expect("string value")
            .chars()
            .all(|c| c == 'x')
    );
    let name: Rc<str> = field.name.as_str().into();
    let bit = Some(Bit::Field { name: name.clone() });

    for pos_idx in field.range {
        instruction_bits[pos_idx as usize] = bit.clone();
    }
}

#[derive(Debug, Clone)]
enum Bit {
    Bit(bool),
    Field { name: Rc<str> },
}
