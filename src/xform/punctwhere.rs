use proc_macro2::TokenStream;
use syn::punctuated::Punctuated;
use syn::{token::Comma, WherePredicate};

use super::{Target, Transform};

/// Implementation for Punctuated<WherePredicate, Comma>
impl Transform for Punctuated<WherePredicate, Comma> {
    fn transform(&self, target: Target) -> TokenStream {
        let mut result = TokenStream::new();
        for (i, predicate) in self.iter().enumerate() {
            if i > 0 {
                result.extend(quote::quote! { , });
            }
            result.extend(predicate.transform(target));
        }
        result
    }
}
