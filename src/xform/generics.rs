use proc_macro2::TokenStream;
use syn::Generics;

use super::{Target, Transform};

/// Implementation for Generics  
impl Transform for Generics {
    fn transform(&self, target: Target) -> TokenStream {
        // If no generics, return empty
        if self.params.is_empty() && self.where_clause.is_none() {
            return TokenStream::new();
        }

        // Manually reconstruct generics with transformed bounds
        let mut result = TokenStream::new();

        // Add type parameters with transformed bounds
        if !self.params.is_empty() {
            result.extend(quote::quote! { < });
            result.extend(self.params.transform(target));
            result.extend(quote::quote! { > });
        }

        // Add where clause with transformed bounds
        if let Some(where_clause) = &self.where_clause {
            result.extend(quote::quote! { where });
            result.extend(where_clause.predicates.transform(target));
        }

        result
    }
}
