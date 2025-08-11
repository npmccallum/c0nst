use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{PredicateType, WherePredicate};

use super::{Target, Transform};

/// Implementation for WherePredicate
impl Transform for WherePredicate {
    fn transform(&self, target: Target) -> TokenStream {
        match self {
            WherePredicate::Type(PredicateType {
                bounded_ty, bounds, ..
            }) => {
                let mut result = TokenStream::new();
                result.extend(quote::quote! { #bounded_ty : });
                result.extend(bounds.transform(target));
                result
            }
            _ => self.to_token_stream(),
        }
    }
}
