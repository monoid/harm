use crate::encoding::Bits;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, Pat, PatIdent, PatType, parse_quote};

pub fn gen_constructor(name: &str, desc: &[Bits]) -> TokenStream {
    let args = gen_constructor_args(desc);
    let expr = gen_expr(desc);

    let fn_name = format_ident!("{}", name);
    let expanded = quote! {
        #[inline]
        pub fn #fn_name(#(#args),*) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(#expr)
        }
    };

    expanded
}

fn gen_constructor_args(desc: &[Bits]) -> impl Iterator<Item = syn::FnArg> {
    let args = desc.iter().filter_map(|bits| match bits {
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
            ty: syn::parse_str(&format!("::aarchmrs_types::BitValue::<{}>", range.width))
                .expect("internal error: malformed type"),
        })),
    });
    args
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
                parse_quote!(u32::from(#name) << #offset)
            }
        })
        .reduce(|e1: Expr, e2: Expr| parse_quote!(#e1 | #e2))
        .unwrap_or(parse_quote!(0))
}

#[cfg(test)]
mod tests {
    use aarchmrs_parser::instructions::Range;

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
                "#[inline]\n",
                "pub fn NOP_HI_hints() -> ::aarchmrs_types::InstructionCode {\n",
                "    ::aarchmrs_types::InstructionCode::from_u32(\n",
                "        0b11010101000000110010000000011111u32 << 0u32,\n",
                "    )\n",
                "}\n",
            )
        );
    }

    #[test]
    fn test_add() {
        let add = 0b0001011001;
        let nop_bits = vec![
            Bits::Field {
                name: "s".into(),
                range: Range {
                    start: 31,
                    width: 1,
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
                name: "Rm".into(),
                range: Range {
                    start: 16,
                    width: 5,
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
                name: "im3".into(),
                range: Range {
                    start: 10,
                    width: 3,
                },
            },
            Bits::Field {
                name: "Rn".into(),
                range: Range { start: 5, width: 5 },
            },
            Bits::Field {
                name: "Rd".into(),
                range: Range { start: 0, width: 5 },
            },
        ];

        let code = gen_constructor("ADD_64_addsub_shift", &nop_bits);
        let code = pretty(code);

        assert_eq!(
            code,
            concat!(
                "#[inline]\n",
                "pub fn ADD_64_addsub_shift(\n",
                "    s: ::aarchmrs_types::BitValue<1>,\n",
                "    Rm: ::aarchmrs_types::BitValue<5>,\n",
                "    option: ::aarchmrs_types::BitValue<3>,\n",
                "    im3: ::aarchmrs_types::BitValue<3>,\n",
                "    Rn: ::aarchmrs_types::BitValue<5>,\n",
                "    Rd: ::aarchmrs_types::BitValue<5>,\n",
                ") -> ::aarchmrs_types::InstructionCode {\n",
                "    ::aarchmrs_types::InstructionCode::from_u32(\n",
                "        u32::from(s) << 31u32 | 0b1011001u32 << 21u32 | u32::from(Rm) << 16u32\n",
                "            | u32::from(option) << 13u32 | u32::from(im3) << 10u32\n",
                "            | u32::from(Rn) << 5u32 | u32::from(Rd) << 0u32,\n",
                "    )\n",
                "}\n",
            )
        );
    }
}
