use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{self, Parse},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
    Attribute, Data, DataEnum, DeriveInput, Field, Fields, Lit, LitInt, Meta, MetaList, Variant,
};
#[derive(Debug, Default)]
struct StackInfo {
    push: u8,
    pop: u8,
}
pub fn expand_opcode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let syn::Data::Enum(data_enum) = input.data else {
        panic!()
    };
    let variants = data_enum.variants;
    let functions: Vec<TokenStream> = variants
        .iter()
        .map(|variant| {
            let function = {
                let stack_info = variant
                    .attrs
                    .iter()
                    .filter_map(|f| {
                        let attr = handle_attr(f);
                        attr
                    })
                    .take(1)
                    .next();
                create_function_with_info(variant, stack_info.as_ref())
            };
            let tokens = quote! {
                #function
            };
            return tokens;
        })
        .collect();

    let impl_block = quote! {
        impl Bytecode {
            #(#functions)*
        }
    };
    proc_macro::TokenStream::from(quote! {
        #impl_block
    })
}
fn handle_attr(attribute: &Attribute) -> Option<StackInfo> {
    let ident = attribute.path.segments.last().unwrap().ident.to_string();
    match ident.as_str() {
        "stack" => {
            let meta = attribute.parse_meta().expect("");
            let mut stack_info = StackInfo::default();
            match meta {
                Meta::List(list) => list.nested.iter().for_each(|nested| match nested {
                    syn::NestedMeta::Meta(meta) => match meta {
                        Meta::NameValue(name_value) => {
                            let name = name_value.path.segments.last().unwrap().ident.to_string();
                            match name.as_str() {
                                "push" => {
                                    let Lit::Int(int) = &name_value.lit else {
                                            panic!()
                                        };
                                    let number: Result<u8, syn::Error> = int.base10_parse();
                                    stack_info.push = number.unwrap();
                                }
                                "pop" => {
                                    let Lit::Int(int) = &name_value.lit else {
                                            panic!()
                                        };
                                    let number: Result<u8, syn::Error> = int.base10_parse();
                                    stack_info.pop = number.unwrap();
                                }
                                _ => {}
                            }
                        }
                        _ => panic!(),
                    },
                    syn::NestedMeta::Lit(_) => todo!(),
                }),
                _ => {}
            };

            return Some(stack_info);
        }
        _ => None,
    }
}
fn create_function_with_info(variant: &Variant, stack_info: Option<&StackInfo>) -> TokenStream {
    let variant_name = &variant.ident;
    // Check if the variant has fields
    let fields = match &variant.fields {
        Fields::Named(fields) => fields.named.to_owned(),
        Fields::Unnamed(fields) => fields.unnamed.to_owned(),
        Fields::Unit => Punctuated::default(),
    };
    // Generate the function parameters based on the fields
    let mut params: Vec<TokenStream> = Vec::new();
    for (i, field) in fields.iter().enumerate() {
        params.push(create_params(i, field))
    }

    // Generate the function body that constructs the variant
    let opcode = if params.is_empty() {
        quote! { OpCode::#variant_name }
    } else {
        let params: Vec<proc_macro2::TokenStream> = params
            .clone()
            .into_iter()
            .map(|tk| {
                let ident: proc_macro2::TokenTree =
                    tk.into_iter().next().unwrap().try_into().unwrap();
                proc_macro2::TokenStream::from(ident)
            })
            .collect();
        quote! {
        OpCode::#variant_name(#(#params),*)
        }
    };
    let variant_fn_name = create_function_identifier(variant_name);

    let stack_info_tokens = {
        if let Some(stack_info) = stack_info {
            let push = stack_info.push;
            let pop = stack_info.pop;
            quote! {
                self.stack_info.push(StackInfo {
                    push: #push,
                    pop: #pop,
               });
            }
        } else {
            quote! {}
        }
    };
    let signature = quote! {
        #[inline(always)]
        pub fn #variant_fn_name(&mut self, #(#params),*) {
            #stack_info_tokens
            self.function.chunk.emit_op(#opcode)
        }
    };
    signature
}

fn create_function_identifier(variant_name: &Ident) -> Ident {
    let variant_name_str = pascal_to_snake(variant_name.to_string().as_str());
    Ident::new(
        format!("write_{variant_name_str}_op").as_str(),
        variant_name.span(),
    )
}
fn pascal_to_snake(input: &str) -> String {
    let mut output = String::new();
    let mut capitalize_next = false;

    for c in input.chars() {
        if c.is_uppercase() {
            if !output.is_empty() {
                output.push('_');
            }
            output.extend(c.to_lowercase());
        } else {
            output.push(c);
        }
    }
    output
}

fn create_function_body(params: &Vec<TokenStream>, variant_name: &Ident) -> TokenStream {
    if params.is_empty() {
        quote! { OpCode::#variant_name }
    } else {
        let params: Vec<proc_macro2::TokenStream> = params
            .clone()
            .into_iter()
            .map(|tk| {
                let ident: proc_macro2::TokenTree =
                    tk.into_iter().next().unwrap().try_into().unwrap();
                proc_macro2::TokenStream::from(ident)
            })
            .collect();

        quote! { OpCode::#variant_name(#(#params),*) }
    }
}
fn create_params(i: usize, field: &Field) -> TokenStream {
    let field_name = &field.ident;
    if field_name.is_some() {
        // there is an identifier, which means its a named field?
        panic!("no named fields (yet)")
    }
    let field_name = Ident::new(format!("field{i}").as_str(), field.span());

    let ty = &field.ty;
    quote! { #field_name: #ty }
}
