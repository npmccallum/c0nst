use proc_macro2::TokenStream;
use syn::ItemType;

use super::{Target, Transform};

/// Implementation for ItemType
impl Transform for ItemType {
    fn transform(&self, target: Target) -> TokenStream {
        let attrs = self.attrs.transform(target);
        let vis = &self.vis;
        let type_token = &self.type_token;
        let ident = &self.ident;
        let generics = self.generics.transform(target);
        let eq_token = &self.eq_token;
        let ty = &self.ty;
        let semi_token = &self.semi_token;

        quote::quote! {
            #attrs
            #vis #type_token #ident #generics #eq_token #ty #semi_token
        }
    }
}
