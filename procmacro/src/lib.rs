use proc_macro::TokenStream;
use quote::quote;
use syn::{self, spanned::Spanned};

// #[proc_macro_attribute]
// pub fn mytest_proc_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
//     eprintln!("{:#?}", parse_macro_input!(attr as AttributeArgs));
//     let body_ast = parse_macro_input!(item as Item);
//     eprintln!("{:#?}", body_ast);
//     quote!(#body_ast).into()
// }

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    match do_expand(&st) {
        Ok(token_stream) => token_stream.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

type StructFields = syn::punctuated::Punctuated<syn::Field, syn::Token![,]>;

fn generate_builder_struct_fields_def(
    st: &syn::DeriveInput,
) -> syn::Result<proc_macro2::TokenStream> {
    let fields = get_fields_from_derive_input(st)?;
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let ret = quote! (
        #(
            #idents: std::option::Option<#types>,
        ),*
    );
    Ok(ret)
}
fn get_fields_from_derive_input(st: &syn::DeriveInput) -> syn::Result<&StructFields> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = &st.data
    {
        Ok(named)
    } else {
        Err(syn::Error::new_spanned(
            st,
            "This derive can only be used on structs with named fields",
        ))
    }
}

fn generate_builder_struct_factory_init_clause(
    st: &syn::DeriveInput,
) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let fields = get_fields_from_derive_input(st)?;
    let init_clause: Vec<_> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            quote! {
                #ident: std::option::Option::None,
            }
        })
        .collect();
    Ok(init_clause)
}

fn generate_setter_functions(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = get_fields_from_derive_input(st)?;
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let mut final_tokenstream = proc_macro2::TokenStream::new();
    for (ident, type_) in idents.iter().zip(types.iter()) {
        let tokenstream_piece = quote! {
            fn #ident(&mut self, #ident: #type_) -> mut Self {
                self.#ident = std::option::Option::Some(#ident);
                self
            }
        };
        final_tokenstream.extend(tokenstream_piece);
    }
    Ok(final_tokenstream)
}

fn do_expand(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    eprintln!("{:#?}", st);
    let struct_name_literal = st.ident.to_string();
    let builder_name_literal = format!("{}Builder", struct_name_literal);
    let builder_name_ident = syn::Ident::new(&builder_name_literal, st.span());
    let struct_ident = &st.ident;
    let struct_init_clause = generate_builder_struct_factory_init_clause(st)?;
    let builder_fields = generate_builder_struct_fields_def(st)?;
    let setter_functions = generate_setter_functions(st)?;
    let ret = quote! {
        pub struct #builder_name_ident {
            #builder_fields
        }
        impl #struct_ident {
            pub fn builder() -> #builder_name_ident {
                #builder_name_ident {
                    #(#struct_init_clause),*
                }
            }
        }

        imply #builder_name_ident {
            #setter_functions
        }
    };
    Ok(ret)
}
