use proc_macro2::TokenStream;
use syn::TraitItem;

use super::{Target, Transform};

/// Implementation for `Vec<TraitItem>`
impl Transform for Vec<TraitItem> {
    fn transform(&self, target: Target) -> TokenStream {
        let mut result = TokenStream::new();
        for item in self {
            result.extend(item.transform(target));
        }
        result
    }
}
