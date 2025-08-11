use proc_macro2::TokenStream;
use syn::Signature;

use super::{Target, Transform};

/// Implementation for Signature
impl Transform for Signature {
    fn transform(&self, target: Target) -> TokenStream {
        // Transform the generics separately to extract type params and where clause
        let generics = &self.generics;

        // Generate the type parameters (without where clause)
        let type_params = if generics.params.is_empty() {
            TokenStream::new()
        } else {
            let params = generics.params.transform(target);
            quote::quote! { < #params > }
        };

        // Generate the where clause separately
        let where_clause = if let Some(where_clause) = &generics.where_clause {
            where_clause.transform(target)
        } else {
            TokenStream::new()
        };

        let constness = &self.constness;
        let asyncness = &self.asyncness;
        let unsafety = &self.unsafety;
        let abi = &self.abi;
        let fn_token = &self.fn_token;
        let ident = &self.ident;
        let inputs = &self.inputs;
        let variadic = &self.variadic;
        let output = &self.output;

        // Standard Rust syntax: return type before where clause
        quote::quote! {
            #constness #asyncness #unsafety #abi #fn_token #ident #type_params ( #inputs #variadic ) #output #where_clause
        }
    }
}
