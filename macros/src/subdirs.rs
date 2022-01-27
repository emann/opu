use proc_macro2::TokenStream;
use syn::{parse_quote, Attribute, Data, DeriveInput, Expr, LitStr, Meta, NestedMeta, Path, Token, DataStruct, Fields};
use proc_macro2::Ident;
use syn::spanned::Spanned;

pub fn derive_op1subdirs(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    match &input.data {
        Data::Struct(struct_data) => get_subdir_impl(input, struct_data),
        _ => Err(syn::Error::new(input.span(), "Only structs are supported")),
    }
}

pub fn get_struct_parser_impl(
    input: &DeriveInput,
    data: &DataStruct,
) -> syn::Result<TokenStream> {
    let ident = &input.ident;

    let fields = match &data.fields {
        Fields::Unnamed(_) | Fields::Unit => Err(syn::Error::new(
            data.fields.span(),
            "only named fields are currently supported",
        )),
        Fields::Named(named) => Ok(&named.named),
    }?
        .iter()
        .map(ParsedField::try_from)
        .collect::<syn::Result<Vec<_>>>()?;

    let error = impl_error_type(ident, &fields)?;

    Ok(quote! {
        #error
        #try_read_from_parser
        #try_from_le_bits
        #to_le_bits
    })
},
Ok,
        )
    })?
}