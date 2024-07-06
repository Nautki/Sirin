#![allow(dead_code)]

use std::collections::HashMap;

use convert_case::{Case, Casing};
use parse::{Access, BitRange, CsrDescription, Reg, RegValue, Subfield};
use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput};

mod parse;

struct ValueParts {
    access: Access,
    parts: Vec<ValuePart>
}

struct ValuePart {
    reg: syn::Ident,
    reg_range: (u8, u8),
    value_range: (u8, u8),
}

struct ValueSubfield {
    access: Access,
    reg: syn::Ident,
    reg_range: (u8, u8)
}

struct Passthrough {
    access: Access,
    reg: syn::Ident,
}

enum Value {
    Parts(ValueParts),
    Subfield(ValueSubfield),
    Passthrough(Passthrough),
}

struct VariableMap {
    pub map: HashMap::<String, Value>,
}

impl VariableMap {
    pub fn new() -> VariableMap {
        VariableMap {
            map: HashMap::new()
        }
    }

    pub fn add_passthrough(&mut self, access: Access, name: String, reg: syn::Ident) {
        self.map.insert(name, Value::Passthrough(Passthrough { access, reg }));
    }

    pub fn add_subfield(&mut self, access: Access, name: String, reg: syn::Ident, reg_range: (u8, u8)) {
        self.map.insert(name, Value::Subfield(ValueSubfield { access, reg, reg_range }));
    }

    pub fn add_part(&mut self, access: Access, name: String, reg: syn::Ident, reg_range: (u8, u8), value_range: (u8, u8)) {
        if self.map.contains_key(&name) {
            match self.map.get_mut(&name).unwrap() {
                Value::Parts(parts) => {
                    parts.parts.push(ValuePart { reg, reg_range, value_range });
                    if parts.access != access {
                        panic!("Mixed r/w access");
                    }
                },
                _ => panic!("Mixed type of {}", name)
            }
        } else {
            self.map.insert(reg.to_string(), Value::Parts(ValueParts {
                access,
                parts: vec![ValuePart { reg, reg_range, value_range }]
            }));
        }
    }
}

#[proc_macro]
pub fn csr(tokens: TokenStream) -> TokenStream {
    let csr = parse_macro_input!(tokens as CsrDescription);
    let device_name = csr.name.clone();

    let addr_ty = quote! { u8 };
    let reg_int_ty = quote! { u8 };

    let mut out = TokenStream2::new();
    let mut vars = VariableMap::new();

    let reg_trait = syn::Ident::new(&(csr.name.to_string() + "Register"), csr.name.span());
    let write_trait = syn::Ident::new(&("Write".to_string() + &csr.name.to_string() + "Register"), csr.name.span());
    let read_trait = syn::Ident::new(&("Read".to_string() + &csr.name.to_string() + "Register"), csr.name.span());

    for reg in csr.regs {
        let addr = reg.addr.clone();
        let name = syn::Ident::new(&("Reg".to_owned() + &reg.reg_name.to_string().to_case(Case::UpperCamel)), reg.reg_name.span());
        let readable = reg.access.readable();
        let writable = reg.access.writable();
        
        let reg_self_value = match &reg.value {
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
                type Value = #reg_self_value;
            }

            impl #reg_trait for #name {}
        });

        match &reg.value {
            RegValue::Subfields(subfields) => {
                for subfield in subfields {
                    if subfield.value_range.is_some() {
                        vars.add_part(
                            subfield.access.unwrap_or(reg.access),
                            subfield.value_name.to_string(),
                            reg.reg_name.clone(),
                            subfield.reg_range.unwrap().into(),
                            subfield.value_range.unwrap().into()
                        );
                    } else {
                        vars.add_subfield(
                            subfield.access.unwrap_or(reg.access),
                            subfield.value_name.to_string(),
                            reg.reg_name.clone(),
                            subfield.reg_range.unwrap().into()
                        )
                    }
                }
            },
            RegValue::Single(subfield) => {
                if subfield.value_range.is_some() {
                    vars.add_part(
                        subfield.access.unwrap_or(reg.access),
                        subfield.value_name.to_string(),
                        reg.reg_name.clone(),
                        subfield.reg_range.unwrap().into(),
                        subfield.value_range.unwrap().into()
                    );
                } else {
                    vars.add_passthrough(
                        subfield.access.unwrap_or(reg.access),
                        subfield.value_name.to_string(),
                        reg.reg_name.clone(),
                    )
                }
            },
            RegValue::None => {},
        }
    }

    let mut read_fns = TokenStream2::new();
    let mut write_fns = TokenStream2::new();

    for (name, value) in vars.map {
        match value {
            Value::Parts(parts) => {
                let setter = "set_".to_string() + &name;
                
                let mut max = 0;
                for part in &parts.parts {
                    max = max.max(part.value_range.1);
                }

                let ty = nearest_prim(max as usize);

                if parts.access.writable() && parts.parts.iter().all(|part| part.value_range.1 - part.value_range.0 == 7 && part.value_range.0 % 8 == 0) {
                    let mut out = TokenStream2::new();

                    for part in &parts.parts {
                        let reg = &part.reg;
                        let shift = part.reg_range.0;
                        out.extend(quote! {
                            self.write_addr(#reg::ADDR, (value >> #shift) as u8).await?;
                        });
                    }

                    write_fns.extend(quote! {
                        fn #setter (&mut self, value: #ty) -> impl core::future::Future<Output = Result<(), Self::Error>> {
                            async {
                                #out
                            }
                        }
                    })
                }

                if parts.access.readable() {
                    let mut out = TokenStream2::new();

                    for part in &parts.parts {
                        let reg = &part.reg;
                        let shift = part.reg_range.0;
                        let size = part.reg_range.1 - part.reg_range.0;
                        out.extend(quote! {
                            result |= (self.read_addr(#reg::ADDR).await? << shift) % size;
                        });
                    }

                    read_fns.extend(quote! {
                        fn #name (&mut self) -> impl core::future::Future<Output = Result<(), Self::Error>> {
                            async {
                                let mut result: #ty = 0;
                                #out
                                result
                            }
                        }
                    })
                }
            },
            Value::Subfield(value) => {
                if value.access.writable() {

                }
            },
            Value::Passthrough(value) => {
                
            }
        }
    }

    out.extend(quote! {
        pub trait #reg_trait: periphs::Register<u8, u8> {}
        pub trait #write_trait {
            type Error;
        
            fn write_addr(&mut self, addr: #addr_ty, value: #reg_int_ty) -> impl core::future::Future<Output = Result<(), Self::Error>>;
        
            fn write_reg<R: #reg_trait + periphs::Writable>(&mut self, value: R::Value) -> impl core::future::Future<Output = Result<(), Self::Error>> {
                let value: periphs::RegValue<#reg_int_ty> = value.into();
                self.write_addr(R::ADDR, value.0)
            }

            #write_fns
        }
        
        pub trait #read_trait {
            type Error;
        
            fn read_addr(&mut self, addr: #addr_ty) -> impl core::future::Future<Output = Result<#reg_int_ty, Self::Error>>;
        
            fn read_reg<R: #reg_trait + periphs::Readable>(&mut self) -> impl core::future::Future<Output = Result<R::Value, Self::Error>> {
                async {
                    match self.read_addr(R::ADDR).await {
                        Ok(val) => Ok(periphs::RegValue(val).into()),
                        Err(err) => Err(err)
                    }
                }
            }

            #read_fns
        }
    });

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

fn exact_prim(num_bits: usize) -> TokenStream2 {
    match num_bits {
        8 => quote! { u8 },
        16 => quote! { u16 },
        32 => quote! { u32 },
        64 => quote! { u64 },
        128 => quote! { u128 },
        _ => quote! { compile_error!("Unsupported bit width") }
    }
}

fn nearest_prim(num_bits: usize) -> TokenStream2 {
    match num_bits {
        0..=8 => quote! { u8 },
        ..=16 => quote! { u16 },
        ..=32 => quote! { u32 },
        ..=64 => quote! { u64 },
        ..=128 => quote! { u128 },
        _ => quote! { compile_error!("Unsupported bit width") }
    }
}

fn booltok(bool: bool) -> TokenStream2 {
    if bool {
        quote! { true }
    } else {
        quote! { false }
    }
}