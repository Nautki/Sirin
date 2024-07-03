#![allow(dead_code)]

use std::collections::HashMap;

use convert_case::{Case, Casing};
use parse::{BitRange, CsrDescription, Reg, RegValue};
use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput};

mod parse;

struct ValuePart {
    reg: syn::Ident,
    reg_range: (u8, u8),
    value_range: (u8, u8),
}

struct ValueSubfield {
    reg: syn::Ident,
    reg_range: (u8, u8),
}

struct Passthrough {
    reg: syn::Ident,
}

enum Value {
    Parts(Vec<ValuePart>),
    Subfield(ValueSubfield),
    Passthrough(Passthrough),
}

struct VariableMap {
    map: HashMap::<String, Value>,
}

impl VariableMap {
    pub fn new() -> VariableMap {
        VariableMap {
            map: HashMap::new()
        }
    }
}

#[proc_macro]
pub fn csr(tokens: TokenStream) -> TokenStream {
    let csr = parse_macro_input!(tokens as CsrDescription);

    let mut out = TokenStream2::new();

    for reg in csr.regs {
        let addr = reg.addr.clone();
        let name = syn::Ident::new(&("Reg".to_owned() + &reg.reg_name.to_string().to_case(Case::UpperCamel)), reg.reg_name.span());
        let readable = reg.access.readable();
        let writable = reg.access.writable();
        
        let reg_value = match &reg.value {
            RegValue::Subfields(_) => {
                create_value_type(&reg, &mut out).to_token_stream()
            },
            _ => {
                quote! { u8 }
            }
        };

        out.extend(quote! {
            pub struct #name;

            impl periphs::Register<u8, u8> for #name {
                const ADDR: u8 = #addr;
                const READABLE: bool = #readable;
                const WRITABLE: bool = #writable;
                type Value = #reg_value;
            }
        });
    }

    println!("{}", out.to_string());

    out.into()
}

fn create_value_type(reg: &Reg, out: &mut TokenStream2) -> syn::Ident {
    let value = syn::Ident::new(&(reg.reg_name.to_string().to_case(Case::UpperCamel) + "Value"), reg.reg_name.span());
    let mut stok = vec![];
    let RegValue::Subfields(subfields) = &reg.value else {
        panic!("Expected subfields on register {}", reg.reg_name)
    };

    for subfield in subfields {
        let name = &subfield.value_name;
        let (bitmask, offset, is_single) = match subfield.reg_range {
            Some(BitRange::Single(index)) => (1 << index as u8, index as u8, true),
            Some(BitRange::Range(start, end)) => ((0xFF << start) % (1 << end) as u8, start as u8, false),
            None => panic!("Expected something")
        };
        if is_single {
            stok.push(quote! {
                pub fn #name (&self) -> bool {
                    unsafe {
                        // Safety: guarunteed to be one bit
                        core::mem::transmute((self.0 & #bitmask) >> #offset)
                    }
                }
            });
        } else {
            stok.push(quote! {
                pub fn #name (&self) -> u8 {
                    (self.0 & #bitmask) >> #offset
                }
            });
        }
    }
    
    out.extend(quote! {
        pub struct #value (pub u8);

        impl #value {
            #(#stok)*
        }

        impl From<periphs::RegValue<u8>> for #value {
            fn from(value: periphs::RegValue<u8>) -> Self {
                #value(value.0)
            }
        }

        impl From<#value> for periphs::RegValue<u8> {
            fn from(value: #value) -> Self {
                periphs::RegValue(value.0)
            }
        }
    });

    value
}

#[proc_macro_derive(RegValue)]
pub fn derive_reg_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Enum(_) => {},
        _ => return quote! { compile_error!("Only enums can use `#[derive(RegValue)]`") }.into()
    }

    let rich = input.ident;

    let mut repr = None;

    for attr in input.attrs {
        let ident = attr.meta.path().get_ident();
        match ident {
            Some(ident) if ident.to_string() == "repr" => {},
            _ => continue
        }

        let Ok(list) = attr.meta.require_list() else {
            continue;
        };

        let Some(TokenTree::Group(group)) = list.tokens.clone().into_iter().next() else {
            continue;
        };

        let Some(TokenTree::Ident(_repr)) = group.stream().into_iter().next() else {
            continue;
        };

        repr = Some(_repr);
    };

    let Some(prim) = repr else {
        return quote! { compile_error!("`#[repr(...)]` is required") }.into()
    };
    
    return quote! {
        impl From<#rich> for RegValue<#prim> {
            fn from(value: #rich) -> Self {
                RegValue(value as #prim)
            }
        }
    }.into()
}

fn exact_prim(num_bits: usize) -> TokenStream {
    match num_bits {
        8 => quote! { u8 },
        16 => quote! { u16 },
        32 => quote! { u32 },
        64 => quote! { u64 },
        128 => quote! { u128 },
        _ => quote! { compile_error!("Unsupported bit width") }
    }.into()
}

fn nearest_prim(num_bits: usize) -> TokenStream {
    match num_bits {
        0..=8 => quote! { u8 },
        ..=16 => quote! { u16 },
        ..=32 => quote! { u32 },
        ..=64 => quote! { u64 },
        ..=128 => quote! { u128 },
        _ => quote! { compile_error!("Unsupported bit width") }
    }.into()
}

fn booltok(bool: bool) -> TokenStream2 {
    if bool {
        quote! { true }
    } else {
        quote! { false }
    }
}