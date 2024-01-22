//! This crate is simply used for [StrawberryVM](https://crates.io/crates/strawberryvm).
//!
//! It implements these macros:
//! - `FromU8` for automatically implementing from u8 traits
//! - `VmInstruction` for creating encode functions, implementing traits, etc.
//!
//! It is not intended to be used outside of the
//! [StrawberryVM](https://crates.io/crates/strawberryvm) project but if you find a use for it
//! somehow, go ahead!

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;

use quote::quote;
use syn::{ItemEnum, LitInt};

/// Automatically creates encode function and implements
/// from traits.
#[proc_macro_derive(VmInstruction, attributes(opcode))]
pub fn derive_vm_instruction_impl(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_opcode_struct(&ast)
}

fn get_type_name(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(p) => p
            .path
            .segments
            .iter()
            .map(|x| x.ident.to_string())
            .collect(),
        _ => panic!("Bad type name!"),
    }
}

fn impl_opcode_struct(ast: &ItemEnum) -> TokenStream {
    let field_names: Vec<_> = ast.variants.iter().map(|x| &x.ident).collect();
    let field_values = ast.variants.iter().map(|x| {
        for attr in x.attrs.iter() {
            if attr.path().is_ident("opcode") {
                return attr.parse_args::<LitInt>().unwrap();
            }
        }

        syn::parse(quote! {0}.into()).unwrap()
    });

    let match_fields: Vec<_> = field_names
        .iter()
        .map(|f| f.to_string().to_lowercase())
        .collect();

    let field_u16_encodings: Vec<_> = ast
        .variants
        .iter()
        .map(|x| {
            let name = &x.ident;

            if let syn::Fields::Unit = &x.fields {
                return quote! {
                    Self::#name => OpCode::#name as u16
                };
            }

            if let syn::Fields::Unnamed(fields) = &x.fields {
                let types: Vec<_> = fields
                    .unnamed
                    .iter()
                    .map(|f| get_type_name(&f.ty))
                    .collect();

                let types: Vec<&str> = types.iter().map(AsRef::as_ref).collect();

                match types[..] {
                    ["u8"] => quote! {
                        Self::#name(u) => OpCode::#name as u16 | ((*u as u16) << 8)
                    },

                    ["Register"] => quote! {
                        Self::#name(r) => OpCode::#name as u16 | ((*r as u16) & 0xf) << 8
                    },

                    ["Register", "Register"] => quote! {
                        Self::#name(r1, r2) => OpCode::#name as u16 | ((*r1 as u16) & 0xf) << 8
                            | ((*r2 as u16) & 0xf) << 12
                    },

                    ["Register", "u8"] => quote! {
                        // reg as u16 | 8 BIT INT
                        // req as u16 | (((*reg as u8) >> 4) | (*amount))
                        // Self::#name(reg, amount) => OpCode::#name as u16 | ((((*reg as u8) >> 4) | (amount)) as u16 >> 8)
                        Self::#name(reg, amount) => ((((*reg as u8) << 4 | amount) as u16) << 8) | (OpCode::#name as u16)
                        // ((*reg as u8) & 0xf) << 8
                        // | ((*amount as u8) & 0xf) << 8
                    },

                    _ => panic!("Invalid types {types:?}"),
                }
            } else {
                panic!("Unknown fields type for ident {name}");
            }
        })
    .collect();

    quote! {
        #[repr(u8)]
        #[derive(Debug)]
        pub enum OpCode {
            #(#field_names = #field_values,)*
        }

        impl FromStr for OpCode {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let s = s.to_lowercase();
                let s = s.as_str();

                match s {
                    #(#match_fields => Ok(Self::#field_names),)*
                    _ => Err(format!("Unknown opcode {s}")),
                }
            }
        }

        impl TryFrom<u8> for OpCode {
            type Error = String;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    #(x if x == Self::#field_names as u8 => Ok(Self::#field_names),)*
                    // x if x == Self::AddReg as u8 => Ok(Self::AddReg),
                    _ => Err(format!("Unknown opcode 0x{value:X}")),
                }
            }
        }

        impl Instruction {
            pub fn encode_u16(&self) -> u16 {
                match self {
                    #(#field_u16_encodings,)*
                }
            }
        }
    }
    .into()
}

/// Automatically implements the from u8 trait
/// for ease of use
#[proc_macro_derive(FromU8)]
pub fn derive_from_u8(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
