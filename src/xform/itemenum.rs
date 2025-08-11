use proc_macro2::TokenStream;
use syn::ItemEnum;

use super::{Target, Transform};

/// Implementation for ItemEnum
impl Transform for ItemEnum {
    fn transform(&self, target: Target) -> TokenStream {
        let attrs = self.attrs.transform(target);
        let vis = &self.vis;
        let enum_token = &self.enum_token;
        let ident = &self.ident;
        let generics = self.generics.transform(target);
        let variants = &self.variants;

        quote::quote! {
            #attrs
            #vis #enum_token #ident #generics {
                #variants
            }
        }
    }
}
