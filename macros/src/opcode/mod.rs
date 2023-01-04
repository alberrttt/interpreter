use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Comma, Data, DeriveInput,
    Field, Fields, Variant,
};
pub fn expand_opcode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let variants = get_variants(&input.data);
    let functions: Vec<TokenStream> = variants
        .iter()
        .map(|variant| create_function(variant))
        .collect();
    proc_macro::TokenStream::from(quote! {
         impl OpCode {
            #(#functions)*
        }
    })
}
fn get_variants(input_data: &Data) -> &Punctuated<Variant, Comma> {
    if let syn::Data::Enum(data) = input_data {
        &data.variants
    } else {
        panic!("Constructor can only be used on enums");
    }
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
