use proc_macro2::TokenStream;
use syn::punctuated::Punctuated;
use syn::{token::Comma, GenericParam};

use super::{Target, Transform};

/// Implementation for Punctuated<GenericParam, Comma>
impl Transform for Punctuated<GenericParam, Comma> {
    fn transform(&self, target: Target) -> TokenStream {
        let mut result = TokenStream::new();
        for (i, param) in self.iter().enumerate() {
            if i > 0 {
                result.extend(quote::quote! { , });
            }
            result.extend(param.transform(target));
        }
        result
    }
}
