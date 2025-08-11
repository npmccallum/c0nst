use proc_macro2::TokenStream;
use syn::TypeParam;

use super::{Target, Transform};

/// Implementation for TypeParam
impl Transform for TypeParam {
    fn transform(&self, target: Target) -> TokenStream {
        let mut result = TokenStream::new();
        let ident = &self.ident;
        result.extend(quote::quote! { #ident });

        if !self.bounds.is_empty() {
            result.extend(quote::quote! { : });
            result.extend(self.bounds.transform(target));
        }
        result
    }
}
