extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, DeriveInput, Lit, LitStr, MetaNameValue, Token};

#[derive(Debug)]
enum Resolution {
    Tick,
    Second,
    Minute,
    Hour,
    Day,
}

impl Resolution {
    fn try_from_str(s: impl AsRef<str>) -> Option<Self> {
        match s.as_ref().to_ascii_lowercase().as_str() {
            "tick" => Some(Self::Tick),
            "second" => Some(Self::Second),
            "minute" => Some(Self::Minute),
            "hour" => Some(Self::Hour),
            "day" => Some(Self::Day),
            _ => None,
        }
    }
}

struct SymbolResolution {
    pub symbol: LitStr,
    pub resolution: Resolution,
}

fn parse_name_value(input: &syn::parse::ParseStream, ident: &str) -> syn::Result<MetaNameValue> {
    let nv = input.parse::<MetaNameValue>()?;
    if nv.path.is_ident(ident) {
        Ok(nv)
    } else {
        Err(syn::parse::Error::new(
            input.span(),
            format!("Invalid name-value pair. Expected ident: '{}'", ident),
        ))
    }
}

impl Parse for SymbolResolution {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let symbol = parse_name_value(&input, "symbol")?;
        input.parse::<Token![,]>()?;
        let resolution = parse_name_value(&input, "resolution")?;

        match (symbol.lit, resolution.lit) {
            (Lit::Str(symbol), Lit::Str(resolution)) => {
                if let Some(resolution) = Resolution::try_from_str(resolution.value()) {
                    Ok(Self {
                        symbol, resolution
                    })
                } else {
                    Err(syn::parse::Error::new(
                        input.span(), "invalid resolution type"
                    ))
                }
            }
            _ => Err(syn::parse::Error::new(
                input.span(),
                "symbol/resolution attribute expected string-literals.\nFor example: (symbol = \"BTCUSD\", resolution = \"tick\")",
            )),
        }
    }
}

#[proc_macro_derive(Algorithm, attributes(algo))]
pub fn derive_algorithm(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let data_funcs_name = ast
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("algo"))
        .map(|attr| match attr.parse_args::<SymbolResolution>() {
            Ok(sr) => sr,
            Err(e) => panic!("{:?}", e),
        })
        .map(|sr| quote::format_ident!("get_{}", sr.symbol.value().to_ascii_lowercase()));

    let name = &ast.ident;
    let data_name = quote::format_ident!("{}{}", name, "AlgorithmData");
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    TokenStream::from(quote! {
        pub struct #data_name;

        impl #data_name {
            #(fn #data_funcs_name (&self) {})*
        }

        impl #impl_generics crypto_crab::algo::AlgorithmData for #name #ty_generics #where_clause {
            type Data = #data_name;
        }
    })
}
