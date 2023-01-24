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
    let mut impls: Vec<TokenStream> = Vec::new();
    let mut arms: Vec<TokenStream> = Vec::new();
    variants.iter().for_each(|variant| {
        let stack_info = variant
            .attrs
            .iter()
            .filter_map(|f| {
                let attr = handle_attr(f);
                attr
            })
            .take(1)
            .next();

        let implementation = if variant.attrs.iter().any(|f| {
            let ident = f.path.segments.last().unwrap().ident.to_string();
            ident == "no_impl"
        }) {
            quote!()
        } else {
            create_function_with_info(variant, stack_info.as_ref())
        };

        if let Some(stack_info) = stack_info {
            let matchargs = {
                let param: TokenStream = {
                    if variant.fields.len() > 0 {
                        let fields: Vec<TokenStream> = variant
                            .fields
                            .iter()
                            .map(|field| {
                                quote! {
                                    _,
                                }
                            })
                            .collect();
                        quote! {(#(#fields)*)}
                    } else {
                        quote!()
                    }
                };

                let push = stack_info.push;
                let pop = stack_info.pop;
                quote! {
                    #param => StackInfo {
                        push: #push,
                        pop: #pop,
                    },
                }
            };
            let variant_name = variant.ident.clone();

            arms.push(quote! {
                OpCode::#variant_name #matchargs
            })
        }
        impls.push(implementation)
    });
    let tokens = quote! { #(#arms)*};
    proc_macro::TokenStream::from(quote! {
        #(#impls)*
        pub fn get_stack_info(op: &OpCode) -> StackInfo {
            match op {
                #(#arms)*
                _ => unimplemented!()
            }
        }
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
    impl Bytecode {
        #[inline(always)]
        pub fn #variant_fn_name(&mut self, #(#params),*) {
            #stack_info_tokens
            self.function.chunk.emit_op(#opcode)
        }
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
