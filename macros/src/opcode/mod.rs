use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{self, Parse},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
    Data, DataEnum, DeriveInput, Field, Fields, Lit, LitInt, Meta, MetaList, Variant,
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
        .map(|variant| create_function(variant))
        .collect();
    variants.iter().for_each(|variant| {
        variant.attrs.iter().for_each(|attribute| {
            let ident = attribute.path.segments.last().unwrap().ident.to_string();
            match ident.as_str() {
                "stack" => {
                    let meta = attribute.parse_meta().expect("");
                    let mut stack_info = StackInfo::default();
                    match meta {
                        Meta::List(list) => list.nested.iter().for_each(|nested| match nested {
                            syn::NestedMeta::Meta(meta) => match meta {
                                Meta::NameValue(name_value) => {
                                    let name =
                                        name_value.path.segments.last().unwrap().ident.to_string();
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
                    dbg!(stack_info);
                }
                _ => {}
            }
        });
    });

    let impl_block = quote! {
        impl OpCode {
            #(#functions)*
        }
    };
    proc_macro::TokenStream::from(quote! {
        #impl_block
    })
}

fn create_function(variant: &Variant) -> TokenStream {
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
    let body = create_function_body(&params, variant_name);
    let variant_fn_name = create_function_identifier(variant_name);
    let signature = quote! {
        pub fn #variant_fn_name(#(#params),*) -> Self {
            #body
        }
    };

    signature
}
fn create_function_identifier(variant_name: &Ident) -> Ident {
    let variant_name_str = variant_name.to_owned().to_string().to_lowercase();
    Ident::new(
        format!("new_{variant_name_str}").as_str(),
        variant_name.span(),
    )
}
fn create_function_body(params: &Vec<TokenStream>, variant_name: &Ident) -> TokenStream {
    if params.is_empty() {
        quote! { Self::#variant_name }
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

        quote! { Self::#variant_name(#(#params),*) }
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
