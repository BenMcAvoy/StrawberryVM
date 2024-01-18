use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemEnum, LitInt};

#[proc_macro_derive(VmInstruction, attributes(opcode))]
pub fn derive_vm_instruction_impl(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_opcode_struct(&ast)
}

fn impl_opcode_struct(ast: &ItemEnum) -> TokenStream {
    let field_names: Vec<_> = ast.variants.iter().map(|x| &x.ident).collect();
    let field_values = ast.variants.iter().map(|x| {
        for attr in x.attrs.iter() {
            if attr.path().is_ident("opcode") {
                return attr.parse_args::<LitInt>().unwrap();
            }
        }

        syn::parse(quote!{0}.into()).unwrap()
    });

    quote! {
        #[repr(u8)]
        #[derive(Debug)]
        pub enum OpCode {
            #(#field_names = #field_values,)*
        }

        impl FromStr for OpCode {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(stringify!(#field_names) => Ok(Self::#field_names),)*
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
                    _ => Err(format!("Unknown opcode {value:X}")),
                }
            }
        }

    }.into()
}
