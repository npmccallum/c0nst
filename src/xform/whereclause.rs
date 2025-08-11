use proc_macro2::TokenStream;
use syn::WhereClause;

use super::{Target, Transform};

/// Implementation for WhereClause
impl Transform for WhereClause {
    fn transform(&self, target: Target) -> TokenStream {
        // Transform the predicates inside the where clause
        let where_token = &self.where_token;
        let predicates = self.predicates.transform(target);

        quote::quote! {
            #where_token #predicates
        }
    }
}
