use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::ImplItem;

use super::{Target, Transform};
use crate::attrs::HasAttribute;

/// Implementation for ImplItem
impl Transform for ImplItem {
    fn transform(&self, target: Target) -> TokenStream {
        match self {
            ImplItem::Fn(method) => {
                let c0nst = match (target, method.attrs.has_attribute("c0nst")) {
                    (Target::Nightly, true) => Some(quote::quote! { const }),
                    _ => None,
                };

                let attrs = method.attrs.transform(target);
                let vis = &method.vis;
                let sig = method.sig.transform(target);
                let block = &method.block;

                quote::quote! {
                    #attrs
                    #vis #c0nst #sig #block
                }
            }
            _ => self.to_token_stream(),
        }
    }
}
