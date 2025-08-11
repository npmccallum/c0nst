use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::ItemMod;

use super::{Adaptable, Target, Transform};

impl Adaptable for ItemMod {
    fn can_adapt(&self) -> bool {
        // Only embedded modules (with content) can be adapted
        // File modules don't have their content available in the token stream
        self.content.is_some()
    }
}

/// Implementation for Module Items
impl Transform for ItemMod {
    fn transform(&self, target: Target) -> TokenStream {
        let items: Vec<_> = match &self.content {
            Some((_, items)) => items.iter().map(|item| item.transform(target)).collect(),
            None => return self.to_token_stream(), // No content to transform
        };

        let attrs = &self.attrs;
        let vis = &self.vis;
        let ident = &self.ident;

        quote::quote! {
            #(#attrs)*
            #vis mod #ident {
                #(#items)*
            }
        }
    }
}
