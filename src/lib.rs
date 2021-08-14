use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Attribute, DeriveInput, Fields, Meta, NestedMeta, Type, Visibility};

#[proc_macro_derive(StructField, attributes(struct_field))]
pub fn derive_field(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let (vis, ty, generics) = (&ast.vis, &ast.ident, &ast.generics);
    let _field_enum_ident = Ident::new(&(ty.to_string() + "Field"), Span::call_site());

    let fields = filter_fields(match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => panic!("Field can only be derived for structs"),
    });

    let _field_enum_var = fields.iter().map(|(_vis, ident, ty)| {
        quote! {
            #ident(#ty)
        }
    });

    let _update_branch = fields.iter().map(|(_vis, ident, _ty)| {
        quote! {
            #_field_enum_ident::#ident(#ident) => self.#ident = #ident
        }
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let tokens = quote! {
        # [allow(non_camel_case_types)]
        #vis enum #_field_enum_ident #ty_generics
            #where_clause
        {
            #(#_field_enum_var),*
        }

        impl #impl_generics #ty #ty_generics
            #where_clause
        {
            pub fn update_field(&mut self, field: #_field_enum_ident #ty_generics){
                match field {
                    #(#_update_branch),*
                }
            }
        }
    };
    tokens.into()
}

fn filter_fields(fields: &Fields) -> Vec<(Visibility, Ident, Type)> {
    fields
        .iter()
        .filter_map(|field| {
            if field
                .attrs
                .iter()
                .find(|attr| has_skip_attr(attr, "struct_field"))
                .is_none()
                && field.ident.is_some()
            {
                let field_vis = field.vis.clone();
                let field_ident = field.ident.as_ref().unwrap().clone();
                let field_ty = field.ty.clone();
                Some((field_vis, field_ident, field_ty))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

const ATTR_META_SKIP: &'static str = "skip";

fn has_skip_attr(attr: &Attribute, path: &'static str) -> bool {
    if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
        if meta_list.path.is_ident(path) {
            for nested_item in meta_list.nested.iter() {
                if let NestedMeta::Meta(Meta::Path(path)) = nested_item {
                    if path.is_ident(ATTR_META_SKIP) {
                        return true;
                    }
                }
            }
        }
    }
    false
}
