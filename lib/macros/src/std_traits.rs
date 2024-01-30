use syn::parse_macro_input;
use syn::DeriveInput;

use quote::quote;

pub fn impl_derive_from_u8(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let variants = if let syn::Data::Enum(data) = input.data {
        data.variants
    } else {
        panic!("FromU8 can only be derived for enums");
    };

    let variant_names: Vec<_> = variants.iter().map(|v| &v.ident).collect();
    let variant_values: Vec<_> = variants.iter().enumerate().map(|(i, _)| i as u8).collect();

    let expanded = quote! {
        impl From<u8> for #name {
            fn from(item: u8) -> Self {
                match item {
                    #(#variant_values => #name::#variant_names,)*
                    _ => panic!("Invalid value"),
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

pub fn impl_derive_from_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let variants = if let syn::Data::Enum(data) = input.data {
        data.variants
    } else {
        panic!("FromStr can only be derived for enums");
    };

    let variant_names: Vec<_> = variants
        .iter()
        .map(|v| &v.ident)
        .collect();

    let expanded = quote! {
        impl std::str::FromStr for #name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let item = s.to_uppercase();
                Ok(match item.as_str() {
                    #(stringify!(#variant_names) => Self::#variant_names,)*
                    _ => return Err("Invalid value".into()),
                })
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

pub fn impl_derive_display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = if let syn::Data::Enum(data) = input.data {
        data.variants
    } else {
        panic!("Display can only be derived for enums");
    };

    let variant_names: Vec<_> = variants.iter().map(|v| &v.ident).collect();

    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let string = match self {
                    #(Self::#variant_names => stringify!(#variant_names),)*
                    _ => panic!("Invalid value"),
                };

                write!(f, "{string}")
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
