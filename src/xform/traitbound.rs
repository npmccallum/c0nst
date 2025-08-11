use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{GenericArgument, PathArguments, TraitBoundModifier, TypeParamBound};

use super::{Target, Transform};

/// Implementation for syn::TraitBound
impl Transform for syn::TraitBound {
    fn transform(&self, target: Target) -> TokenStream {
        let segment = match self.path.segments.len() {
            1 => &self.path.segments[0],
            _ => return self.to_token_stream(), // Not a c0nst<T> pattern
        };

        if segment.ident != "c0nst" {
            return self.to_token_stream();
        }

        let args = match &segment.arguments {
            PathArguments::AngleBracketed(args) => args,
            _ => return self.to_token_stream(), // Not a c0nst<T> pattern
        };

        let path = match args.args.first() {
            Some(GenericArgument::Type(syn::Type::Path(type_path))) => &type_path.path,
            _ => return self.to_token_stream(), // Not a c0nst<T> pattern
        };

        match target {
            Target::Stable => {
                // Transform c0nst<T> → T and ?c0nst<T> → T for stable
                TypeParamBound::Trait(syn::TraitBound {
                    paren_token: self.paren_token,
                    modifier: TraitBoundModifier::None, // Remove ? modifier too
                    lifetimes: self.lifetimes.clone(),
                    path: path.clone(),
                })
                .to_token_stream()
            }

            Target::Nightly => {
                match self.modifier {
                    // Transform ?c0nst<T> → [const] T
                    TraitBoundModifier::Maybe(_) => quote::quote! { [const] #path },

                    // Transform c0nst<T> → const T
                    TraitBoundModifier::None => quote::quote! { const #path },
                }
            }
        }
    }
}
