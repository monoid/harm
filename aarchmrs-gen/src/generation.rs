/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::encoding::Bits;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, Pat, PatIdent, PatType, parse_quote};

pub fn gen_constructor(name: &str, desc: &[Bits]) -> TokenStream {
    let args: Vec<_> = gen_constructor_args(desc).collect();
    let (fields, inits) = gen_fields(desc);
    let expr = gen_expr(desc);

    let fn_name = format_ident!("{}", name);
    let expanded = quote! {
        #[derive(Copy, Clone, Debug, Default)]
        pub struct #fn_name {
            #(#fields),*
        }

        impl #fn_name {
            #[inline]
            pub fn new(#(#args),*) -> Self {
                Self { #(#inits),* }
            }

            #[inline]
            pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
                ::aarchmrs_types::InstructionCode::from_u32(#expr)
            }

        }
    };

    expanded
}

fn gen_constructor_args(desc: &[Bits]) -> impl Iterator<Item = syn::FnArg> {
    let args = desc.iter().rev().filter_map(|bits| match bits {
        Bits::Bit { .. } => None,
        Bits::Field { name, range } => Some(syn::FnArg::Typed(PatType {
            attrs: vec![],
            pat: Box::new(Pat::Ident(PatIdent {
                attrs: vec![],
                by_ref: None,
                mutability: None,
                ident: format_ident!("{}", name.as_ref()),
                subpat: None,
            })),
            colon_token: <_>::default(),
            ty: syn::parse_str(&format!(
                "impl Into<::aarchmrs_types::BitValue::<{}>>",
                range.width
            ))
            .expect("internal error: malformed type"),
        })),
    });
    args
}

fn gen_fields(desc: &[Bits]) -> (Vec<syn::Field>, Vec<syn::FieldValue>) {
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
                inits.push(parse_quote!(#field: #field.into()));
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
                parse_quote!(u32::from(self.#name) << #offset)
            }
        })
        .rev()
        .reduce(|e1: Expr, e2: Expr| parse_quote!(#e1 | #e2))
        .unwrap_or(parse_quote!(0))
}

#[cfg(test)]
mod tests {
    use aarchmrs_parser::instructions::Range;
    use pretty_assertions::assert_eq;

    use super::*;

    fn pretty(code: TokenStream) -> String {
        let file: syn::File = parse_quote!(#code);
        let code = prettyplease::unparse(&file);
        code
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

        let code = pretty(gen_constructor("NOP_HI_hints", &nop_bits));
        assert_eq!(
            code,
            concat!(
                "#[derive(Copy, Clone, Debug, Default)]\n",
                "pub struct NOP_HI_hints {}\n",
                "impl NOP_HI_hints {\n",
                "    #[inline]\n",
                "    pub fn new() -> Self {\n",
                "        Self {}\n",
                "    }\n",
                "    #[inline]\n",
                "    pub fn build(&self) -> ::aarchmrs_types::InstructionCode {\n",
                "        ::aarchmrs_types::InstructionCode::from_u32(\n",
                "            0b11010101000000110010000000011111u32 << 0u32,\n",
                "        )\n",
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

        let code = gen_constructor("ADD_64_addsub_shift", &nop_bits);
        let code = pretty(code);

        assert_eq!(
            code,
            concat!(
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
                "    pub fn new(\n",
                "        s: impl Into<::aarchmrs_types::BitValue<1>>,\n",
                "        Rm: impl Into<::aarchmrs_types::BitValue<5>>,\n",
                "        option: impl Into<::aarchmrs_types::BitValue<3>>,\n",
                "        im3: impl Into<::aarchmrs_types::BitValue<3>>,\n",
                "        Rn: impl Into<::aarchmrs_types::BitValue<5>>,\n",
                "        Rd: impl Into<::aarchmrs_types::BitValue<5>>,\n",
                "    ) -> Self {\n",
                "        Self {\n",
                "            s: s.into(),\n",
                "            Rm: Rm.into(),\n",
                "            option: option.into(),\n",
                "            im3: im3.into(),\n",
                "            Rn: Rn.into(),\n",
                "            Rd: Rd.into(),\n",
                "        }\n",
                "    }\n",
                "    #[inline]\n",
                "    pub fn build(&self) -> ::aarchmrs_types::InstructionCode {\n",
                "        ::aarchmrs_types::InstructionCode::from_u32(\n",
                "            u32::from(self.s) << 31u32 | 0b0001011001u32 << 21u32\n",
                "                | u32::from(self.Rm) << 16u32 | u32::from(self.option) << 13u32\n",
                "                | u32::from(self.im3) << 10u32 | u32::from(self.Rn) << 5u32\n",
                "                | u32::from(self.Rd) << 0u32,\n",
                "        )\n",
                "    }\n",
                "}\n",
            )
        );
    }
}
