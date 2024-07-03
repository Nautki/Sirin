use syn::{braced, bracketed, parse::Parse, token::Bracket, Attribute, ExprRange, Ident, LitInt, Token};

pub struct CsrDescription {
    pub regs: Vec<Reg>,
}

impl Parse for CsrDescription {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let regs_ident = input.parse::<Ident>()?;
        if regs_ident.to_string() != "regs" {
            return Err(syn::Error::new(regs_ident.span(), "Expected 'addrs'"));
        }

        let regs;
        braced!(regs in input);
        let regs = regs.parse_terminated(Reg::parse, Token![,])?;

        Ok(CsrDescription {
            regs: regs.into_iter().collect(),
        })
    }
}

pub enum RegValue {
    Subfields(Vec<Subfield>),
    Single(Subfield),
    None
}

pub struct Reg {
    pub addr: LitInt,
    pub reg_name: Ident,
    pub value: RegValue,
    pub access: Access,
}

impl Parse for Reg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let addr = input.parse::<LitInt>()?;
        let name = input.parse::<Ident>()?;
        let access = input.parse::<Access>()?;

        if input.peek(Token![,]) {
            return Ok(Reg {
                addr,
                reg_name: name,
                value: RegValue::None,
                access
            });
        }

        if input.peek(Ident) {
            let ident = input.parse()?;
            let range = if input.peek(Bracket) {
                let _range;
                bracketed!(_range in input);
                Some(_range.parse::<BitRange>()?)
            } else {
                None
            };
            
            return Ok(Reg {
                addr,
                reg_name: name,
                access: access.clone(),
                value: RegValue::Single(Subfield {
                    attrs: vec![],
                    value_name: ident,
                    reg_range: None,
                    value_range: range,
                    access: Some(access)
                })
            })
        }

        let subfields;
        braced!(subfields in input);
        let subfields = subfields.parse_terminated(Subfield::parse, Token![,])?;

        Ok(Reg {
            addr,
            access,
            reg_name: name,
            value: RegValue::Subfields(subfields.into_iter().collect()),
        })
    }
}

pub struct Subfield {
    pub attrs: Vec<Attribute>, 
    pub value_name: Ident,
    pub reg_range: Option<BitRange>,
    pub value_range: Option<BitRange>,
    pub access: Option<Access>,
}

impl Parse for Subfield {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let reg_range = Some(input.parse::<BitRange>()?);

        // bad code but idc
        let access = if input.fork().parse::<Access>().is_ok() {
            Some(input.parse()?)
        } else {
            None
        };

        let value_name = input.parse::<Ident>()?;
        let value_range = if input.peek(Bracket) {
            let value_range;
            bracketed!(value_range in input);
            Some(value_range.parse()?)
        } else {
            None
        };

        Ok(Subfield {
            attrs,
            value_name,
            reg_range,
            value_range,
            access,
        })
    }
}

pub enum BitRange {
    Single(usize),
    Range(usize, usize),
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

#[derive(Clone, Copy)]
pub enum Access {
    Read,
    Write,
    ReadWrite,
}

impl Access {
    pub fn readable(&self) -> bool {
        match self {
            Access::Read | Access::ReadWrite => true,
            _ => false,
        }
    }
    
    pub fn writable(&self) -> bool {
        match self {
            Access::Write | Access::ReadWrite => true,
            _ => false,
        }
    }
}

impl Parse for Access {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        match ident.to_string().as_str() {
            "r" => Ok(Access::Read),
            "w" => Ok(Access::Write),
            "rw" => Ok(Access::ReadWrite),
            _ => Err(syn::Error::new(ident.span(), "Expected 'r', 'w', or 'rw'"))
        }
    }
}