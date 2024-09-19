use convert_case::{Case, Casing};
use deluxe::ExtractAttributes;
use proc_macro::{self, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

use crate::synerror;

#[derive(ExtractAttributes, Clone)]
struct ColumnFormatAttributes {
    format: Option<String>,
    data_type: String,
    display_name: Option<String>,
    trimmable: Option<bool>,
    tag_options: Option<Ident>,
}

#[derive(ExtractAttributes)]
struct IdParameterAttribute(Ident);

pub fn derive_process_endpoint(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: type_name,
        data,
        ..
    } = parse_macro_input!(input);

    let Data::Struct(data_struct) = data else {
        synerror!(
            type_name,
            "cannot derive `ProcessEndpoint` for non-struct types"
        )
    };

    let fields: Vec<(String, Ident, ColumnFormatAttributes)> = {
        let Fields::Named(_) = &data_struct.fields else {
            synerror!(
                type_name,
                "cannot derive `ProcessEndpoint` for unit or tuple structs"
            )
        };

        let mut fields = Vec::new();
        for mut field in data_struct.fields.into_iter() {
            let field_ident = field.ident.clone().unwrap();
            let field_name = field_ident.clone().to_string();
            let Ok(format_attributes): Result<ColumnFormatAttributes, syn::Error> =
                deluxe::extract_attributes(&mut field)
            else {
                synerror!(
                    field_ident,
                    "cannot derive `ProcessEndpoint` without `#[col_format(...)]` attribute on each column"
                )
            };

            fields.push((field_name, field_ident, format_attributes));
        }

        fields
    };

    let columns: Vec<Ident> = fields.iter().map(|f| f.1.clone()).collect();
    let column_formats: Vec<TokenStream2> = fields
        .iter()
        .map(|f| {
            let column_ident = f.1.clone();
            let column_format = f.2.clone();

            match column_format.format {
                Some(format_variant_name) => match format_variant_name.as_str() {
                    "id" => quote!(#column_ident: crate::api::endpoints::ColumnFormat::Id),
                    "currency" => {
                        quote!(#column_ident: crate::api::endpoints::ColumnFormat::Currency)
                    }
                    "date" => quote!(#column_ident: crate::api::endpoints::ColumnFormat::Date),
                    "tag" => quote!(#column_ident: crate::api::endpoints::ColumnFormat::Tag),
                    _ => synerror!(
                        column_ident,
                        "invalid value for `format` in `#[col_format(...)]` attribute"
                    ),
                },
                None => quote!(#column_ident: crate::api::endpoints::ColumnFormat::None),
            }
        })
        .collect();
    let column_metadata: Vec<TokenStream2> = fields
        .iter()
        .map(|f| {
            let column_name = f.0.clone();
            let column_ident = f.1.clone();
            let column_format = f.2.clone();

            let data_type = match column_format.data_type.as_str() {
                "integer" => quote!(crate::api::endpoints::FrontendDataType::Integer),
                "decimal" => quote!(crate::api::endpoints::FrontendDataType::Decimal),
                "string" => quote!(crate::api::endpoints::FrontendDataType::String),
                "timestamp" => quote!(crate::api::endpoints::FrontendDataType::Timestamp),
                "tag" => quote!(crate::api::endpoints::FrontendDataType::Tag),
                _ => synerror!(
                    column_ident,
                    "invalid value for `data_type` in `#[col_format(...)]` attribute"
                ),
            };

            let display_name = match column_format.display_name {
                Some(name) => name,
                None => column_name.to_case(Case::Title),
            };
            
            let (display_type, trimmable_or_tag_options) = match (column_format.trimmable, column_format.tag_options) {
                (Some(trimmable), None) => (quote!(crate::api::endpoints::FrontendColumnDisplay::Text), quote!(trimmable: #trimmable)),
                (None, Some(tag_options)) => (quote!(crate::api::endpoints::FrontendColumnDisplay::Tag), quote!(options: #tag_options)),
                _ => synerror!(
                    column_ident,
                    "either `trimmable` or `tag_options` must be specified in `#[col_format(...)]` attribute"
                )
            };

            let display = quote! {
                #display_type {
                    name: #display_name,
                    #trimmable_or_tag_options,
                }
            };

            quote! {
                #column_ident: crate::api::endpoints::FrontendColumnMetadata {
                    data_type: #data_type,
                    display: #display,
                }
            }
            .into()
        })
        .collect();

    quote! {
        struct EndpointFormatting {
            #(
                #columns: crate::api::endpoints::ColumnFormat,
            )*
        }

        #[derive(Serialize)]
        struct EndpointMetadata {
            #(
                #columns: crate::api::endpoints::FrontendColumnMetadata,
            )*
        }

        impl EndpointFormatting {
            const fn new() -> Self {
                Self {
                    #(
                        #column_formats,
                    )*
                }
            }
        }

        impl EndpointMetadata {
            const fn new() -> Self {
                Self {
                    #(
                        #column_metadata,
                    )*
                }
            }
        }
    }.into()
}

pub fn derive_serve_entity_json(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: type_name,
        data,
        ..
    } = parse_macro_input!(input);

    let Data::Struct(_) = data else {
        synerror!(
            type_name,
            "cannot derive `ServeEntityJson` for non-struct types"
        )
    };

    quote! {
        impl crate::api::ServeEntityJson for #type_name {}
    }
    .into()
}

pub fn derive_serve_row_json(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input);
    let type_name = input.ident.clone();
    let Data::Struct(_) = input.data else {
        synerror!(
            type_name,
            "cannot derive `ServeRowJson` for non-struct types"
        )
    };

    let Ok(IdParameterAttribute(id_param_type)) = deluxe::extract_attributes(&mut input) else {
        synerror!(
            type_name,
            "cannot derive `ServeRowJson` without `#[id_param(...)]` attribute"
        )
    };

    quote! {
        impl crate::api::ServeRowJson<#id_param_type> for #type_name {}
    }
    .into()
}

pub fn derive_id_parameter(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: type_name,
        data,
        ..
    } = parse_macro_input!(input);

    let fields = match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => fields,
            _ => {
                synerror!(
                    type_name,
                    "cannot derive `IdParameter` for unit or tuple structs"
                )
            }
        },
        _ => {
            synerror!(
                type_name,
                "cannot derive `IdParameter` for non-struct types"
            )
        }
    };

    let first_field = fields.named.into_iter().next();
    if let Some(first_field) = first_field {
        let first_field_name = first_field.ident.unwrap();
        quote! {
            impl crate::api::IdParameter for #type_name {
                fn new(#first_field_name: usize) -> Self {
                    Self { #first_field_name }
                }

                fn id(&self) -> usize {
                    self.#first_field_name
                }
            }
        }
        .into()
    } else {
        synerror!(
            type_name,
            "cannot derive `IdParameter` for structs with no fields"
        )
    }
}
