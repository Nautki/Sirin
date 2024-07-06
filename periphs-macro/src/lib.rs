use parse::{BitRange, Periph, Var};
use proc_macro::TokenStream;
use syn::{parse_macro_input, Ident};
use heck::*;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

mod parse;

macro_rules! ident {
    ($format:expr, $ident:expr) => {
        Ident::new(&(format!($format, $ident)), $ident.span())
    };
}

struct IdentGen {
    counter: usize
}

impl IdentGen {
    pub fn new() {
        IdentGen {
            counter: 0
        }
    }

    fn next(&mut self) -> Ident {
        let ident = Ident::new(&format!("_{}", self.counter), Span::call_site());
        self.counter += 1;
        ident
    }
}

#[proc_macro]
pub fn periph(input: TokenStream) -> TokenStream {
    let Periph {
        addr_ty,
        word_ty,
        name,
        regs,
        vars,
        ..
    } = parse_macro_input!(input as Periph);

    let addr_trait = ident!("As{}Addr", name);
    let read_trait = ident!("Read{}", name);
    let write_trait = ident!("Write{}", name);

    let mut out = TokenStream2::new();

    let mut read_out = TokenStream2::new();
    let mut write_out = TokenStream2::new();

    for reg in regs {
        let reg_struct = ident!("Reg{}", reg.reg);
        let Reg { addr, access, .. } = reg;
        let readable = access.readable;
        let writable = access.writable;

        out.extend(quote!{
            pub struct #reg_struct;
            impl #reg_struct {
                pub const ADDR: #addr_ty = #addr;
                pub const READABLE: bool = #readable;
                pub const WRITABLE: bool = #writable;
            }

            impl #addr_trait for #reg_struct {
                fn as_addr(&self) -> #addr_ty {
                    #addr
                }
            }
        });

        if readable {
            out.extend(quote! {
                impl ReadableAddr for #reg_struct {}
            });
        }

        if writable {
            out.extend(quote! {
                impl WritableAddr for #reg_struct {}
            });
        }
    }

    for (_, var) in vars {
        let IoFn { read, write } = gen_io_fn(var);

        let name = var.name;
        let set_var = ident!("set_{}", name);
        let var_ty = var.ty;

        if var.access.readable {
            read_out.extend(quote! {
                fn #var() -> impl core::future::Future<Output = Result<#var_ty, Self::Error>> {
                    #read
                }
            })
        }

        if var.access.writable {
            write_out.extend(quote! {
                fn #set_var(value: #word_ty) -> impl core::future::Future<Output = Result<(), Self::Error>> {
                    #write
                }
            })
        }
    }

    out.extend(quote! {
        pub trait #addr_trait {
            fn as_addr(&self) -> #addr_ty;
        }
        pub trait ReadableAddr: #addr_trait {}
        pub trait WritableAddr: #addr_trait {}

        impl #addr_trait for #addr_ty {
            fn as_addr(&self) -> #addr_ty {
                *self
            }
        }
        impl ReadableAddr for #addr_ty {}
        impl WritableAddr for #addr_ty {}

        pub trait #read_trait {
            type Error;

            fn read_contiguous<const WORDS: usize>(
                &mut self,
                addr: impl ReadableAddr
            ) -> impl core::future::Future<Output = Result<[#word_ty, WORDS], Self::Error>>;

            fn read(
                &mut self,
                addr: impl ReadableAddr
            ) -> impl core::future::Future<Output = Result<#word_ty, Self::Error>> {
                async {
                    match self.read_contiguous::<1>(addr).await {
                        Ok(res) => Ok(res[0]),
                        Err(err) => Err(err)
                    }
                }
            }
        }

        pub trait #write_trait {
            type Error;

            /// Write consecutive words to the peripheral. Most chips have it so
            /// that when writing more than just one word, the next word goes into the
            /// next address.
            fn write_contiguous<const WORDS: usize>(
                &mut self,
                addr: impl WritableAddr,
                values: &[#word_ty],
            ) -> impl core::future::Future<Output = Result<(), Self::Error>>;

            /// Write one word to the peripheral. Default calls `write_contiguous` with length `1`.
            fn write(
                &mut self,
                addr: impl WritableAddr,
                value: #word_ty
            ) -> impl core::future::Future<Output = Result<(), Self::Error>> {
                self.write_contiguous::<1>(addr, &[value])
            }
        }
    });

    todo!();
}

struct IoFn {
    read: TokenStream2,
    write: TokenStream2
}
fn gen_io_fn(var: Var) -> IoFn {
    if var.parts.len() == 0 {
        panic!("Var with no parts???");
    }

    let name = var.parts[0].var;

    if var.parts.len() == 1
        && var.parts[0].reg_range == BitRange::Entire
        && var.parts[0].var_range == BitRange::Entire
    {
        let part = var.parts[0];
        let reg = part.reg;
        return IoFn {
            read: quote! {
                self.read(#reg)
            },
            write: quote! {
                self.write(#reg)
            }
        }
    }

    let gen = IdentGen::new();

    struct Contig {
        start: Ident,
        start_addr: usize,
        /// Ident that corresponds to each word.
        words: Vec<Ident>
    }

    let mut read_var_out = TokenStream2::new();
    let mut write_var_out = TokenStream2::new();

    let mut contigs: Vec<Contig> = vec![];

    let mut acc = gen.next();

    for part in var.parts {
        let addr = part.reg_addr.base10_parse().unwrap();
        let ident = gen.next();
        
        let mut contig = match contigs.last() {
            Some(last) if last.start_addr + 1 == addr => {
                let mut contig = contigs.pop().unwrap();
                contig.words.push(ident);
                contig
            },
            _ => {
                Contig {
                    start: part.reg,
                    start_addr: addr,
                    words: vec![ident]
                }
            }
        };

        match part.reg_range {
            BitRange::Entire => {
                let var_start = part.var_range.start().unwrap();
                let var_end = part.var_range.end().unwrap();

                read_var_out.extend(quote! {
                    #acc += #ident << #var_start;
                });

                write_var_out.extend(quote! {
                    let #ident = ((#acc % (#var_end + 1)) >> #var_start);
                });
            },
            range => {
                let reg_start = part.reg_range.start().unwrap();
                let reg_end = part.reg_range.end().unwrap();
                let var_start = match part.var_range {
                    BitRange::Entire => 0,
                    range => range.start().unwrap()
                };

                read_var_out.extend(quote! {
                    #acc += ((#ident % (#reg_end + 1)) >> #reg_start) << #var_start;
                });

                write_var_out.extend(quote! {
                    let #ident = ((#acc % (#var_end + 1)) >> #var_start) << #reg_start;
                })
            }
        }
    }

    let mut read_begin = TokenStream2::new();
    let mut write_end = TokenStream2::new();

    for contig in contigs {
        if contig.words.len() == 1 {
            let ident = contig.words[0];
            let reg = contig.start;

            read_begin.extend(quote! {
                let #ident = self.read(#reg).await?;
            });

            write_end.extend(quote! {
                self.write(#reg, #ident).await?;
            });
        } else {
            let words = contig.words.iter();
            let len = contig.words.len();
            let start = contig.start;

            read_begin.extend(quote! {
                let [#(#words),*] = self.read_contiguous::<#len>(#start).await?;
            });

            write_end.extend(quote! {
                self.write_contiguous::<#len>(#start, &[#(#words),*]).await?;
            });
        }
    }

    return IoFn {
        read: quote! {
            async {
                #read_begin
                let mut #acc = 0;
                #read_var_out
                Ok(#acc)
            }
        },
        write: quote! {
            async {
                let mut #acc = value;
                #write_var_out
                #write_end
                Ok(())
            }
        }
    }
}