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
use syn::ItemEnum;

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
    let mut field_encodings = Vec::new();
    let mut field_decodings = Vec::new();
    let mut field_to_string = Vec::new();
    let mut field_from_str = Vec::new();

    for x in ast
        .variants
        .iter() {
            let name = &x.ident;
            let opcode = variant_opcode_value(x);

            if let syn::Fields::Unit = &x.fields {
                field_encodings.push(quote! {
                    Self::#name => #opcode as u16
                });

                field_decodings.push(quote! {
                    #opcode => Ok(Self::#name)
                });

                field_to_string.push(quote! {
                    Self::#name => write!(f, stringify!(#name))
                });

                field_from_str.push(quote! {
                    stringify!(#name) => {
                        Instruction::assert_length(&parts, 1).map_err(|x| Self::Err::Fail(x.to_string()))?;
                        Ok(Self::#name)
                    }
                });

                continue;
            }

            if let syn::Fields::Unnamed(fields) = &x.fields {
                let types: Vec<_> = fields
                    .unnamed
                    .iter()
                    .map(|f| get_type_name(&f.ty))
                    .collect();

                let types: Vec<&str> = types.iter().map(AsRef::as_ref).collect();

                match types[..] {
                    ["u8"] => {
                        field_encodings.push(quote! {
                            Self::#name(u) => #opcode as u16 | ((*u as u16) << 8)
                        });

                        field_decodings.push(quote! {
                            #opcode => Ok(Self::#name(((ins & 0xff00) >> 8) as u8))
                        });

                        field_to_string.push(quote! {
                            Self::#name(b) => write!(f, "{} {}", stringify!(#name), b)
                        });

                        field_from_str.push(quote! {
                            stringify!(#name) => {
                                Instruction::assert_length(&parts, 2).map_err(|x| Self::Err::Fail(x.to_string()))?;

                                Ok(Self::#name(Self::parse_numeric(parts[1]).map_err(|x| Self::Err::Fail(x.to_string()))?))
                            }
                        });
                    }

                    ["Register"] => {
                        field_encodings.push(quote! {
                            Self::#name(r) => #opcode as u16 | (((*r as u16)&0xf) << 8)
                        });

                        field_decodings.push(quote! {
                            #opcode => Ok(Self::#name(Register::from(((ins & 0xf00) >> 8) as u8)))
                        });

                        field_to_string.push(quote! {
                            Self::#name(r) => write!(f, "{} {}", stringify!(#name), r)
                        });

                        field_from_str.push(quote! {
                            stringify!(#name) => {
                                Instruction::assert_length(&parts, 2).map_err(|x| Self::Err::Fail(x.to_string()))?;
                                Ok(Self::#name(Register::from_str(parts[1]).map_err(|x| Self::Err::Fail(x.to_string()))?))
                            }
                        })
                    }

                    ["Register", "Register"] => {
                        field_encodings.push(quote! {
                            Self::#name(r1, r2) => #opcode as u16 | (((*r1 as u16) & 0xf) << 8)
                                | (((*r2 as u16) & 0xf) << 12)
                        });

                        field_decodings.push(quote! {
                            #opcode => {
                                let r1 = (ins & 0xf00) >> 8;
                                let r2 = (ins & 0xf000) >> 12;

                                Ok(Self::#name(Register::from(r1 as u8), Register::from(r2 as u8)))
                            }
                        });

                        field_to_string.push(quote! {
                            Self::#name(r1, r2) => write!(f, "{} {} {}", stringify!(#name), r1, r2)
                        });

                        field_from_str.push(quote! {
                            stringify!(#name) => {
                                Instruction::assert_length(&parts, 3).map_err(|x| Self::Err::Fail(x.to_string()))?;

                                Ok(Self::#name(
                                    Register::from_str(parts[1]).map_err(|x| Self::Err::Fail(x.to_string()))?,
                                    Register::from_str(parts[2]).map_err(|x| Self::Err::Fail(x.to_string()))?
                                ))
                            }
                        });
                    }

                    _ => panic!("Invalid types {types:?}")
                }
            } else {
                panic!("Unknown fields type for ident {name}");
            }
    };

    quote! {
        impl TryFrom<u16> for Instruction {
            type Error = String;

            fn try_from(ins: u16) -> Result<Self, Self::Error> {
                let op = (ins & 0xff) as u8;

                match op {
                    #(#field_decodings,)*
                    _ => panic!("Invalid types"),
                }
            }
        }

        impl std::str::FromStr for Instruction {
            type Err = InstructionParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let parts: Vec<_> = s.split_whitespace().filter(|p| !p.is_empty()).collect();

                if parts.is_empty() {
                    return Err(Self::Err::NoContent);
                }

                match parts[0] {
                    #(#field_from_str,)*
                    _ => Err(Self::Err::Fail(format!("Unknown opcode {}", parts[0]))),
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
                    #(#field_encodings,)*
                }
            }

            /// Used to parse a numeric based on whether it is binary,
            /// decimal, or hexadecimal.
            pub fn parse_numeric(s: &str) -> Result<u8, Box<dyn std::error::Error>> {
                let first = s.chars().next().unwrap();
                let (num, radix) = match first {
                    '$' => (&s[1..], 16),
                    '%' => (&s[1..], 2),
                    _ => (s, 10),
                };

                Ok(u8::from_str_radix(num, radix)?)
            }

            pub fn assert_length(parts: &[&str], n: usize) -> Result<(), Box<dyn std::error::Error>> {
                if !parts.len() == n {
                    return Err(format!("Expected {} got {}", n, parts.len()).into());
                }

                Ok(())
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
