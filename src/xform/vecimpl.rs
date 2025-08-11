use proc_macro2::TokenStream;
use syn::ImplItem;

use super::{Target, Transform};

/// Implementation for `Vec<ImplItem>`
impl Transform for Vec<ImplItem> {
    fn transform(&self, target: Target) -> TokenStream {
        let mut result = TokenStream::new();
        for item in self {
            result.extend(item.transform(target));
        }
        result
    }
}
