use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::TypeParamBound;

use super::{Target, Transform};

/// Implementation for TypeParamBound
impl Transform for TypeParamBound {
    fn transform(&self, target: Target) -> TokenStream {
        match self {
            TypeParamBound::Trait(trait_bound) => trait_bound.transform(target),
            _ => self.to_token_stream(),
        }
    }
}
