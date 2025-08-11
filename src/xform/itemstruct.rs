use proc_macro2::TokenStream;
use syn::ItemStruct;

use super::{Target, Transform};

/// Implementation for ItemStruct
impl Transform for ItemStruct {
    fn transform(&self, target: Target) -> TokenStream {
        let attrs = self.attrs.transform(target);
        let vis = &self.vis;
        let struct_token = &self.struct_token;
        let ident = &self.ident;
        let generics = self.generics.transform(target);
        let fields = &self.fields;
        let semi_token = &self.semi_token;

        quote::quote! {
            #attrs
            #vis #struct_token #ident #generics #fields #semi_token
        }
    }
}
