use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::Parser;
use quote::quote;

fn main() {
    println!("Hello, world!");
}

#[proc_macro_attribute]
pub fn spi_device(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut device_struct = syn::parse_macro_input!(input as syn::ItemStruct);
    device_struct.generics.params.insert(0, syn::parse_quote!(S: ::sirin::spi::SpiConfig));
    match device_struct.fields {
        syn::Fields::Named(ref mut fields) => {
            fields.named.insert(0,
                syn::Field::parse_named.parse2(quote! { spi: ::sirin::spi::Spi<S> }).unwrap()
            );
            fields.named.insert(1,
                syn::Field::parse_named.parse2(quote! { cs_pin: ::embassy_stm32::gpio::AnyPin }).unwrap()
            );
        },
        _ => panic!("SpiDevice must be a struct with named fields"),
    }

    device_struct.into_token_stream().into()
}