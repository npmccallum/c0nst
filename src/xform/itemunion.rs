use proc_macro2::TokenStream;
use syn::ItemUnion;

use super::{Target, Transform};

/// Implementation for ItemUnion
impl Transform for ItemUnion {
    fn transform(&self, target: Target) -> TokenStream {
        let attrs = self.attrs.transform(target);
        let vis = &self.vis;
        let union_token = &self.union_token;
        let ident = &self.ident;
        let generics = self.generics.transform(target);
        let fields = &self.fields;

        quote::quote! {
            #attrs
            #vis #union_token #ident #generics #fields
        }
    }
}
