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
    let _field_name = fields.iter().map(|(_vis, ident, _ty)| {
        let ident_name = ident.to_string();
        quote! {
            #_field_enum_ident::#ident(_) => #ident_name
        }
    });

    let _update_branch = fields.iter().map(|(_vis, ident, _ty)| {
        quote! {
            #_field_enum_ident::#ident(#ident) => self.#ident = #ident
        }
    });
    let _fetch_branch = fields.iter().map(|(_vis, ident, _ty)| {
        let ident_name = ident.to_string();
        quote! {
            #ident_name => Some(#_field_enum_ident::#ident(self.#ident.clone()))
        }
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let tokens = quote! {
        # [allow(non_camel_case_types)]
        # [derive(Clone)]
        #vis enum #_field_enum_ident #ty_generics
            #where_clause
        {
            #(#_field_enum_var),*
        }
        impl #impl_generics #_field_enum_ident #ty_generics
            #where_clause
        {
            pub fn name(&self) -> &'static str {
                match self {
                    #(#_field_name,)*
                }
            }
        }

        impl #impl_generics #ty #ty_generics
            #where_clause
        {
            pub fn update_field(&mut self, field: #_field_enum_ident #ty_generics){
                match field {
                    #(#_update_branch),*
                }
            }
            pub fn fetch_field(&mut self, field: &'static str) -> Option<#_field_enum_ident #ty_generics> {
                match field {
                    #(#_fetch_branch,)*
                    _x => None,
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
