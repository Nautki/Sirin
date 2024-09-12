#![allow(unused)]

use std::{any::TypeId, cmp::Ordering, collections::HashMap, panic::catch_unwind};

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{braced, bracketed, parenthesized, parse::{Parse, ParseStream}, parse_quote, token::{Brace, Bracket, Paren, Token}, Attribute, Ident, LitInt, Result, Token, Type};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(dev);
    custom_keyword!(regs);
}


#[derive(Clone)]
pub struct Periph {
    pub name: Ident,
    pub addr_ty: Type,
    pub word_ty: Type,
    pub regs: Vec<Reg>,
    pub vars: HashMap<String, Var>,
    pub attr: Vec<Attribute>,
    pub keywords: Vec<TokenStream>,
}

impl Parse for Periph {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut keywords = Vec::new();
        let attr = input.call(Attribute::parse_outer)?;

        keywords.push(input.parse::<kw::dev>()?.into_token_stream());

        let name = input.parse::<Ident>()?;

        let (addr_ty, word_ty) = if input.peek(Paren) {
            let inside;
            parenthesized!(inside in input);
            let addr_ty = inside.parse::<Type>()?;
            inside.parse::<Token![,]>()?;
            (addr_ty, inside.parse::<Type>()?)
        } else {
            let addr_ty: Type = parse_quote!(u8);
            (addr_ty.clone(), addr_ty.clone())
        };

        let inside;
        braced!(inside in input);
        let input = &inside;

        keywords.push(input.parse::<kw::regs>()?.into_token_stream());

        let inside;
        braced!(inside in input);
        let input = &inside;

        let regs: Vec<Reg> = input.parse_terminated(Reg::parse, Token![,])?.into_iter().collect();
        
        let mut vars = HashMap::<String, Var>::new();
        for reg in &regs {
            for var_part in &reg.vars {
                match vars.get_mut(&var_part.var.to_string()) {
                    Some(var) => {
                        var.access = var.access & var_part.access;
                        var.parts.push(var_part.clone());
                    },
                    None => {
                        vars.insert(var_part.var.to_string(), Var {
                            access: var_part.access,
                            parts: vec![var_part.clone()],
                            ty: Type::Verbatim(Default::default())
                        });
                    }
                };
            }
        }

        for (name, var) in &mut vars {
            var.parts.sort_unstable_by(|a, b| {
                match a.var_range.partial_cmp(&b.var_range) {
                    Some(ord) => ord,
                    None => panic!("The bits for {} overlap somewhere", name)
                }
            });

            let last = var.parts.last().unwrap();
            var.ty = match (&last.reg_range, &last.var_range) {
                (BitRange::Single(_), BitRange::Entire) => parse_quote!(bool),
                _ => match last.var_range.end() {
                    Some(end) => nearest_prim(end + 1),
                    _ => word_ty.clone()
                }
            };
        }
        
        Ok(Periph {
            name,
            addr_ty,
            word_ty,
            regs,
            vars,
            attr,
            keywords
        })
    }
}

#[derive(Clone)]
pub struct Reg {
    pub attr: Vec<Attribute>,
    pub addr: LitInt,
    pub reg: Ident,
    pub access: Access,
    pub vars: Vec<VarPart>
}

impl Parse for Reg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attr = input.call(Attribute::parse_outer)?;
        let addr = input.parse::<LitInt>()?;
        let reg = input.parse::<Ident>()?;
        let access = input.parse()?;

        let vars = if input.peek(Ident) {
            let var = input.parse::<Ident>()?;

            let var_range = BitRange::parse_optional_bracketed(input)?;

            vec![VarPart {
                attr: attr.clone(),
                reg: reg.clone(),
                reg_range: BitRange::Entire,
                reg_addr: addr.clone(),
                access,
                var,
                var_range
            }]
        } else if input.peek(Brace) {
            let inside;
            braced!(inside in input);

            let reg_parts = inside.parse_terminated(OneRegPart::parse, Token![,])?;

            reg_parts.into_iter().map(|part| VarPart {
                attr: part.attr,
                reg: reg.clone(),
                reg_range: part.reg_range,
                reg_addr: addr.clone(),
                access: part.access.unwrap_or(access),
                var: part.var,
                var_range: part.var_range
            }).collect()
        } else {
            vec![]
        };

        Ok(Reg {
            attr,
            addr,
            reg,
            access,
            vars
        })
    }
}

#[derive(Clone)]
pub struct OneRegPart {
    pub attr: Vec<Attribute>,
    pub reg_range: BitRange,
    pub access: Option<Access>,
    pub var: Ident,
    pub var_range: BitRange
}

impl Parse for OneRegPart {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attr = input.call(Attribute::parse_outer)?;
        let reg_range = input.parse()?;
        let access = if input.fork().parse::<Access>().is_ok() {
            Some(input.parse()?)
        } else {
            None
        };
        let var = input.parse()?;
        let var_range = BitRange::parse_optional_bracketed(input)?;

        Ok(OneRegPart {
            attr,
            reg_range,
            access,
            var,
            var_range
        })
    }
}

#[derive(Clone)]
pub struct Var {
    pub access: Access,
    pub parts: Vec<VarPart>,
    pub ty: Type
}

#[derive(Clone)]

pub struct VarPart {
    pub attr: Vec<Attribute>,
    pub reg: Ident,
    pub reg_range: BitRange,
    pub reg_addr: LitInt,
    pub access: Access,
    pub var: Ident,
    pub var_range: BitRange
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BitRange {
    Single(usize),
    Range(usize, usize),
    Entire
}

impl Parse for BitRange {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek2(Token![..]) {
            let first = input.parse::<LitInt>()?;
            input.parse::<Token![..]>()?;
            Ok(BitRange::Range(first.base10_parse()?, input.parse::<LitInt>()?.base10_parse()?))
        } else {
            Ok(BitRange::Single(input.parse::<LitInt>()?.base10_parse()?))
        }
    }
}

impl PartialOrd for BitRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.end() < other.start() {
            Some(Ordering::Less)
        } else if other.end() < self.start() {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl BitRange {
    pub fn parse_optional_bracketed(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Bracket) {
            let inside;
            bracketed!(inside in input);
            inside.parse()
        } else {
            Ok(BitRange::Entire)
        }
    }

    pub fn start(&self) -> Option<usize> {
        match self {
            Self::Range(start, _) => Some(*start),
            Self::Single(start) => Some(*start),
            Self::Entire => None
        }
    }

    pub fn end(&self) -> Option<usize> {
        match self {
            Self::Range(_, end) => Some(*end),
            Self::Single(end) => Some(*end),
            Self::Entire => None
        }
    }
}

/*
impl From<BitRange> for (u8, u8) {
    fn from(value: BitRange) -> Self {
        match value {
            BitRange::Single(b) => (b as u8, b as u8),
            BitRange::Range(a, b) => (a as u8, b as u8)
        }
    }
}
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Access {
    pub readable: bool,
    pub writable: bool
}

impl std::ops::BitAnd for Access {
    type Output = Access;

    fn bitand(self, rhs: Self) -> Self::Output {
        Access {
            readable: self.readable && rhs.readable,
            writable: self.writable && rhs.writable
        }
    }
}

impl Parse for Access {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        match ident.to_string().as_str() {
            "r" => Ok(Access {
                readable: true,
                writable: false,
            }),
            "w" => Ok(Access {
                readable: false,
                writable: true,
            }),
            "rw" => Ok(Access {
                readable: true,
                writable: true,
            }),
            _ => Err(syn::Error::new(ident.span(), "Expected 'r', 'w', or 'rw'"))
        }
    }
}

fn exact_prim(num_bits: usize) -> Type {
    match num_bits {
        8 => parse_quote! { u8 },
        16 => parse_quote! { u16 },
        32 => parse_quote! { u32 },
        64 => parse_quote! { u64 },
        128 => parse_quote! { u128 },
        _ => parse_quote! { compile_error!("Unsupported bit width") }
    }
}

fn nearest_prim(num_bits: usize) -> Type {
    match num_bits {
        0..=8 => parse_quote! { u8 },
        ..=16 => parse_quote! { u16 },
        ..=32 => parse_quote! { u32 },
        ..=64 => parse_quote! { u64 },
        ..=128 => parse_quote! { u128 },
        _ => parse_quote! { compile_error!("Unsupported bit width") }
    }
}
