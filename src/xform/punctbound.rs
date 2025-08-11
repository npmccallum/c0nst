use proc_macro2::TokenStream;
use syn::punctuated::Punctuated;
use syn::{token::Plus, TypeParamBound};

use super::{Target, Transform};

/// Implementation for Punctuated<TypeParamBound, Plus>
impl Transform for Punctuated<TypeParamBound, Plus> {
    fn transform(&self, target: Target) -> TokenStream {
        // Join bounds with + separators using Transform::transform()
        let mut result = TokenStream::new();
        for (i, bound) in self.iter().enumerate() {
            if i > 0 {
                result.extend(quote::quote! { + });
            }
            result.extend(bound.transform(target));
        }
        result
    }
}
