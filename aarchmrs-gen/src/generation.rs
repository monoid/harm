/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::encoding::Bits;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, Pat, PatIdent, PatType, parse_quote};

struct Masks {
    mask: u32,
    opcode: u32,
}

pub fn gen_constructor(name: &str, desc: &[Bits], should_be_mask: u32) -> TokenStream {
    let args: Vec<_> = gen_constructor_args(desc).collect();
    let (fields, inits) = gen_struct_fields(desc);
    let expr = gen_expr(desc);

    let Masks { mask, opcode } = gen_mask(desc);

    let fmt_name = format_ident!("{}", name);
    let mask: syn::LitInt = syn::parse_str(&format!("0b{:0w$b}u32", mask, w = 32))
        .expect("internal error: malformed mask");
    let opcode: syn::LitInt = syn::parse_str(&format!("0b{:0w$b}u32", opcode, w = 32))
        .expect("internal error: malformed opcode");
    let should_be_mask: syn::LitInt =
        syn::parse_str(&format!("0b{:0w$b}u32", should_be_mask, w = 32))
            .expect("internal error: malformed should_be_mask");
    let expanded = quote! {
        pub const OPCODE_MASK: u32 = #mask;
        pub const OPCODE: u32 = #opcode;
        pub const SHOULD_BE_MASK: u32 = #should_be_mask;
        pub const NAME: &str = #name;

        #[derive(Copy, Clone, Debug, Default)]
        pub struct #fmt_name {
            #(#fields),*
        }

        impl #fmt_name {
            #[inline]
            pub const fn new(#(#args),*) -> Self {
                Self { #(#inits),* }
            }

            #[inline]
            pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
                ::aarchmrs_types::InstructionCode::from_u32(#expr)
            }

            #[inline]
            pub const fn opcode_mask() -> u32 {
                self::OPCODE_MASK
            }

            #[inline]
            pub const fn opcode() -> u32 {
                self::OPCODE
            }

            #[inline]
            pub const fn should_be_mask() -> u32 {
                self::SHOULD_BE_MASK
            }

            #[inline]
            pub const fn match_opcode(opcode: u32) -> bool {
                let opcode = opcode & self::OPCODE_MASK;
                opcode == self::OPCODE
            }

            #[inline]
            pub const fn name() -> &'static str {
                self::NAME
            }
        }
    };

    expanded
}

fn gen_constructor_args(desc: &[Bits]) -> impl Iterator<Item = syn::FnArg> {
    desc.iter().rev().filter_map(|bits| match bits {
        Bits::Bit { .. } => None,
        Bits::Field { name, range } => field_to_fnarg(name, range),
    })
}

fn field_to_fnarg(name: &str, range: &aarchmrs_parser::instructions::Range) -> Option<syn::FnArg> {
    Some(syn::FnArg::Typed(PatType {
        attrs: vec![],
        pat: Box::new(Pat::Ident(PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: format_ident!("{}", name),
            subpat: None,
        })),
        colon_token: <_>::default(),
        ty: syn::parse_str(&format!("::aarchmrs_types::BitValue::<{}>", range.width))
            .expect("internal error: malformed type"),
    }))
}

fn gen_struct_fields(desc: &[Bits]) -> (Vec<syn::Field>, Vec<syn::FieldValue>) {
    let mut fields = vec![];
    let mut inits = vec![];
    for bits in desc.iter().rev() {
        match bits {
            Bits::Bit { .. } => {}
            Bits::Field { name, range } => {
                let field = format_ident!("{}", **name);
                let ty: syn::Type =
                    syn::parse_str(&format!("::aarchmrs_types::BitValue::<{}>", range.width))
                        .expect("internal error: malformed type");
                fields.push(parse_quote!(
                    pub #field: #ty
                ));
                inits.push(parse_quote!(#field));
            }
        }
    }
    (fields, inits)
}

fn gen_expr(desc: &[Bits]) -> syn::Expr {
    desc.iter()
        .map(|bits| match bits {
            Bits::Bit { bits, range } => {
                let bits: syn::Expr =
                    syn::parse_str(&format!("0b{:0w$b}u32", bits, w = range.width as usize))
                        .expect("internal error: malformed integer");
                let offset = range.start;
                parse_quote!(#bits << #offset)
            }
            Bits::Field { name, range } => {
                let name = format_ident!("{}", name.as_ref());
                let offset = range.start;
                parse_quote!(self.#name.into_inner() << #offset)
            }
        })
        .rev()
        .reduce(|e1: Expr, e2: Expr| parse_quote!(#e1 | #e2))
        .unwrap_or(parse_quote!(0))
}

fn gen_mask(desc: &[Bits]) -> Masks {
    let mut mask = 0u32;
    let mut encoding = 0u32;

    for bits in desc.iter() {
        match bits {
            Bits::Bit { bits, range } => {
                eprintln!("width: {}, start: {}", range.width, range.start);
                mask |= 1u32.unbounded_shl(range.width).wrapping_sub(1) << range.start;
                encoding |= bits << range.start;
            }
            Bits::Field { .. } => {}
        }
    }
    Masks {
        mask,
        opcode: encoding,
    }
}

#[cfg(test)]
mod tests {
    use aarchmrs_parser::instructions::Range;
    use pretty_assertions::assert_eq;

    use super::*;

    fn pretty(code: TokenStream) -> String {
        let file: syn::File = parse_quote!(#code);

        prettyplease::unparse(&file)
    }

    #[test]
    fn test_noop() {
        let nop = 0b11010101000000110010000000011111u32;
        let nop_bits = vec![Bits::Bit {
            bits: nop,
            range: Range {
                start: 0,
                width: 32,
            },
        }];

        let code = pretty(gen_constructor("NOP_HI_hints", &nop_bits, 0));
        assert_eq!(
            code,
            concat!(
                "pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;\n",
                "pub const OPCODE: u32 = 0b11010101000000110010000000011111u32;\n",
                "pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;\n",
                "pub const NAME: &str = \"NOP_HI_hints\";\n",
                "#[derive(Copy, Clone, Debug, Default)]\n",
                "pub struct NOP_HI_hints {}\n",
                "impl NOP_HI_hints {\n",
                "    #[inline]\n",
                "    pub const fn new() -> Self {\n",
                "        Self {}\n",
                "    }\n",
                "    #[inline]\n",
                "    pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {\n",
                "        ::aarchmrs_types::InstructionCode::from_u32(\n",
                "            0b11010101000000110010000000011111u32 << 0u32,\n",
                "        )\n",
                "    }\n",
                "    #[inline]\n",
                "    pub const fn opcode_mask() -> u32 {\n",
                "        self::OPCODE_MASK\n",
                "    }\n",
                "    #[inline]\n",
                "    pub const fn opcode() -> u32 {\n",
                "        self::OPCODE\n",
                "    }\n",
                "    #[inline]\n",
                "    pub const fn should_be_mask() -> u32 {\n",
                "        self::SHOULD_BE_MASK\n",
                "    }\n",
                "    #[inline]\n",
                "    pub const fn match_opcode(opcode: u32) -> bool {\n",
                "        let opcode = opcode & self::OPCODE_MASK;\n",
                "        opcode == self::OPCODE\n",
                "    }\n",
                "    #[inline]\n",
                "    pub const fn name() -> &'static str {\n",
                "        self::NAME\n",
                "    }\n",
                "}\n",
            )
        );
    }

    #[test]
    fn test_add() {
        let add = 0b0001011001;
        let nop_bits = vec![
            Bits::Field {
                name: "Rd".into(),
                range: Range { start: 0, width: 5 },
            },
            Bits::Field {
                name: "Rn".into(),
                range: Range { start: 5, width: 5 },
            },
            Bits::Field {
                name: "im3".into(),
                range: Range {
                    start: 10,
                    width: 3,
                },
            },
            Bits::Field {
                name: "option".into(),
                range: Range {
                    start: 13,
                    width: 3,
                },
            },
            Bits::Field {
                name: "Rm".into(),
                range: Range {
                    start: 16,
                    width: 5,
                },
            },
            Bits::Bit {
                bits: add,
                range: Range {
                    start: 21,
                    width: 10,
                },
            },
            Bits::Field {
                name: "s".into(),
                range: Range {
                    start: 31,
                    width: 1,
                },
            },
        ];

        let code = gen_constructor("ADD_64_addsub_shift", &nop_bits, 0);
        let code = pretty(code);

        assert_eq!(
            code,
            concat!(
                "pub const OPCODE_MASK: u32 = 0b01111111111000000000000000000000u32;\n",
                "pub const OPCODE: u32 = 0b00001011001000000000000000000000u32;\n",
                "pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;\n",
                "pub const NAME: &str = \"ADD_64_addsub_shift\";\n",
                "#[derive(Copy, Clone, Debug, Default)]\n",
                "pub struct ADD_64_addsub_shift {\n",
                "    pub s: ::aarchmrs_types::BitValue<1>,\n",
                "    pub Rm: ::aarchmrs_types::BitValue<5>,\n",
                "    pub option: ::aarchmrs_types::BitValue<3>,\n",
                "    pub im3: ::aarchmrs_types::BitValue<3>,\n",
                "    pub Rn: ::aarchmrs_types::BitValue<5>,\n",
                "    pub Rd: ::aarchmrs_types::BitValue<5>,\n",
                "}\n",
                "impl ADD_64_addsub_shift {\n",
                "    #[inline]\n",
                "    pub const fn new(\n",
                "        s: ::aarchmrs_types::BitValue<1>,\n",
                "        Rm: ::aarchmrs_types::BitValue<5>,\n",
                "        option: ::aarchmrs_types::BitValue<3>,\n",
                "        im3: ::aarchmrs_types::BitValue<3>,\n",
                "        Rn: ::aarchmrs_types::BitValue<5>,\n",
                "        Rd: ::aarchmrs_types::BitValue<5>,\n",
                "    ) -> Self {\n",
                "        Self { s, Rm, option, im3, Rn, Rd }\n",
                "    }\n",
                "    #[inline]\n",
                "    pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {\n",
                "        ::aarchmrs_types::InstructionCode::from_u32(\n",
                "            self.s.into_inner() << 31u32 | 0b0001011001u32 << 21u32\n",
                "                | self.Rm.into_inner() << 16u32 | self.option.into_inner() << 13u32\n",
                "                | self.im3.into_inner() << 10u32 | self.Rn.into_inner() << 5u32\n",
                "                | self.Rd.into_inner() << 0u32,\n",
                "        )\n",
                "    }\n",
                "    #[inline]\n",
                "    pub const fn opcode_mask() -> u32 {\n",
                "        self::OPCODE_MASK\n",
                "    }\n",
                "    #[inline]\n",
                "    pub const fn opcode() -> u32 {\n",
                "        self::OPCODE\n",
                "    }\n",
                "    #[inline]\n",
                "    pub const fn should_be_mask() -> u32 {\n",
                "        self::SHOULD_BE_MASK\n",
                "    }\n",
                "    #[inline]\n",
                "    pub const fn match_opcode(opcode: u32) -> bool {\n",
                "        let opcode = opcode & self::OPCODE_MASK;\n",
                "        opcode == self::OPCODE\n",
                "    }\n",
                "    #[inline]\n",
                "    pub const fn name() -> &'static str {\n",
                "        self::NAME\n",
                "    }\n",
                "}\n",
            )
        );
    }
}
