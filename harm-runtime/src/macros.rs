/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

/// A helper macro to append multiple instructions to the assembler, and return the first error if any of them fails.
#[macro_export]
macro_rules! harm {
    ($asm:expr; $($inst:expr),*) => {
        {
            let asm = &mut $asm;
            'block: {
                $(
                    if let Err(e) = asm.append($inst) {
                        break 'block Err(e);
                    }
                )*
                Ok(())
            }
        }
    };

    ($asm:expr; $($inst:expr,)*) => {
        harm!{ $asm; $($inst),* }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "memmap2")]
    fn test_harm_macro() -> Result<(), Box<dyn std::error::Error>> {
        use crate::*;
        use harm::{
            instructions::{dpimm::movz, logical::and},
            register::Reg64,
        };

        use crate::memory::MmapBuffer;

        let mem = MmapBuffer::allocate(16).unwrap();
        let mut asm = Assembler::new(mem);

        harm! {
            asm;
            movz(Reg64::X0, 0),
            and(Reg64::X3, Reg64::X3, 8)?  // instruction-level error.
        }
        .unwrap(); // memory buffer level error.

        asm.build::<_, ()>().unwrap();

        Ok(())
    }
}
