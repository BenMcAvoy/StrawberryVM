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
use quote::quote;
use syn::{ItemEnum, LitInt};

use std_traits::impl_derive_display;
use std_traits::impl_derive_from_str;
use std_traits::impl_derive_from_u8;

mod std_traits;

/// Automatically creates encode function and implements
/// from traits.
///
/// # Example usage
/// ```rust
/// pub enum Instruction {
///     #[opcode(0x0)]
///     Nop
///
///     #[opcode(0x1)]
///     Push(u8)
/// }
/// ```
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

fn variant_opcode_value(v: &syn::Variant) -> u8 {
    for attr in v.attrs.iter() {
        if attr.path().is_ident("opcode") {
            return attr
                .parse_args::<syn::LitInt>()
                .unwrap()
                .base10_parse()
                .unwrap();
        }
    }

    0
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

                    _ => panic!("Invalid types {types:?}"),
                }
            } else {
                panic!("Unknown fields type for ident {name}");
            }
        })
        .collect();

    let field_u16_decodings: Vec<_> = ast
        .variants
        .iter()
        .map(|x| {
            let value = variant_opcode_value(x);
            let name = &x.ident;

            if let syn::Fields::Unit = &x.fields {
                return quote! {
                    #value => Ok(Self::#name)
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
                        #value => Ok(Self::#name(((ins & 0xff00) >> 8) as u8))
                    },

                    ["Register"] => quote! {
                        #value => Ok(Self::#name(Register::from(((ins & 0xf00) >> 8) as u8)))
                    },

                    ["Register", "Register"] => quote! {
                        #value => {
                            let r1 = Register::from(((ins & 0xf00) >> 8) as u8);
                            let r2 = Register::from(((ins & 0xf000) >> 12) as u8);

                            Ok(Self::#name(r1, r2))
                        }
                    },

                    _ => panic!("Invalid types {types:?}"),
                }
            } else {
                panic!("Unknown fields type for ident {name}");
            }
        })
        .collect();

    let field_to_string: Vec<_> = ast
        .variants
        .iter()
        .map(|x| {
            let name = &x.ident;

            if let syn::Fields::Unit = &x.fields {
                return quote! {
                    Self::#name => write!(f, stringify!(#name))
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
                        Self::#name(byte) => write!(f, "{} {}", stringify!(#name), byte)
                    },

                    ["Register"] => quote! {
                        Self::#name(r) => write!(f, "{} {}", stringify!(#name), r)
                    },

                    ["Register", "Register"] => quote! {
                        Self::#name(r1, r2) => write!(f, "{} {} {}", stringify!(#name), r1, r2)
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
                    _ => Err(format!("Unknown opcode 0x{value:X}")),
                }
            }
        }

        impl TryFrom<u16> for Instruction {
            type Error = String;

            fn try_from(ins: u16) -> Result<Self, Self::Error> {
                let op = (ins & 0xff) as u8;

                match op {
                    #(#field_u16_decodings,)*
                    _ => panic!("Invalid types"),
                }
            }
        }

        impl std::fmt::Display for Instruction {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#field_to_string,)*
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
    impl_derive_from_u8(input)
}

/// Automatically implements the from display trait
/// for ease of use
#[proc_macro_derive(Display)]
pub fn derive_display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_derive_display(input)
}

/// Automatically implements the from u8 trait
/// for ease of use
#[proc_macro_derive(FromStr)]
pub fn derive_from_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_derive_from_str(input)
}
