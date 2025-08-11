use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Attribute;

use super::{Target, Transform};
use crate::attrs::IsAttribute;

/// Implementation for `Vec<Attribute>`
impl Transform for Vec<Attribute> {
    fn transform(&self, _target: Target) -> TokenStream {
        let mut result = TokenStream::new();
        for attr in self {
            // Skip #[c0nst] attributes for both stable and nightly (they get converted to const keywords)
            if !attr.is_attribute("c0nst") {
                result.extend(attr.to_token_stream());
            }
        }
        result
    }
}
