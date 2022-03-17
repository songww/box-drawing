extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

#[proc_macro_derive(Parameters)]
pub fn derive_parameters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let option: syn::PathSegment = syn::parse_str("Option<i32>").unwrap();

    let mut args = vec![];
    let mut attrs = vec![];
    let mut optionals = vec![];
    let mut optional_types = vec![];

    match input.data {
        syn::Data::Struct(data) => {
            if let syn::Fields::Named(fields) = data.fields {
                for field in fields.named.into_iter() {
                    if let syn::Type::Path(tp) = field.ty.clone() {
                        let ps = tp.path.segments.first().unwrap();
                        if ps.ident == option.ident {
                            optionals.push(field.ident.clone());
                            if let syn::PathArguments::AngleBracketed(argtype) = &ps.arguments {
                                optional_types.push(argtype.args.clone());
                            }
                        } else {
                            attrs.push(field.ident.clone());
                            args.push(field);
                        }
                    } else {
                        return quote_spanned! {
                            field.span() => core::compile_error!("Unsupported field type.")
                        }
                        .into();
                    }
                }
            } else {
                return quote_spanned! {
                    data.fields.span() => core::compile_error!("Only support named fields.");
                }
                .into();
            }
        }
        _ => {
            return quote_spanned! {
                    input.span() => core::compile_error!("Only support named struct.");
            }
            .into()
        }
    };

    let name = input.ident;
    let generics = input.generics;
    quote! {
        impl #generics   #name #generics {
            pub fn new(#(#args,)*) -> #name #generics {
                #name {
                    #(#attrs, )*
                    #(#optionals: None, )*
                }
            }

            #(pub fn #optionals(mut self, #optionals: #optional_types) -> #name #generics { self.#optionals.replace( #optionals ); self })*
        }
    }.into()
}

#[proc_macro_derive(PositionalArgs)]
pub fn derive_positional_args(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let option: syn::PathSegment = syn::parse_str("Option<i32>").unwrap();

    let mut args = vec![];
    let mut attrs = vec![];
    let mut types = vec![];
    let mut optionals = vec![];
    let mut optional_types = vec![];

    let mut positionals = vec![];

    match input.data {
        syn::Data::Struct(data) => {
            if let syn::Fields::Named(fields) = data.fields {
                for field in fields.named.into_iter() {
                    if let syn::Type::Path(tp) = field.ty.clone() {
                        attrs.push(field.ident.clone());
                        types.push(field.ty.clone());
                        positionals.push(field.ident.clone().unwrap());
                        let ps = tp.path.segments.first().unwrap();
                        if ps.ident == option.ident {
                            optionals.push(field.ident.clone());
                            if let syn::PathArguments::AngleBracketed(argtype) = &ps.arguments {
                                optional_types.push(argtype.args.clone());
                            }
                        } else {
                            args.push(field);
                        }
                    } else {
                        return quote_spanned! {
                            field.span() => core::compile_error!("Unsupported field type.")
                        }
                        .into();
                    }
                }
            } else {
                return quote_spanned! {
                    data.fields.span() => core::compile_error!("Only support named fields.");
                }
                .into();
            }
        }
        _ => {
            return quote_spanned! {
                    input.span() => core::compile_error!("Only support named struct.");
            }
            .into()
        }
    };

    let args = positionals.iter().enumerate().map(|(pos, field)| {
        let field = field.to_string();
        quote! { #pos => #field }
    });

    let setter = (0..attrs.len()).map(|idx| format_ident!("set_{}", idx));

    let name = input.ident;
    let generics = input.generics;
    let builder = format_ident!("{}Builder", name);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn position(position: usize) -> &'static str {
                match position {
                    #(#args,)*
                    _ => unreachable!()
                }
            }
        }

        impl #impl_generics #builder #ty_generics #where_clause {
            #(pub fn #setter(&mut self, value: #types) {
                self.#attrs = value.into();
            })*
        }

    }
    .into()
}

// struct SetAttribute {
//     ty: syn::Type,
//     instance: syn::Ident,
//     pos: syn::Lit,
//     value: syn::Expr,
// }
//
// // vert_line < VertLine > . some => value
// impl syn::parse::Parse for SetAttribute {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<SetAttribute> {
//         let instance: syn::Ident = input.parse()?;
//         input.parse::<syn::Token![::]>()?;
//         input.parse::<syn::Token![<]>()?;
//         let ty: syn::Type = input.parse()?;
//         input.parse::<syn::Token![>]>()?;
//         input.parse::<syn::Token![.]>()?;
//         let pos: syn::Lit = input.parse()?;
//         input.parse::<syn::Token![=]>()?;
//         let value: syn::Expr = input.parse()?;
//         Ok(SetAttribute {
//             ty,
//             instance,
//             pos,
//             value,
//         })
//     }
// }

// #[proc_macro]
// pub fn set_attribute(input: TokenStream) -> TokenStream {
//     let SetAttribute {
//         ty,
//         instance,
//         pos,
//         value,
//     } = parse_macro_input!(input as SetAttribute);
//
//     let attr = ty::position()
//     let attr = format_ident!(
//         "{}",
//         attribute.into_token_stream().to_string(),
//         span = proc_macro2::Span::call_site()
//     );
//
//     quote! {
//         #instance.#attr(#value)
//     }
//     .into()
// }
